use std::fs;
mod token;
mod parse;
use crate::token::{lex_file, Token};

fn main() {

    let token_list : Vec<Token> = lex("src_files/basic_lexing/test_1.c");
    
    for tok in token_list {
        println!("{}", tok.val);
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



