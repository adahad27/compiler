/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::fs;

use crate::parse::Node;
use crate::parse::NodeType;

pub fn generate_code(filename : &String, parse_tree : Node) {
    let mut program_string : String = "".to_string();

    generate_start_stub(&mut program_string);




    generate_exit_stub(&mut program_string, 0);

    fs::write(filename, program_string).expect("Unable to write to file");

}

fn generate_start_stub(program_string : &mut String) {
    program_string.push_str("global _start\n_start:\n");
}

fn generate_from_tree(program_string : &mut String, parse_tree : &Node) {
    match parse_tree.node_type {
        NodeType::Program_Start => {

        }
        NodeType::Function_Declaration => {

        }
        NodeType::Primitive => {

        }
        NodeType::Identifier => {

        }
        NodeType::Open_Paren => {

        }
        NodeType::Close_Paren => {

        }
        NodeType::Open_Curly => {

        }
        NodeType::Body => {

        }
        NodeType::Close_Curly => {

        }
        NodeType::Return => {

        }
        NodeType::Constant => {

        }
        NodeType::Semicolon => {

        }

    }
}



fn generate_exit_stub(program_string : &mut String, exit_status : i32) {
    program_string.push_str("\tmov rax, 60\n");
    program_string.push_str(format!("\tmov rdi, {}\n", exit_status).as_str());
    program_string.push_str("\tsyscall\n");
}