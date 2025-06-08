use std::fs;
mod token;

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

    //Create a dynamic size vector to append tokens from regex into.

    //Read the source file here
    let contents : String = fs::read_to_string(src).expect("Should have been able to read from file");

    

    // for reg in regex_list {
    //     if reg.is_match(&contents) {
    //         for token in reg.captures_iter(&contents) {
    //             token_list.push(token[0].to_string())
    //         }
    //     }
        
    // }

    

    return lex_file(contents);
}



