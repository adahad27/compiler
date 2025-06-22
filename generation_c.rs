/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::fs;

use crate::{parse_c::{Node, NodeType, STManager}, token_c::is_identifier};

static mut CURRENT_LABEL_INDEX : u32 = 0;

pub fn generate_code(filename : &String, parse_tree : &mut Node, symbol_table : &mut STManager) {
    let mut program_string : String = "".to_string();

    generate_start_stub(&mut program_string);

    let mut register_manager = RegisterManager{register_list : Vec::new()};
    register_manager.initialize();

    generate_from_tree(&mut program_string, parse_tree, symbol_table, &mut register_manager);


    generate_exit_stub(&mut program_string);

    fs::write(filename, program_string).expect("Unable to write to file");

}

fn generate_start_stub(program_string : &mut String) {
    program_string.push_str("global _start\n_start:\n");
}

fn generate_from_tree(program_string : &mut String, parse_tree : &mut Node, symbol_table : &mut STManager, register_manager : &mut RegisterManager) {
    match parse_tree.node_type {
        NodeType::Function_Declaration => {
            program_string.push_str(format!("\tpush rbp\n\tmov rbp, rsp\n").as_str());
            generate_children(program_string, parse_tree, symbol_table, register_manager);
            
        }
        NodeType::Expression => {
            
            generate_children(program_string, parse_tree, symbol_table, register_manager);

            if !parse_tree.properties.contains_key("operator") {
                //Allocate a register
                let register_number : u32;
                let register_name : String;
                let operand : String;
                if 
                is_identifier(&parse_tree.properties["terminal"]) &&
                symbol_table.query(&parse_tree.properties["terminal"]).unwrap().register == -1 {

                    //If this identifier is not stored in a register
                    let addr : u32 = symbol_table.query(&parse_tree.properties["terminal"]).unwrap().addr;

                    register_number = register_manager.register_alloc(addr).unwrap();
                    register_name = register_manager.register_name(register_number);
                    symbol_table.modify_register(&parse_tree.properties["terminal"], register_number as i32);

                    operand = format!("[rbp-{}]", addr);
                    parse_tree.properties.insert("register".to_string(), register_name.to_string());
                    //Move value into allocated register
                    program_string.push_str(format!("\tmov {}, {}\n", register_name, operand).as_str());
                    
                }
                else if 
                is_identifier(&parse_tree.properties["terminal"]) &&
                symbol_table.query(&parse_tree.properties["terminal"]).unwrap().register != -1 {
                    let register_name : String = register_manager.register_name(symbol_table.query(&parse_tree.properties["terminal"]).unwrap().register as u32);
                    parse_tree.properties.insert("register".to_string(), register_name);
                }
                else if !is_identifier(&parse_tree.properties["terminal"]){
                    operand = parse_tree.properties["terminal"].clone();
                    register_number = register_manager.register_alloc(0).unwrap();
                    register_name = register_manager.register_name(register_number);
                    parse_tree.properties.insert("register".to_string(), register_name.to_string());
                    //Move value into allocated register
                    program_string.push_str(format!("\tmov {}, {}\n", register_name, operand).as_str());
                }

                
            }
            else {
                //This is an expression of the form 'identifier | constant + expression'

                //Allocate a register for the left node if one hasn't been allocated already
                let left_operand : String = parse_tree.children[0].properties["value"].clone();
                if is_identifier(&left_operand) {
                    //Left node is an identifier, so allocate a register iff one isn't allocated already.
                    if symbol_table.query(&left_operand).unwrap().register == -1 {
                        //No register has been allocated to it so far
                        let addr : u32 = symbol_table.query(&left_operand).unwrap().addr;
                        let register_number: u32 = register_manager.register_alloc(addr).unwrap();
                        let register_name : String = register_manager.register_name(register_number);
                        parse_tree.properties.insert("register".to_string(), register_name.clone());
                        program_string.push_str(format!("\tmov {}, {}\n", register_name, addr).as_str());
                    }
                    else {
                        let register_name : String = register_manager.register_name(symbol_table.query(&left_operand).unwrap().register as u32);
                        parse_tree.properties.insert("register".to_string(), register_name);
                    }
                }
                else {
                    //Left node is a constant, so we must allocate a register
                    let addr : u32 = 0;
                    let register_number: u32 = register_manager.register_alloc(addr).unwrap();
                    let register_name : String = register_manager.register_name(register_number);
                    parse_tree.properties.insert("register".to_string(), register_name.clone());
                    program_string.push_str(format!("\tmov {}, {}\n", register_name, left_operand).as_str());
                }
                
                //Perform the operation and store in the register for the left node.
                register_manager.register_free(register_manager.register_index(&parse_tree.children[2].properties["register"]) as u32);
                program_string.push_str(format!("\tadd {}, {}\n", &parse_tree.properties["register"], &parse_tree.children[2].properties["register"]).as_str());


                //Free the right node.
            }
            
        }
        NodeType::VarDecl => {            
            
            generate_children(program_string, parse_tree, symbol_table, register_manager);
            
            if parse_tree.children.len() == 3 || parse_tree.children.len() == 5 {
                program_string.push_str("\tpush 0\n");
            }
            let offset : i32 = symbol_table.query(&parse_tree.properties["identifier"]).unwrap().addr.clone() as i32;
            if parse_tree.children.len() == 4 {
                let register_name : String =  parse_tree.children[2].properties["register"].clone();
                symbol_table.modify_register(&parse_tree.properties["identifier"], register_manager.register_index(&register_name.clone()));
                program_string.push_str(format!("\tmov qword [rbp-{}], {}\n", offset, register_name).as_str());
            }
            else if parse_tree.children.len() == 5 {
                let register_name : String =  parse_tree.children[3].properties["register"].clone();
                symbol_table.modify_register(&parse_tree.properties["identifier"], register_manager.register_index(&register_name.clone()));
                program_string.push_str(format!("\tmov qword [rbp-{}], {}\n", offset, register_name).as_str());
            }
            
        }
        NodeType::ReturnStatement => {
            generate_children(program_string, parse_tree, symbol_table, register_manager);
            if parse_tree.children[1].properties.contains_key("terminal") {
                let operand : String = parse_tree.children[1].properties["terminal"].clone();
                let source : String;
                if is_identifier(&operand) {
                    //Then we have an identifier
                    let register_name : String = register_manager.register_name(symbol_table.query(&operand).unwrap().register as u32);
                    source = register_name;
                    
                }
                else {
                    //Then we have an actual number
                    source = operand;
                }
                program_string.push_str(format!("\tmov rdi, {}\n", source).as_str());
            }
            
            
        }        
        _ => {
            generate_children(program_string, parse_tree, symbol_table, register_manager);
        }

    }
}

fn generate_children(program_string : &mut String, parse_tree : &mut Node, symbol_table : &mut STManager, register_manager : &mut RegisterManager) {
    for mut node in &mut parse_tree.children {
        generate_from_tree(program_string, &mut node, symbol_table, register_manager);
    }
}

fn generate_exit_stub(program_string : &mut String) {
    program_string.push_str("\tmov rax, 60\n");
    program_string.push_str("\tsyscall\n");
}

struct Register {
    name : String,
    in_use : bool,
    addr : u32
}
struct RegisterManager {
    register_list : Vec<Register>
}

impl RegisterManager {

    fn initialize(&mut self) {
        // self.register_list = Vec::new();
        //Hardcoded register names specifically for x86 architecture
        self.register_list.push(Register{name : "rbx".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r10".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r11".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r12".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r13".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r14".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r15".to_string(), in_use : false, addr : 0});
    }

    fn register_alloc(&mut self, addr : u32) -> Option<u32> {
        let mut index : usize = 0;

        while index < self.register_list.len() {

            if !self.register_list[index].in_use {
                self.register_list[index].in_use = true;
                self.register_list[index].addr = addr;
                return Option::Some(index as u32);            
            }

            index += 1;
        }
        //Currently throw errors if there are no available registers.
        return Option::None;
    }

    fn register_free(&mut self, reg_index : u32) {
        self.register_list[reg_index as usize].in_use = false;
        self.register_list[reg_index as usize].addr = 0;
    }

    fn register_index(&self, register_name : &String) -> i32 {
        let name = register_name.as_str();
        match name {
            "rbx" => {
                0
            }
            "r10" => {
                1
            }
            "r11" => {
                2
            }
            "r12" => {
                3
            }
            "r13" => {
                4
            }
            "r14" => {
                5
            }
            "r15" => {
                6
            }
            _ => {
                -1
            }
        }
    }

    fn register_name(&self, reg_index : u32) -> String {
        return self.register_list[reg_index as usize].name.clone();
    }

}

fn label_create() -> u32 {
    unsafe {
        CURRENT_LABEL_INDEX += 1;
        return CURRENT_LABEL_INDEX;
    }
}

fn label_name(index : u32)->String {
    return format!(".L{}", index);
}