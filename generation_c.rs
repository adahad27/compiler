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

            if parse_tree.children.len() > 1 {
                //This is an expression with an operator
            }
            else {
                /* This is an expression without an operator, so it must be an identifier or a constant.
                If identifier, we query for the address and move into first available register.
                If constant, we take value of the constant and move into first available register. */

                let available_register : u32 = register_manager.register_alloc().unwrap();
                parse_tree.register = available_register as i32;

                

                // program_string.push_str(
                //     format!("\tmov {}, {}\n", 
                //     register_manager.register_name(available_register), 
                //     if is_identifier(&parse_tree.value) {"".to_string()} else {parse_tree.value.clone()}).as_str()
                // );                

            }
            
        }
        NodeType::VarDecl => {
            /* 
            1.) Subtract stack pointer by size
            2.) Push variable onto stack
            3.) Update symbol table
            */
            
            
            generate_children(program_string, parse_tree, symbol_table, register_manager);
            
            if parse_tree.children.len() == 3 || parse_tree.children.len() == 5 {
                //This is a declaration like "int x;"
                program_string.push_str("\tpush 0\n");
            }
            if parse_tree.children.len() == 4 || parse_tree.children.len() == 5 {
                //This is an assignment like y = 4;
                
                let offset : i32 = symbol_table.query(&parse_tree.properties["identifier"]).unwrap().addr.clone() as i32;

                let operand : String = parse_tree.properties["value"].clone();

                if is_identifier(&operand) {
                    program_string.push_str(format!("\tmov rax, [rbp-{}]\n", symbol_table.query(&operand).unwrap().addr).as_str());
                    program_string.push_str(format!("\tmov [rbp-{}], rax\n", offset).as_str());
                }
                else {
                    program_string.push_str(format!("\tmov dword [rbp-{}], {}\n", offset, operand).as_str());
                }

                
            }
            


            // if let Option::Some(query_value) = symbol_table.query(&parse_tree.properties["identifier"]) {
            //     program_string.push_str(format!("\tpush {}\n", &parse_tree.properties["value"]).as_str());
            // }
        }
        NodeType::ReturnStatement => {
            generate_children(program_string, parse_tree, symbol_table, register_manager);
            if parse_tree.children[1].properties.contains_key("terminal") {
                if is_identifier(&parse_tree.children[1].properties["terminal"]) {
                    //Then we have an identifier
                    //We must perform a lookup to get the value
                    let offset : i32 = symbol_table.query(&parse_tree.children[1].properties["terminal"]).unwrap().addr.clone() as i32;
                    program_string.push_str(format!("\tmov rdi, [rbp-{}]\n", offset).as_str());
                }
                else {
                    //Then we have an actual number
                    program_string.push_str(format!("\tmov rdi, {}\n", parse_tree.children[1].properties["terminal"]).as_str());
                }
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
    in_use: bool
}
struct RegisterManager {
    register_list : Vec<Register>
}

impl RegisterManager {

    fn initialize(&mut self) {
        // self.register_list = Vec::new();
        //Hardcoded register names specifically for x86 architecture
        self.register_list.push(Register{name : "rbx".to_string(), in_use : false});
        self.register_list.push(Register{name : "r10".to_string(), in_use : false});
        self.register_list.push(Register{name : "r11".to_string(), in_use : false});
        self.register_list.push(Register{name : "r12".to_string(), in_use : false});
        self.register_list.push(Register{name : "r13".to_string(), in_use : false});
        self.register_list.push(Register{name : "r14".to_string(), in_use : false});
        self.register_list.push(Register{name : "r15".to_string(), in_use : false});
    }

    fn register_alloc(&mut self) -> Option<u32> {
        let mut index : usize = 0;

        while index < self.register_list.len() {

            if !self.register_list[index].in_use {
                self.register_list[index].in_use = true;
                return Option::Some(index as u32);            
            }

            index += 1;
        }
        //Currently throw errors if there are no available registers.
        return Option::None;
    }

    fn register_free(&mut self, reg_index : u32) {
        self.register_list[reg_index as usize].in_use = false;
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