use std::fs;
use regex::Regex;

fn main() {

    let token_list : Vec<String> = lex("src_files/basic_lexing/test_no_ret.c");
    
    for token in token_list {
        println!("{}", token);
    }
}


/* 
Pass a source file to this function to receive a list of all tokens contained in
the source file.
*/
fn lex(src : &str) -> Vec<String> {

    //Create a dynamic size vector to append tokens from regex into.
    let mut token_list : Vec<String> = Vec::new();

    //Read the source file here
    let contents : String = fs::read_to_string(src).expect("Should have been able to read from file");

    let mut regex_list : Vec<regex::Regex> = Vec::new();

    //Regex values of all (so far) allowed tokens
    let allowed_tokens : Vec<&str> = 
    vec![
    "[[:alpha:]]+",
    ";", 
    r"\(",
    r"\)",
    "[0-9]+",
    "<=",
    ">=",
    "==",
    "=",
    r"\+=",
    "-=",
    r"\+\+",
    "--",
    r"\+",
    "-"
    ];

    for token in allowed_tokens {
        regex_list.push(Regex::new(&token).unwrap());
    }

    //Regex matching across all constructed regexes.
    for reg in regex_list {
        if reg.is_match(&contents) {
            for token in reg.captures_iter(&contents) {
                token_list.push(token[0].to_string())
            }
        }
        
    }

    

    return token_list;
}