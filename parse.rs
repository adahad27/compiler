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
    
    body -> statement body

    statement -> var_decl
    var_decl -> primitive identifier;
    var_decl -> primitive idenitifer = constant;

    statement -> ret_stmt
    ret_stmt -> keyword constant ;
*/

static mut CURRENT_TOKEN_INDEX : u32 = 0; 

// mod token;


use crate::token;

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
    Open_Paren,
    Close_Paren,
    Open_Curly,
    Body,
    Close_Curly,
    ReturnStatement,
    Keyword,
    Constant,
    Semicolon
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



pub fn parse(mut current_node : Node, tokens : &Vec<token::Token>) -> Option<Node> {

    match current_node.node_type {

        NodeType::Program_Start => {

            //Create a new node of type function declaration            
            let function_declaration_node : Node = create_node(NodeType::Function_Declaration);

            
            /* 
            Parse the function declaration node first. If it returns a node, then
            the parser is free to continue, if it returns None, then the parser
            must backtrack.
            */
            if let Option::Some(node) = parse(function_declaration_node, tokens) {
                current_node.children.push(node);
            }
            else {
                return Option::None;
            }

            
            //Return current_node
            return Option::Some(current_node);
        }
        
        NodeType::Function_Declaration => {

            let primitive_node : Node = create_node(NodeType::Primitive);
            let identifier_node : Node = create_node(NodeType::Identifier);
            let open_paren_node : Node = create_node(NodeType::Open_Paren);
            let close_paren_node : Node = create_node(NodeType::Close_Paren);
            let open_curly_node : Node = create_node(NodeType::Open_Curly);
            let body_node : Node = create_node(NodeType::Body);
            let close_curly_node : Node = create_node(NodeType::Close_Curly);

            /* 
            To add backtracking, all we should theoretically have to do is add
            elif statements here to check if the next rule matches, and keep on
            doing this exhaustively for each rule.
            */

            if let
            (
                Option::Some(prim_node), Option::Some(iden_node), Option::Some(o_paren_node),
                Option::Some(c_paren_node), Option::Some(o_curly_node), Option::Some(b_node),
                Option::Some(c_curly_node)
            ) =
            (
                parse(primitive_node, tokens), parse(identifier_node, tokens), parse(open_paren_node, tokens),
                parse(close_paren_node, tokens), parse(open_curly_node, tokens), parse(body_node, tokens),
                parse(close_curly_node, tokens)
            ) 
            {
                current_node.children.push(prim_node);
                current_node.children.push(iden_node);
                current_node.children.push(o_paren_node);
                current_node.children.push(c_paren_node);
                current_node.children.push(o_curly_node);
                current_node.children.push(b_node);
                current_node.children.push(c_curly_node);
            }
            else {
                return Option::None;
            }
            

            return Option::Some(current_node);
        }

        NodeType::Primitive => {

            /* 
            We check if the current token has primitive_type, if so, then it is
            the correct case and we can return.
            */
            
            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Primitive) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }

        NodeType::Identifier => {

            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Identifier) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }

        NodeType::Open_Paren => {

            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Separator) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }

        NodeType::Close_Paren => {
            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Separator) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }

        NodeType::Open_Curly => {
            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Separator) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }

        NodeType::Body => {

            let return_node : Node = create_node(NodeType::ReturnStatement);

            if let Option::Some(ret_node) = parse(return_node, tokens) 
            
            {
                current_node.children.push(ret_node);
            }
            else {
                return Option::None;
            }

            

            return Option::Some(current_node);
        }

        NodeType::Close_Curly => {
            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Separator) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }

        NodeType::ReturnStatement => {
            let return_node : Node = create_node(NodeType::Keyword);
            let constant_node : Node = create_node(NodeType::Constant);
            let semicolon_node : Node = create_node(NodeType::Semicolon);

            if let
            (Option::Some(ret_node), Option::Some(cons_node), Option::Some(semi_node)) = 
            (parse(return_node, tokens), parse(constant_node, tokens), parse(semicolon_node, tokens)) 
            
            {
                current_node.children.push(ret_node);
                current_node.children.push(cons_node);
                current_node.children.push(semi_node);
            }
            else {
                return Option::None;
            }

            

            return Option::Some(current_node);
        }
        NodeType::Keyword => {
            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Keyword) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }
        NodeType::Constant => {
            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Constant) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }

        NodeType::Semicolon => {
            let token::Token{token_type, val} = &tokens[get_current_token_index()];
            if !matches!(token_type, token::TokenType::Separator) {
                //Throw some kind of error here for backtracking
                return Option::None;
            }
            current_node.value = val.clone();
            next_token_index();

            return Option::Some(current_node);
        }

    }

}

fn main() {
    println!("Just for compliation");
}
