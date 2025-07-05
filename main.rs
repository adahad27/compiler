use std::fs;
mod token_c;
mod parse_c;
mod generation_c;
mod expression_c;
mod statement_c;
mod symbol_table_c;

use std::collections::HashMap;
use crate::token_c::{lex_file, Token};
use crate::generation_c::generate_code;
use crate::parse_c::{parse, create_node, Node, NodeType};
use crate::symbol_table_c::{*};
use std::rc::Rc;


fn main() {


    let symbol_table = create_new_STNode(1);
    
    let path : String = "src_files/feature_testing/".to_string();
    let file : String = "test_bool.c".to_string();

    let token_list : Vec<Token> = lex("src_files/feature_testing/test_loops.c");

    // print_tokens(&token_list);

    let mut current_node : Node = create_node(NodeType::Program_Start);

    if parse(&mut current_node, &token_list, &symbol_table) {
        
        let filename : String = "main_generated.asm".to_string();
        // let filename : String = "src_files/feature_testing/test_loops.asm".to_string();
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



