
use regex::Regex;
/* 
This file is responsible for the first part of Compiler construction which is
the lexer/tokenizer. It implements the TokenType Enum, and the Token struct
which will be used by the parser to implement parse trees.

*/
#[derive(PartialEq, Eq)] 
pub enum TokenType {
    Identifier,
    //Separators are (, ), {, }, ;
    Separator,
    //Keywords are if, else if, else, for, while, return
    Keyword,
    //Primitives are int, bool, char, float
    Primitive,
    //Operators are unary and binary
    // !, ==, !=, &&, ||, <, >, <=, >=, +, -, +=, -=
    Operator,
    Constant, //Only handling decimal numbers for now
    Special,
    Default
}

pub struct Token {
    pub token_type : TokenType,
    pub val : String
}

pub fn is_whitespace(input : &String) -> bool {
    let whitespace_regex : Regex = Regex::new(r"\s+").unwrap();
    return whitespace_regex.is_match(input);
}

pub fn is_operator(input : &String) -> bool {
    if
    input == "!" ||
    input == "!=" ||
    input == "==" ||
    input == "&&" ||
    input == "||" ||
    input == "<" ||
    input == ">" ||
    input == "<=" ||
    input == ">=" ||
    input == "+" ||
    input == "+=" ||
    input == "-" ||
    input == "-="||
    input == "=" ||
    input == "*" ||
    input == "/" {
        return true;
    }
    return false;
}

pub fn is_identifier(input : &String) -> bool {
    return input.chars().nth(0).unwrap().is_alphabetic();
}

pub fn is_separator(input : &String) -> bool {
    if
    input == "(" || 
    input == ")" || 
    input == "{" || 
    input == "}" || 
    input == ";" {
        return true;
    }
    return false;
}

fn is_keyword(input : &String) -> bool {
    if 
    input == "if" ||
    input == "else if" ||
    input == "else" ||
    input == "while" ||
    input == "for" ||
    input == "return" {
        return true;
    }
    return false;
}

pub fn is_primitive(input : &String) -> bool {
    if 
    input == "int" ||
    input == "float" ||
    input == "bool" ||
    input == "char" {
        return true;
    }
    return false;
}

pub fn construct_token(input : &String)-> Token{
    let mut t_type : TokenType = TokenType::Default;
    
    let identifier_regex : Regex = Regex::new(r"[[:alpha:]]+").unwrap();
    let integer_regex : Regex = Regex::new(r"[0-9]+").unwrap();

    //TODO: Replace this with a match statement

    if is_separator(&input){
        t_type = TokenType::Separator;
    }
    else if is_keyword(&input)
    {
        t_type = TokenType::Keyword;
    }
    else if is_primitive(&input) {
        t_type = TokenType::Primitive;
    }
    else if is_operator(&input){
        t_type = TokenType::Operator;
    }
    else if identifier_regex.is_match(&input) {
        t_type = TokenType::Identifier;
    }
    else if integer_regex.is_match(&input) {
        t_type = TokenType::Constant;
    }

    return Token{token_type : t_type, val : input.clone()};
}


pub fn lex_file(input : String) -> Vec<Token> {
    let mut token_vector : Vec<Token> = Vec::new();
    
    let mut current_token_val : String = "".to_string();


    /* 
    This for loop will read the source file character by character. It will 
    delimit tokens according to whitespace, separators, or operators.
    Separators and operators will also be tokenized.
    */
    for character in input.chars() {
        if is_whitespace(&character.to_string()) {
            if current_token_val != "".to_string() {
                token_vector.push(construct_token(&current_token_val));
            }
            current_token_val = "".to_string();
            continue;
        }
        /* 
        TODO: Current implementation will not be able to lex operators that have
        more than one character. This will need to be fixed.
        */
        if is_operator(&character.to_string()) || is_separator(&character.to_string()){
            if current_token_val != "".to_string() {
                token_vector.push(construct_token(&current_token_val));
            }
            token_vector.push(construct_token(&character.to_string()));
            current_token_val = "".to_string();
            continue;
        }
        current_token_val.push(character);
    }

    return token_vector;
}

pub fn print_tokens(tokens : &Vec<Token>) {
    for tok in tokens {
        println!("{}", tok.val);
    }
}