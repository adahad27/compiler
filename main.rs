use std::fs;
mod token;
mod parse;
mod generation;
use crate::token::{lex_file, Token};
use crate::generation::generate_code;
use crate::parse::{parse, create_node, Node, NodeType};

fn main() {

    let token_list : Vec<Token> = lex("src_files/basic_lexing/test_1.c");
    
    // for tok in token_list {
    //     println!("{}", tok.val);
    // }

    let mut current_node : Node = create_node(NodeType::Program_Start);

    if parse(&mut current_node, &token_list) {
        
        let filename : String = "main_generated.asm".to_string();
        generate_code(&filename, &current_node);
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



