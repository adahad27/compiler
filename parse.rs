/* 
This file will be responsible for creating a parser which takes in a list of 
tokens from the lexer, and returns a parse tree. This file will contain all code
necessary to syntactically analyze the source file.
*/

/* 
This is the grammer that is currently being implemeted by the parser:

    x before a rule means it was noted but not used.
    Assume that functions don't have parameters/arguments.

    program_start -> function declaration
x   function declaration -> primitive identifier () ;
    function declaration -> primitive identifier () {body}
    body -> keyword constant ;
*/

static mut current_token : u32 = 0; 

enum NodeType {
    Program_Start,
    Function_Declaration,
    Primitive,
    Identifier,
    Open_Curly,
    Body,
    Close_Curly,
    Return, //return is a reserved keyword
    Constant,
    Semicolon
}

struct Node {
    is_terminal : bool,
    node_type : NodeType,
    children : Vec<Node>
}


