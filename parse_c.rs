/* 
This file will be responsible for creating a parser which takes in a list of 
tokens from the lexer, and returns a parse tree. This file will contain the main
code necessary to syntactically analyze the source file.
*/

/* 
This is the grammer that is currently being implemeted by the parser:

    x before a rule means it was noted but not used.
    Assume that functions don't have parameters/arguments.

    program_start -> other_decl
x   func_decl -> primitive identifier (arguments);
    func_decl -> primitive identifier (arguments){body}
    other_decl -> [func_decl other_decl] | empty
    
    arguments -> primitive identifier, arguments | empty

    body -> statement | statement body

    assign_expr -> identifier = expr

    expr -> arith_expr | condition_expr

    optional_expr -> var_decl | assign_expr | expr | empty

    statement -> var_decl;
    var_decl -> primitive identifier
    var_decl -> primitive assign_expr

    statement -> assign_expr ;

x   expression -> identifier = expression
    
    a = identifier | constant

    arith_expr -> arith_term subexpr
    subexpr -> [+ arith_term subexpr] | [- arith_term subexpr] | arith_term | empty
    arith_term -> arith_factor arith_subterm
    arith_subterm -> [* arith_factor arith_subterm] | [/ arith_factor arith_subterm] | arith_factor | empty
    arith_factor -> a | (arith_expr)

    bool_expr -> bool_term bool_subexpr
    bool_subexpr -> [|| bool_term bool_subexpr] | empty
    bool_term -> bool_factor bool_subterm
    bool_subterm -> [&& bool_factor bool_subterm] | empty
    bool_factor -> bool_operand bool_subfactor
    bool_subfactor -> [== | !=] bool_operand bool_subfactor | empty
    bool_operand -> [! bool_expr] | id | keyword

    relational_expr -> arith_expr [< | <= | > | >=] arith_expr


    statement -> while_statement
    
    while_statement -> keyword (condition_expr){body}
    condition_expr -> bool_expr | relational_expr



    statement -> for_statement

    for_statement -> keyword (optional_expr ; optional_expr ; optional_expr) {body}

    statement -> if_stmt
    if_stmt -> keyword (condition_expr){body} elif_stmt
    elif_stmt -> [keyword(condition_expr){body} elif_stmt] | else_stmt |empty
    else_stmt -> [keyword {body}] | empty

    statement -> ret_stmt
    ret_stmt -> keyword expression ;
*/

use std::collections::HashMap;
use std::rc::Rc;

use crate::token_c::{TokenType, Token};
use crate::expression_c::{*};
use crate::statement_c::{*};
use crate::symbol_table_c::{*};

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


pub fn get_current_token_index() -> usize {
    unsafe {
        return CURRENT_TOKEN_INDEX as usize;
    }
    
}


pub fn next_token_index() -> usize {
    unsafe {
        CURRENT_TOKEN_INDEX += 1;
        return CURRENT_TOKEN_INDEX as usize;
    }
    
}

pub fn prev_token_index() -> usize {
    unsafe {
        CURRENT_TOKEN_INDEX -= 1;
        return CURRENT_TOKEN_INDEX as usize;
    }
    
}




#[allow(non_camel_case_types)]
pub enum NodeType {
    
    Program_Start,
    Func_Decl,
    Other_Decl,
    Arguments,
    Primitive,
    Identifier,
    Body,
    Assign_Expr,
    Expression,
    Optional_Expr,
    Condition_Expr,
    Arith_Expr,
    Arith_Subexpr,
    Arith_Term,
    Arith_Subterm,
    Arith_Factor,
    Bool_Expr,
    Bool_Subexpr,
    Bool_Term,
    Bool_Subterm,
    Bool_Factor,
    Bool_Subfactor,
    Bool_Operand,
    Relational_Expr,
    Statement,
    If_Stmt,
    Elif_Stmt,
    Else_Stmt,
    While_Stmt,
    For_Stmt,
    Return_Stmt,
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



pub fn parse(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool{

    match current_node.node_type {

        NodeType::Program_Start => parse_start_node(current_node, tokens, symbol_table),
        
        NodeType::Other_Decl => parse_other_decl(current_node, tokens, symbol_table),

        NodeType::Func_Decl => parse_func_decl(current_node, tokens, symbol_table),

        NodeType::Arguments => parse_arguments(current_node, tokens, symbol_table),

        NodeType::Primitive => parse_terminal(current_node, tokens, &TokenType::Primitive),

        NodeType::Identifier => parse_terminal(current_node, tokens, &TokenType::Identifier),

        NodeType::Separator => parse_terminal(current_node, tokens, &TokenType::Separator),

        NodeType::Body => parse_body(current_node, tokens, symbol_table),

        NodeType::Assign_Expr => parse_assign_expr(current_node, tokens, symbol_table),

        NodeType::Expression => parse_expr(current_node, tokens, symbol_table),

        NodeType::Arith_Expr => parse_arith_expr(current_node, tokens, symbol_table),

        NodeType::Arith_Subexpr => parse_arith_subexpr(current_node, tokens, symbol_table),

        NodeType::Arith_Term => parse_arith_term(current_node, tokens, symbol_table),

        NodeType::Arith_Subterm => parse_arith_subterm(current_node, tokens, symbol_table),

        NodeType::Arith_Factor => parse_arith_factor(current_node, tokens, symbol_table),

        NodeType::Bool_Expr => parse_bool_epxr(current_node, tokens, symbol_table),

        NodeType::Bool_Subexpr => parse_bool_subepxr(current_node, tokens, symbol_table),

        NodeType::Bool_Term => parse_bool_term(current_node, tokens, symbol_table),

        NodeType::Bool_Subterm => parse_bool_subterm(current_node, tokens, symbol_table),

        NodeType::Bool_Factor => parse_bool_factor(current_node, tokens, symbol_table),

        NodeType::Bool_Subfactor => parse_bool_subfactor(current_node, tokens, symbol_table),

        NodeType::Bool_Operand => parse_bool_operand(current_node, tokens, symbol_table),

        NodeType::Relational_Expr => parse_relational_expr(current_node, tokens, symbol_table),

        NodeType::Condition_Expr => parse_cond_expr(current_node, tokens, symbol_table),

        NodeType::Optional_Expr => parse_optional_expr(current_node, tokens, symbol_table),

        NodeType::Statement => parse_statement(current_node, tokens, symbol_table),

        NodeType::VarDecl => parse_var_decl(current_node, tokens, symbol_table),

        NodeType::Return_Stmt => parse_ret_stmt(current_node, tokens, symbol_table),

        NodeType::If_Stmt => parse_if_stmt(current_node, tokens, symbol_table),

        NodeType::Elif_Stmt => parse_elif_stmt(current_node, tokens, symbol_table),

        NodeType::Else_Stmt => parse_else_stmt(current_node, tokens, symbol_table),

        NodeType::While_Stmt => parse_while_stmt(current_node, tokens, symbol_table),

        NodeType::For_Stmt => parse_for_stmt(current_node, tokens, symbol_table),

        NodeType::Keyword => parse_terminal(current_node, tokens, &TokenType::Keyword),

        NodeType::Operator => parse_terminal(current_node, tokens, &TokenType::Operator),

        NodeType::Constant => parse_terminal(current_node, tokens, &TokenType::Constant)
    
    }


}

fn parse_start_node(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    //Create a new node of type function declaration            
    let mut other_decl_node : Node = create_node(NodeType::Other_Decl);

    
    /* 
    Parse the function declaration node first. If it returns a node, then
    the parser is free to continue, if it returns None, then the parser
    must backtrack.
    */

    if parse(&mut other_decl_node, tokens, symbol_table) {
        current_node.children.push(other_decl_node);
        return true;
    }
    return false;
}

fn parse_func_decl(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    //New scope made here

    symbol_table.push_child(1);
    let current_table = &symbol_table.children.borrow()[symbol_table.children.borrow().len() - 1];


    let mut primitive_node : Node = create_node(NodeType::Primitive);
    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut arguments_node : Node = create_node(NodeType::Arguments);
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
    parse(&mut primitive_node, tokens, current_table) &&
    parse(&mut identifier_node, tokens, current_table) &&
    parse(&mut open_paren_node, tokens, current_table) &&
    parse(&mut arguments_node, tokens, current_table) &&
    parse(&mut close_paren_node, tokens, current_table) &&
    parse(&mut open_curly_node, tokens, current_table) &&
    parse(&mut body_node, tokens, current_table) &&
    parse(&mut close_curly_node, tokens, current_table)
    {
        symbol_table.bind(&identifier_node.properties["value"], &primitive_node.properties["value"], true);
        
        current_node.children.push(primitive_node);
        current_node.children.push(identifier_node);
        current_node.children.push(open_paren_node);
        current_node.children.push(arguments_node);
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

fn parse_terminal(current_node : &mut Node, tokens : &Vec<Token>, tok_type : &TokenType) -> bool {

    if tok_type == &tokens[get_current_token_index()].token_type {
        current_node.properties.insert("value".to_string(), (&tokens[get_current_token_index()].val).clone());
        next_token_index();
        return true;
    }
    return false;
    
}


fn parse_arguments(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut prim_node : Node = create_node(NodeType::Primitive);
    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut comma_node : Node = create_node(NodeType::Separator);
    let mut argument_node : Node = create_node(NodeType::Arguments);

    if
    parse(&mut prim_node, tokens, symbol_table) &&
    parse(&mut identifier_node, tokens, symbol_table) &&
    parse(&mut comma_node, tokens, symbol_table) &&
    parse(&mut argument_node, tokens, symbol_table) {
        
        current_node.children.push(prim_node);
        current_node.children.push(identifier_node);
        current_node.children.push(comma_node);
        current_node.children.push(argument_node);
        return true;
    }
    else if 
    tokens[get_current_token_index()].val == ")" {
        
        return true;
    }


    return false;
}

fn parse_other_decl(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut func_decl_node: Node = create_node(NodeType::Func_Decl);
    let mut other_decl_node : Node = create_node(NodeType::Other_Decl);

    if get_current_token_index() == tokens.len() {
        return true;
    }
    else if 
    parse(&mut func_decl_node, tokens, symbol_table) &&
    parse(&mut other_decl_node, tokens, symbol_table) {

        current_node.children.push(func_decl_node);
        current_node.children.push(other_decl_node);

        return true;
    }

    return false;
}