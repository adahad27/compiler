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
x   function declaration -> primitive identifier ();
    function declaration -> primitive identifier (){body}
    
    body -> statement | statement body

    statement -> var_decl
    var_decl -> primitive identifier;
    var_decl -> primitive idenitifer = constant;

    statement -> ret_stmt
    ret_stmt -> keyword constant ;
*/

static mut CURRENT_TOKEN_INDEX : u32 = 0; 

// mod token;


use crate::token::{self, is_primitive, TokenType};

fn get_current_token_index() -> usize {
    unsafe {
        return CURRENT_TOKEN_INDEX as usize;
    }
    
}

fn next_token_index() -> usize {
    unsafe {
        CURRENT_TOKEN_INDEX += 1;
        return CURRENT_TOKEN_INDEX as usize;
    }
    
}

fn prev_token_index() ->usize {
    unsafe {
        CURRENT_TOKEN_INDEX -= 1;
        return CURRENT_TOKEN_INDEX as usize;
    }
}


#[allow(non_camel_case_types)]
pub enum NodeType {
    
    Program_Start,
    Function_Declaration,
    Primitive,
    Identifier,
    Body,
    Statement,
    ReturnStatement,
    VarDecl,
    Keyword,
    Constant,
    Operator,
    Separator
}

pub struct Node {
    pub node_type : NodeType,
    pub children : Vec<Node>,
    pub value : String
}

pub fn create_node(n_type : NodeType) -> Node {
    return Node {
        node_type : n_type,
        children : Vec::new(),
        value : "".to_string()
    };
} 



pub fn parse(current_node : &mut Node, tokens : &Vec<token::Token>) -> bool{

    match current_node.node_type {

        NodeType::Program_Start => {
            return parse_start_node(current_node, tokens);
        }
        
        NodeType::Function_Declaration => {
            return parse_func_decl(current_node, tokens);
        }

        NodeType::Primitive => {

            /* 
            We check if the current token has primitive_type, if so, then it is
            the correct case and we can return.
            */
            return parse_terminal(current_node, tokens, &token::TokenType::Primitive);
            
        }

        NodeType::Identifier => {

            return parse_terminal(current_node, tokens, &token::TokenType::Identifier);
        }

        NodeType::Separator => {
            return parse_terminal(current_node, tokens, &token::TokenType::Separator);
        }

        NodeType::Body => {

            while tokens[get_current_token_index()].val != "}".to_string(){
                let mut stmt_node : Node = create_node(NodeType::Statement);
                
                if parse(&mut stmt_node, tokens)                 
                {
                    current_node.children.push(stmt_node);
                }
                else 
                {
                    return false;
                }

            }
            return true;
            
        }

        NodeType::Statement => {
            
            if tokens[get_current_token_index()].val == "return".to_string() {
                let mut return_node : Node = create_node(NodeType::ReturnStatement);

                if parse(&mut return_node, tokens) 
                {
                    current_node.children.push(return_node);
                    return true;
                }
                else {
                    return false;
                }
            }
            else if is_primitive(&tokens[get_current_token_index()].val) {
                //Then we have found a variable declaration
                let mut var_decl : Node = create_node(NodeType::VarDecl);

                if parse(&mut var_decl, tokens) 
                {
                    current_node.children.push(var_decl);
                    return true;
                }
                else {
                    return false;
                }
                
            }
            return false;
        }
        NodeType::VarDecl => {
            let mut primitive_node : Node = create_node(NodeType::Primitive);
            let mut identity_node : Node = create_node(NodeType::Identifier);

            if 
            parse(&mut primitive_node, tokens) && parse(&mut identity_node, tokens)
            {
                
                current_node.children.push(primitive_node);
                current_node.children.push(identity_node);
                
                let mut semicolon_node : Node = create_node(NodeType::Separator);
                let mut operator_node : Node = create_node(NodeType::Operator);
                let mut constant_node : Node = create_node(NodeType::Constant);

                if parse(&mut semicolon_node, tokens)
                {
                    current_node.children.push(semicolon_node);
                    current_node.value = "0".to_string();
                    return true;
                }
                else if 
                parse(&mut operator_node, tokens) &&
                parse(&mut constant_node, tokens) &&
                parse(&mut semicolon_node, tokens)
                {
                    current_node.children.push(operator_node);
                    current_node.children.push(constant_node);
                    current_node.children.push(semicolon_node);
                    current_node.value = current_node.children[1].value.clone();
                }
                else {
                    return false;
                }
                return true;
            }

            return false;
        }
        NodeType::ReturnStatement => {
            let mut return_node : Node = create_node(NodeType::Keyword);
            let mut constant_node : Node = create_node(NodeType::Constant);
            let mut semicolon_node : Node = create_node(NodeType::Separator);

            if 
            parse(&mut return_node, tokens) &&
            parse(&mut constant_node, tokens) &&
            parse(&mut semicolon_node, tokens)
            {
                current_node.children.push(return_node);
                current_node.children.push(constant_node);
                current_node.children.push(semicolon_node);
                return true;
            }
            else {
                return false;
            }
        }
        NodeType::Keyword => {
            return parse_terminal(current_node, tokens, &token::TokenType::Keyword);
        }
        NodeType::Operator => {
            return parse_terminal(current_node, tokens, &token::TokenType::Operator);
        }
        NodeType::Constant => {
            return parse_terminal(current_node, tokens, &token::TokenType::Constant);
        }


    }


}

fn parse_start_node(current_node : &mut Node, tokens : &Vec<token::Token>) -> bool {
    //Create a new node of type function declaration            
            let mut function_declaration_node : Node = create_node(NodeType::Function_Declaration);

            
            /* 
            Parse the function declaration node first. If it returns a node, then
            the parser is free to continue, if it returns None, then the parser
            must backtrack.
            */

            if parse(&mut function_declaration_node, tokens) {
                current_node.children.push(function_declaration_node);
                return true;
            }
            else {
                return false;
            }
}

fn parse_func_decl(current_node : &mut Node, tokens : &Vec<token::Token>) ->bool {
    let mut primitive_node : Node = create_node(NodeType::Primitive);
    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut close_paren_node : Node = create_node(NodeType::Separator);
    let mut open_curly_node : Node = create_node(NodeType::Separator);
    let mut body_node : Node = create_node(NodeType::Body);
    let mut close_curly_node : Node = create_node(NodeType::Separator);

    /* 
    To add backtracking, all we should theoretically have to do is add
    elif statements here to check if the next rule matches, and keep on
    doing this exhaustively for each rule.
    */

    if 
    parse(&mut primitive_node, tokens) &&
    parse(&mut identifier_node, tokens) &&
    parse(&mut open_paren_node, tokens) &&
    parse(&mut close_paren_node, tokens) &&
    parse(&mut open_curly_node, tokens) &&
    parse(&mut body_node, tokens) &&
    parse(&mut close_curly_node, tokens)
    {
        current_node.children.push(primitive_node);
        current_node.children.push(identifier_node);
        current_node.children.push(open_paren_node);
        current_node.children.push(close_paren_node);
        current_node.children.push(open_curly_node);
        current_node.children.push(body_node);
        current_node.children.push(close_curly_node);
        
        return true;
    }
    else {
        return false;
    }
}

fn parse_terminal(current_node : &mut Node, tokens : &Vec<token::Token>, tok_type : &TokenType) -> bool {

    if tok_type == &tokens[get_current_token_index()].token_type {
        current_node.value = (&tokens[get_current_token_index()].val).clone();
        next_token_index();
        return true;
    }
    return false;
    
}

fn main() {
    println!("Just for compliation");
}
