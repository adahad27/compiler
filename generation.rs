/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::fs;

use crate::parse::Node;
use crate::parse::NodeType;

pub fn generate_code(filename : &String, mut parse_tree : &Node) {
    let mut program_string : String = "".to_string();

    generate_start_stub(&mut program_string);

    generate_from_tree(&mut program_string, parse_tree);


    generate_exit_stub(&mut program_string);

    fs::write(filename, program_string).expect("Unable to write to file");

}

fn generate_start_stub(program_string : &mut String) {
    program_string.push_str("global _start\n_start:\n");
}

fn generate_from_tree(program_string : &mut String, mut parse_tree : &Node) {
    match parse_tree.node_type {
        NodeType::Program_Start => {
            println!("Program_Start Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Function_Declaration => {
            println!("Function_Declaration Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Primitive => {
            println!("Primitive Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Identifier => {
            println!("Identifier Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Open_Paren => {
            println!("Open_Paren Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Close_Paren => {
            println!("Close_Paren Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Open_Curly => {
            println!("Open_Curly Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Body => {
            println!("Body Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Close_Curly => {
            println!("Close_Curly Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::ReturnStatement => {
            // generate_from_tree(program_string, &mut parse_tree.children[1]);
            println!("ReturnStatement Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
            program_string.push_str(format!("\tmov rdi, {}\n", parse_tree.children[1].value).as_str());
        }
        NodeType::Keyword => {
            println!("Keyword Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Constant => {
            println!("Constant Node encountered with value {}", parse_tree.value);
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }
        NodeType::Semicolon => {
            println!("Semicolon Node encountered");
            for node in &parse_tree.children {
                generate_from_tree(program_string, node);
            }
        }

    }
}



fn generate_exit_stub(program_string : &mut String) {
    program_string.push_str("\tmov rax, 60\n");
    program_string.push_str("\tsyscall\n");
}