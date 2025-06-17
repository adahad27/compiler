/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::fs;

use crate::parse::{Node, NodeType, STManager};

pub fn generate_code(filename : &String, mut parse_tree : &Node, symbol_table : &mut STManager) {
    let mut program_string : String = "".to_string();

    generate_start_stub(&mut program_string);

    generate_from_tree(&mut program_string, parse_tree, symbol_table);


    generate_exit_stub(&mut program_string);

    fs::write(filename, program_string).expect("Unable to write to file");

}

fn generate_start_stub(program_string : &mut String) {
    program_string.push_str("global _start\n_start:\n");
}

fn generate_from_tree(program_string : &mut String, mut parse_tree : &Node, symbol_table : &mut STManager) {
    match parse_tree.node_type {
        NodeType::Program_Start => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::Function_Declaration => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::Primitive => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::Identifier => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::Separator => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::Body => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::VarDecl => {
            /* 
            1.) Subtract stack pointer by size
            2.) Push variable onto stack
            3.) Update symbol table
            */
            if let Option::Some(query_value) = symbol_table.query(&parse_tree.children[1].value) {
                program_string.push_str(format!("\tsub rsp, {}\n\tpush {}\n", &query_value.size, &parse_tree.value).as_str());
            }
            
            


            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::Statement => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::Operator => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::ReturnStatement => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
            if parse_tree.children[1].value.chars().nth(0).unwrap().is_alphabetic() {
                //Then we have an identifier
                //We must perform a lookup to get the value
                let mut offset : i32 = symbol_table.query(&parse_tree.children[1].value).unwrap().addr.clone() as i32;
                let size : i32 = symbol_table.query(&parse_tree.children[1].value).unwrap().size.clone() as i32;
                offset += size;

                program_string.push_str(format!("\tmov rdi, [rbp-{}]\n", offset).as_str());
            }
            else {
                //Then we have an actual number
                program_string.push_str(format!("\tmov rdi, {}\n", parse_tree.children[1].value).as_str());
            }
            
        }
        NodeType::Keyword => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }
        NodeType::Constant => {
            for node in &parse_tree.children {
                generate_from_tree(program_string, node, symbol_table);
            }
        }

    }
}



fn generate_exit_stub(program_string : &mut String) {
    program_string.push_str("\tmov rax, 60\n");
    program_string.push_str("\tsyscall\n");
}