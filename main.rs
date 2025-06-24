use std::fs;
mod token_c;
mod parse_c;
mod generation_c;

use std::collections::HashMap;
use crate::token_c::{lex_file, Token};
use crate::generation_c::generate_code;
use crate::parse_c::{parse, create_node, Node, NodeType, STManager};


fn main() {


    let mut symbol_table : STManager = STManager{symbol_table : HashMap::new(), ordinal : 1};
    
    let token_list : Vec<Token> = lex("src_files/basic_lexing/test_1.c");

    // print_tokens(&token_list);

    let mut current_node : Node = create_node(NodeType::Program_Start);

    if parse(&mut current_node, &token_list, &mut  symbol_table) {
        
        let filename : String = "main_generated.asm".to_string();
        generate_code(&filename, &mut current_node, &mut symbol_table);
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



