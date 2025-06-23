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
    var_decl -> primitive idenitifer = expression ;
    var_decl -> identifier = expression ;

x   expression -> identifier = expression
    
    a = identifier | constant

    expression -> a subexpr
    subexpr -> [+ term subexpr] | [- term subexpr] | term | empty
    term -> a subterm
    subterm -> [* factor subterm] | [/ factor subterm] | factor | empty
    factor -> a | (expression)

    expression -> constant | identifier + expression
    expression -> constant | identifier

    statement -> ret_stmt
    ret_stmt -> keyword expression ;
*/

use std::collections::HashMap;

use crate::token_c::{self, is_identifier, is_operator, is_primitive, is_separator, TokenType};

static mut CURRENT_TOKEN_INDEX : u32 = 0; 

//This function returns the size of a primitive based on the declared type
fn get_primitive_size(prim : &String) -> u32 {
    let primitive : &str = prim.as_str();
    match primitive {
        //For now we make them all take up a register's worth of space
        "int" => 8,
        "char" => 8,
        "bool" => 8,
        _ => 0
    }
}


fn get_current_token_index() -> usize {
    unsafe {
        return CURRENT_TOKEN_INDEX as usize;
    }
    
}

fn token_lookahead() -> usize {
    unsafe {
        return (CURRENT_TOKEN_INDEX + 1) as usize;
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

pub struct Symbol {
    pub primitive : String, 
    pub addr : u32,
    pub size : u32,
    pub register : i32
}

pub struct STManager {
    pub symbol_table : HashMap<String, Symbol>,
    pub ordinal : u32
}


impl STManager {
    /* Handles updating address for each local variable from base of stack frame
    for easier assembly generation */
    fn insert(&mut self, identifier : &String, prim : &String) {
        //Construct symbol
        self.symbol_table.insert(identifier.clone(), Symbol{primitive : prim.clone(), addr : self.ordinal * 8, size : get_primitive_size(&prim), register : -1});

        //Update stack pointer
        self.ordinal += 1;
    }

    pub fn query(&self, identifier : &String) -> Option<&Symbol>{
        return self.symbol_table.get(identifier)
    }

    pub fn modify_register(&mut self, identifier : &String, register : i32) {
        self.symbol_table.insert(
        identifier.clone(), 
        Symbol {
            primitive: self.symbol_table[identifier].primitive.clone(),
            addr: self.symbol_table[identifier].addr,
            size: self.symbol_table[identifier].size,
            register: register 
        });
    }

}


#[allow(non_camel_case_types)]
pub enum NodeType {
    
    Program_Start,
    Function_Declaration,
    Primitive,
    Identifier,
    Body,
    Expression,
    Subexpr,
    Term,
    Subterm,
    Factor,
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
    pub properties : HashMap<String, String>,
    pub register : i32 //-1 implies that there is no register assigned
}

pub fn create_node(n_type : NodeType) -> Node {
    return Node {
        node_type : n_type,
        children : Vec::new(),
        properties : HashMap::new(),
        register : -1
    };
} 



pub fn parse(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool{

    match current_node.node_type {

        NodeType::Program_Start => {
            return parse_start_node(current_node, tokens, symbol_table);
        }
        
        NodeType::Function_Declaration => {
            return parse_func_decl(current_node, tokens, symbol_table);
        }

        NodeType::Primitive => {
            return parse_terminal(current_node, tokens, &token_c::TokenType::Primitive);
        }

        NodeType::Identifier => {
            return parse_terminal(current_node, tokens, &token_c::TokenType::Identifier);
        }

        NodeType::Separator => {
            return parse_terminal(current_node, tokens, &token_c::TokenType::Separator);
        }

        NodeType::Body => {
            return parse_body(current_node, tokens, symbol_table);
        }
        NodeType::Expression => {
            return parse_expression(current_node, tokens, symbol_table)
        }
        NodeType::Subexpr => {
            return parse_subexpr(current_node, tokens, symbol_table)
        }
        NodeType::Term => {
            return parse_term(current_node, tokens, symbol_table);
        }
        NodeType::Subterm => {
            return parse_subterm(current_node, tokens, symbol_table)
        }
        NodeType::Factor => {
            return parse_factor(current_node, tokens, symbol_table);
        }
        NodeType::Statement => {
            return parse_statement(current_node, tokens, symbol_table);            
        }
        NodeType::VarDecl => {
            return parse_var_decl(current_node, tokens, symbol_table);
        }
        NodeType::ReturnStatement => {
            return parse_ret_stmt(current_node, tokens, symbol_table);
        }
        NodeType::Keyword => {
            return parse_terminal(current_node, tokens, &token_c::TokenType::Keyword);
        }
        NodeType::Operator => {
            return parse_terminal(current_node, tokens, &token_c::TokenType::Operator);
        }
        NodeType::Constant => {
            return parse_terminal(current_node, tokens, &token_c::TokenType::Constant);
        }


    }


}

fn parse_start_node(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    //Create a new node of type function declaration            
            let mut function_declaration_node : Node = create_node(NodeType::Function_Declaration);

            
            /* 
            Parse the function declaration node first. If it returns a node, then
            the parser is free to continue, if it returns None, then the parser
            must backtrack.
            */

            if parse(&mut function_declaration_node, tokens, symbol_table) {
                current_node.children.push(function_declaration_node);
                return true;
            }
            else {
                return false;
            }
}

fn parse_func_decl(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) ->bool {
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
    parse(&mut primitive_node, tokens, symbol_table) &&
    parse(&mut identifier_node, tokens, symbol_table) &&
    parse(&mut open_paren_node, tokens, symbol_table) &&
    parse(&mut close_paren_node, tokens, symbol_table) &&
    parse(&mut open_curly_node, tokens, symbol_table) &&
    parse(&mut body_node, tokens, symbol_table) &&
    parse(&mut close_curly_node, tokens, symbol_table)
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

fn parse_terminal(current_node : &mut Node, tokens : &Vec<token_c::Token>, tok_type : &TokenType) -> bool {

    if tok_type == &tokens[get_current_token_index()].token_type {
        current_node.properties.insert("value".to_string(), (&tokens[get_current_token_index()].val).clone());
        next_token_index();
        return true;
    }
    return false;
    
}

fn parse_statement(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) ->bool {

    /* Include all rules for CFGs that have statements on the LHS here. */

    if tokens[get_current_token_index()].val == "return".to_string() {
        let mut return_node : Node = create_node(NodeType::ReturnStatement);

        if parse(&mut return_node, tokens, symbol_table) 
        {
            current_node.children.push(return_node);
            return true;
        }
        else {
            return false;
        }
    }
    else if 
    is_primitive(&tokens[get_current_token_index()].val) ||
    is_identifier(&tokens[get_current_token_index()].val) {
        //Then we have found a variable declaration
        let mut var_decl : Node = create_node(NodeType::VarDecl);

        if parse(&mut var_decl, tokens, symbol_table) 
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

fn parse_var_decl(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) ->bool {
    let mut primitive_node : Node = create_node(NodeType::Primitive);
    let mut identity_node : Node = create_node(NodeType::Identifier);
    if 
    parse(&mut primitive_node, tokens, symbol_table) && parse(&mut identity_node, tokens, symbol_table)
    {
        current_node.children.push(primitive_node);
        current_node.children.push(identity_node);
        
        let mut semicolon_node : Node = create_node(NodeType::Separator);
        let mut operator_node : Node = create_node(NodeType::Operator);
        let mut expression_node : Node = create_node(NodeType::Expression);

        if is_separator(&tokens[get_current_token_index()].val) {
            parse(&mut semicolon_node, tokens, symbol_table);
            current_node.children.push(semicolon_node);
            
            current_node.properties.insert("value".to_string(), "0".to_string());
            current_node.properties.insert("identifier".to_string(), current_node.children[1].properties["value"].clone());
            
            symbol_table.insert(&current_node.children[1].properties["value"], &current_node.children[0].properties["value"]);
            return true;
        }
        else if is_operator(&tokens[get_current_token_index()].val) {

            parse(&mut operator_node, tokens, symbol_table);
            parse(&mut expression_node, tokens, symbol_table);
            parse(&mut semicolon_node, tokens, symbol_table);

            current_node.children.push(operator_node);
            current_node.children.push(expression_node);
            current_node.children.push(semicolon_node);

            current_node.properties.insert("identifier".to_string(), current_node.children[1].properties["value"].clone());
            symbol_table.insert(&current_node.children[1].properties["value"], &current_node.children[0].properties["value"]);
            return true;

        }
        return false;

    }
    else if parse(&mut identity_node, tokens, symbol_table) {
        let mut operator_node : Node = create_node(NodeType::Operator);
        let mut expression_node : Node = create_node(NodeType::Expression);
        let mut semicolon_node : Node = create_node(NodeType::Separator);

        if
        parse(&mut operator_node, tokens, symbol_table) &&
        parse(&mut expression_node, tokens, symbol_table) &&
        parse(&mut semicolon_node, tokens, symbol_table) {
            
            current_node.children.push(identity_node);
            current_node.children.push(operator_node);
            current_node.children.push(expression_node);
            
            current_node.properties.insert("identifier".to_string(), current_node.children[0].properties["value"].clone());
            current_node.children.push(semicolon_node);

            return true;
        }
    }

    return false;
}

fn parse_ret_stmt(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) ->bool {
    let mut return_node : Node = create_node(NodeType::Keyword);
    let mut expression_node : Node = create_node(NodeType::Expression);
    let mut semicolon_node : Node = create_node(NodeType::Separator);



    if
    parse(&mut return_node, tokens, symbol_table) &&
    parse(&mut expression_node, tokens, symbol_table) &&
    parse(&mut semicolon_node, tokens, symbol_table) {
        
        current_node.children.push(return_node);
        current_node.children.push(expression_node);
        current_node.children.push(semicolon_node);

        return true;
    }

    return false;

    
   
}

fn parse_body(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) ->bool {
    while tokens[get_current_token_index()].val != "}".to_string(){
        let mut stmt_node : Node = create_node(NodeType::Statement);
        
        if parse(&mut stmt_node, tokens, symbol_table)                 
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

fn parse_expression(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut constant_node : Node = create_node(NodeType::Constant);
    let mut subexpr_node : Node = create_node(NodeType::Subexpr);

    let identifier_parse : bool = parse(&mut identifier_node, tokens, symbol_table);
    let constant_parse : bool = parse(&mut constant_node, tokens, symbol_table);

    if identifier_parse != constant_parse && parse(&mut subexpr_node, tokens, symbol_table) {

        current_node.children.push(if identifier_parse {identifier_node} else {constant_node});
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["value"].clone());

        current_node.children.push(subexpr_node);
        if current_node.children[1].properties.contains_key("operator") {
            current_node.properties.insert("operator".to_string(), current_node.children[1].properties["operator"].clone());
        }
        return true;
    }
    return false;
}

fn parse_subexpr(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    subexpr -> [+ term subexpr] | [- term subexpr] | term | empty
     */
    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut term_node : Node = create_node(NodeType::Term);
    let mut subexpr_node : Node = create_node(NodeType::Subexpr);

    if 
    (tokens[get_current_token_index()].val == "+".to_string() ||
    tokens[get_current_token_index()].val == "-".to_string()) &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut term_node, tokens, symbol_table) &&
    parse(&mut subexpr_node, tokens, symbol_table) {
        
        //First 2 rules + semantic checking
        
        current_node.properties.insert("operator".to_string(), operator_node.properties["value"].clone());
        current_node.children.push(operator_node);

        current_node.children.push(term_node);
        current_node.children.push(subexpr_node);

        return true;
    }
    else if parse(&mut term_node, tokens, symbol_table) {

        //3rd rule
        current_node.children.push(term_node);
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        return true;
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        
        //This is equivalent to the empty character case because ; signifies the end of the expression
        return true;
    }

    return false;
}

fn parse_term(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    term -> constant | identifier subterm
     */

    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut constant_node : Node = create_node(NodeType::Constant);
    let mut subterm_node : Node = create_node(NodeType::Subterm);
    let identifier_parse : bool = parse(&mut identifier_node, tokens, symbol_table);
    let constant_parse : bool = parse(&mut constant_node, tokens, symbol_table);

    if identifier_parse != constant_parse  && parse(&mut subterm_node, tokens, symbol_table){
        
        current_node.children.push(if identifier_parse {identifier_node} else {constant_node});
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["value"].clone());

        current_node.children.push(subterm_node);
        if current_node.children[1].properties.contains_key("operator") {
            current_node.properties.insert("operator".to_string(), current_node.children[1].properties["operator"].clone());
        }
        return true;
    }

    return false;
}

fn parse_subterm(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    subterm -> [* factor subterm] | [/ factor subterm] | factor | empty
     */
    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut factor_node : Node = create_node(NodeType::Factor);
    let mut subterm_node : Node = create_node(NodeType::Subterm);

    if 
    (tokens[get_current_token_index()].val == "*".to_string() ||
    tokens[get_current_token_index()].val == "/".to_string()) &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut factor_node, tokens, symbol_table) &&
    parse(&mut subterm_node, tokens, symbol_table) {
        
        //First 2 rules + semantic checking
        
        current_node.properties.insert("operator".to_string(), operator_node.properties["value"].clone());
        current_node.children.push(operator_node);

        current_node.children.push(factor_node);
        current_node.children.push(subterm_node);

        return true;
    }
    else if parse(&mut factor_node, tokens, symbol_table) {

        //3rd rule
        current_node.children.push(factor_node);
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        return true;
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        
        //This is equivalent to the empty character case because ; signifies the end of the expression
        let mut semicolon_node : Node = create_node(NodeType::Separator);
        // parse(&mut semicolon_node, tokens, symbol_table);
        return true;
    }

    return false;
}

fn parse_factor(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    Factor -> constant | identifier | (expression)
     */
    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut constant_node : Node = create_node(NodeType::Constant);
    let identifier_parse : bool = parse(&mut identifier_node, tokens, symbol_table);
    let constant_parse : bool = parse(&mut constant_node, tokens, symbol_table);

    if identifier_parse != constant_parse {
        current_node.children.push(if identifier_parse {identifier_node} else {constant_node});
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["value"].clone());

        return true;
    }

    return false;
}