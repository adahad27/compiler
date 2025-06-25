/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::fs;

use crate::{parse_c::{create_node, Node, NodeType, STManager}, token_c::is_identifier};

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
        NodeType::Arith_Expr => {

            assert!(parse_tree.properties.contains_key("terminal"));
            let left_operand : String = parse_tree.properties["terminal"].clone();

            if !is_identifier(&left_operand) {
                //Left operand is a constant
                
                //Move it into a register
                let term_node : &mut Node = &mut parse_tree.children[0];
                generate_from_tree(program_string, term_node, symbol_table, register_manager);
                let reg_name : String = term_node.properties["register"].clone();


                let subexpr_node : &mut Node = &mut parse_tree.children[1];

                subexpr_node.properties.insert("prev_register".to_string(), reg_name.clone());
                generate_from_tree(program_string, subexpr_node, symbol_table, register_manager);
                let result_reg : String = 
                if parse_tree.children[1].properties.contains_key("register") {
                    parse_tree.children[1].properties["register"].clone()
                }
                else {
                    reg_name
                };
                parse_tree.properties.insert("register".to_string(), result_reg);
                //Free allocated register
                fs::write("main_generated.asm", program_string).expect("Unable to write to file");
            }

        }
        NodeType::Arith_Subexpr => {
            if parse_tree.properties.contains_key("operator") {
                let operator : String = parse_tree.properties["operator"].clone();

                /* Generate term node completely, so all multiplication, division, and parenthesis
                are given priority before addition, subtraction */
                let term_node : &mut Node = &mut parse_tree.children[1];
                generate_from_tree(program_string, term_node, symbol_table, register_manager);
                //The register that stores the result from evaluating term is stored in the result property
                let result_reg : String = term_node.properties["register"].clone();

                program_string.push_str(format!("\t{} {}, {}\n", to_operator(operator), parse_tree.properties["prev_register"].clone(), result_reg).as_str());

                //Store the results in the next subexpr node, so that it can pick up from where this node left off if needed.
                parse_tree.children[2].properties.insert("prev_register".to_string(), parse_tree.properties["prev_register"].clone());
                parse_tree.properties.insert("register".to_string(), parse_tree.properties["prev_register"].clone());

                let subexpr_node : &mut Node = &mut parse_tree.children[2];
                generate_from_tree(program_string, subexpr_node, symbol_table, register_manager);
            }
            else if parse_tree.properties.contains_key("terminal") {
                let term_node : &mut Node = &mut parse_tree.children[0];
                generate_from_tree(program_string, term_node, symbol_table, register_manager);

                let result_reg : String = term_node.properties["register"].clone();
                parse_tree.properties.insert("register".to_string(), result_reg.clone());
            }

            //TODO: Add terminal case for when subexprs end
        }
        NodeType::Arith_Term => {
            
            assert!(parse_tree.properties.contains_key("terminal"));
            let left_operand : String = parse_tree.properties["terminal"].clone();

            if !is_identifier(&left_operand) {
                //Left operand is a constant
                
                //Move it into a register
                let factor_node : &mut Node = &mut parse_tree.children[0];
                generate_from_tree(program_string, factor_node, symbol_table, register_manager);
                let reg_name : String = factor_node.properties["register"].clone();

                let mut subterm_node : &mut Node = &mut parse_tree.children[1];

                subterm_node.properties.insert("prev_register".to_string(), reg_name.clone());

                generate_from_tree(program_string, &mut subterm_node, symbol_table, register_manager);
                let result_reg : String = 
                if parse_tree.children[1].properties.contains_key("register") {
                    parse_tree.children[1].properties["register"].clone()
                }
                else {
                    reg_name
                };
                parse_tree.properties.insert("register".to_string(), result_reg);
                //Free allocated register
            }
        }
        NodeType::Arith_Subterm => {
            if parse_tree.properties.contains_key("operator") {
                let operator : String = parse_tree.properties["operator"].clone();

                /* Generate term node completely, so all multiplication, division, and parenthesis
                are given priority before addition, subtraction */
                let factor_node : &mut Node = &mut parse_tree.children[1];
                generate_from_tree(program_string, factor_node, symbol_table, register_manager);
                //The register that stores the result from evaluating term is stored in the result property
                let result_reg : String = factor_node.properties["register"].clone();

                /* This operator will always be multiplication or division, so proper assembly needs to be added to 
                facilitate these operations. */

                if operator == "/".to_string() {
                    program_string.push_str("\tmov rdx, 0\n");
                }
                program_string.push_str(format!("\tmov rax, {}\n", parse_tree.properties["prev_register"].clone()).as_str());
                program_string.push_str(format!("\t{} {}\n", to_operator(operator), result_reg).as_str());
                program_string.push_str(format!("\tmov {}, rax\n", parse_tree.properties["prev_register"].clone()).as_str());
                //Store the results in the next subexpr node, so that it can pick up from where this node left off if needed.
                parse_tree.children[2].properties.insert("prev_register".to_string(), parse_tree.properties["prev_register"].clone());
                parse_tree.properties.insert("register".to_string(), parse_tree.properties["prev_register"].clone());

                let subterm_node : &mut Node = &mut parse_tree.children[2];
                generate_from_tree(program_string, subterm_node, symbol_table, register_manager);
            }
            else if parse_tree.properties.contains_key("terminal"){
                let factor_node : &mut Node = &mut parse_tree.children[0];
                generate_from_tree(program_string, factor_node, symbol_table, register_manager);

                let result_reg : String = factor_node.properties["register"].clone();
                parse_tree.properties.insert("register".to_string(), result_reg.clone());
            }
            
            //TODO: Add terminal case for when factor_node end
        }
        NodeType::Arith_Factor => {
            let operand : String = parse_tree.properties["terminal"].clone();
            if is_identifier(&operand) {

            }
            else {
                //Left operand is a constant

                //Allocate register for it
                let reg_index : u32 = register_manager.register_alloc(0).unwrap();
                let reg_name : String = register_manager.register_name(reg_index);
                
                //Move it into a register
                program_string.push_str(format!("\tmov qword {}, {}\n", reg_name, operand).as_str());

                parse_tree.properties.insert("register".to_string(), reg_name);
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
            "rbx" => 0,
            "r10" => 1,
            "r11" => 2,
            "r12" => 3,
            "r13" => 4,
            "r14" => 5,
            "r15" => 6,
            _ => -1
        }
    }

    fn register_name(&self, reg_index : u32) -> String {
        return self.register_list[reg_index as usize].name.clone();
    }

}

fn to_operator(operator : String) -> String {
    let op_str = operator.as_str();
    match op_str {
        "+" => "add".to_string(),
        "-" => "sub".to_string(),
        "*" => "imul".to_string(),
        "/" => "idiv".to_string(),

        _ => "Error: Incorrect operator found".to_string()
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