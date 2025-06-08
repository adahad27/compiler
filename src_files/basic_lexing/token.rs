use regex::Regex;

enum Token_Type {
    identifier,
    //Separators are (, ), {, }, ;
    separator,
    //Keywords are if, else if, else, for, while, return, int, float, bool, char
    keyword,
    //Operators are unary and binary
    //!, ==, !=, &&, ||, <, >, <=, >=, +, -, +=, -=
    operator,
    constant, //Only handling decimal numbers for now
    special
}

struct Token {
    token_type : Token_Type,
    val : String
}



pub fn construct_token(input : String)-> Token{
    let t_type : Token_Type;
    
    let identifier_regex : Regex = Regex.new(r"[[:alpha:]]+").unwrap();
    let integer_regex : Regex = Regex.new(r"[0-9]+").unwrap();

    if 
    input == "(" || 
    input == ")" || 
    input == "{" || 
    input == "}" || 
    input == ";"{
        t_type = Token_Type::separator;
    }
    else if 
    input == "if" ||
    input == "else if" ||
    input == "else" ||
    input == "while" ||
    input == "for" ||
    input == "return" ||
    input == "int" ||
    input == "float" ||
    input == "bool" ||
    input == "char" {
        t_type = Token_Type::keyword;
    }
    else if
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
    input == "-="{
        t_type = Token_Type::operator;
    }
    else if identifier_regex.is_match(&input) {
        t_type = Token_Type::identifier;
    }
    else if integer_regex.is_match(&input) {
        t_type = Token_Type::constant;
    }

    return Token{token_type : t_type, val : input};
}