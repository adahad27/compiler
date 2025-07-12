use std::fs;
mod token_c;
mod parse_c;
mod code_gen_c;
mod expression_c;
mod statement_c;
mod symbol_table_c;
// mod ir_gen_c;

// use crate::ir_gen_c::generate_ir;
use crate::token_c::{lex_file, Token};
use crate::code_gen_c::generate_code;
use crate::parse_c::{parse, create_node, Node, NodeType};
use crate::symbol_table_c::{*};
use std::env;

fn main() {

    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter the file you want to compile");
        return;
    }
    else if args.len() > 3 {
        println!("The format of the input is \"./compiler <file for compilation> (optional)<name of output file>\"");
        return
    }

    let symbol_table = create_new_stnode(1);
    

    let token_list : Vec<Token> = lex(args[1].as_str());
    let mut current_node : Node = create_node(NodeType::Program_Start);

    if parse(&mut current_node, &token_list, &symbol_table) {
        let filename : String;
        if args.len() == 3 {
            filename = args[2].clone();
        }
        else {
            filename = "a.asm".to_string();
        }

        let _filename_ir : String = "main_generated.ll".to_string();
        // generate_ir(&filename_ir, &mut current_node, &symbol_table);
        generate_code(&filename, &mut current_node, &symbol_table);
    }
    else {
        println!("Sorry there was a parsing error!");
    }

    
}


/* 
Pass a source file to this function to receive a list of all tokens contained in
the source file.
*/
fn lex(src : &str) -> Vec<Token> {

    
    let contents : String = fs::read_to_string(src).expect("Should have been able to read from file");   

    return lex_file(contents);
}



