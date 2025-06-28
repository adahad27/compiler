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


    optional_expr -> arith_expr | condition_expr | empty

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



pub fn parse(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool{

    match current_node.node_type {

        NodeType::Program_Start => parse_start_node(current_node, tokens, symbol_table),
        
        NodeType::Function_Declaration => parse_func_decl(current_node, tokens, symbol_table),

        NodeType::Primitive => parse_terminal(current_node, tokens, &token_c::TokenType::Primitive),

        NodeType::Identifier => parse_terminal(current_node, tokens, &token_c::TokenType::Identifier),

        NodeType::Separator => parse_terminal(current_node, tokens, &token_c::TokenType::Separator),

        NodeType::Body => parse_body(current_node, tokens, symbol_table),

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

        NodeType::Keyword => parse_terminal(current_node, tokens, &token_c::TokenType::Keyword),

        NodeType::Operator => parse_terminal(current_node, tokens, &token_c::TokenType::Operator),

        NodeType::Constant => parse_terminal(current_node, tokens, &token_c::TokenType::Constant)
    
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
        let mut return_node : Node = create_node(NodeType::Return_Stmt);

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
    else if tokens[get_current_token_index()].val == "if".to_string() {
        let mut if_stmt : Node = create_node(NodeType::If_Stmt);
        if parse(&mut if_stmt, tokens, symbol_table) 
        {
            current_node.children.push(if_stmt);
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
    
    //Defaults to arith_expr
    // let mut expr_node : Node = create_node(NodeType::Arith_Expr);
    // let mut expr_node : Node = create_node(NodeType::Bool_Expr);
    let mut expr_node : Node = create_node(NodeType::Relational_Expr);


    if 
    parse(&mut primitive_node, tokens, symbol_table) && parse(&mut identity_node, tokens, symbol_table) {
        
        current_node.children.push(primitive_node);
        current_node.children.push(identity_node);
        
        if current_node.children[0].properties["value"] == "int".to_string() {
            expr_node = create_node(NodeType::Arith_Expr);
        }
        else if current_node.children[0].properties["value"] == "bool".to_string() {
            expr_node = create_node(NodeType::Condition_Expr);
        }


        let mut semicolon_node : Node = create_node(NodeType::Separator);
        let mut operator_node : Node = create_node(NodeType::Operator);

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
            parse(&mut expr_node, tokens, symbol_table);
            parse(&mut semicolon_node, tokens, symbol_table);

            current_node.children.push(operator_node);
            current_node.children.push(expr_node);
            current_node.children.push(semicolon_node);

            current_node.properties.insert("identifier".to_string(), current_node.children[1].properties["value"].clone());
            symbol_table.insert(&current_node.children[1].properties["value"], &current_node.children[0].properties["value"]);
            return true;

        }
        return false;

    }
    else if parse(&mut identity_node, tokens, symbol_table) {
        let mut operator_node : Node = create_node(NodeType::Operator);
        let mut semicolon_node : Node = create_node(NodeType::Separator);
        
        if symbol_table.query(&identity_node.properties["value"]).unwrap().primitive == "int".to_string() {
            expr_node = create_node(NodeType::Arith_Expr);
        }
        else if symbol_table.query(&identity_node.properties["value"]).unwrap().primitive == "bool".to_string() {
            expr_node = create_node(NodeType::Condition_Expr);
        }

        if
        parse(&mut operator_node, tokens, symbol_table) &&
        parse(&mut expr_node, tokens, symbol_table) &&
        parse(&mut semicolon_node, tokens, symbol_table) {
            
            current_node.children.push(identity_node);
            current_node.children.push(operator_node);
            current_node.children.push(expr_node);
            
            current_node.properties.insert("identifier".to_string(), current_node.children[0].properties["value"].clone());
            current_node.children.push(semicolon_node);

            return true;
        }
    }

    return false;
}

fn parse_ret_stmt(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) ->bool {
    let mut return_node : Node = create_node(NodeType::Keyword);
    let mut arith_expr_node : Node = create_node(NodeType::Arith_Expr);
    let mut semicolon_node : Node = create_node(NodeType::Separator);



    if
    parse(&mut return_node, tokens, symbol_table) &&
    parse(&mut arith_expr_node, tokens, symbol_table) &&
    parse(&mut semicolon_node, tokens, symbol_table) {
        
        current_node.children.push(return_node);
        current_node.children.push(arith_expr_node);
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

fn parse_arith_expr(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    let mut arith_term_node : Node = create_node(NodeType::Arith_Term);
    let mut arith_subexpr_node : Node = create_node(NodeType::Arith_Subexpr);


    if 
    parse(&mut arith_term_node, tokens, symbol_table) && 
    parse(&mut arith_subexpr_node, tokens, symbol_table) {

        current_node.children.push(arith_term_node);
        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }
        

        current_node.children.push(arith_subexpr_node);
        if current_node.children[1].properties.contains_key("operator") {
            current_node.properties.insert("operator".to_string(), current_node.children[1].properties["operator"].clone());
        }
        return true;
    }
    return false;
}

fn parse_arith_subexpr(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    arith_subexpr -> [+ term arith_subexpr] | [- term arith_subexpr] | term | empty
     */
    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut arith_term_node : Node = create_node(NodeType::Arith_Term);
    let mut arith_subexpr_node : Node = create_node(NodeType::Arith_Subexpr);

    if 
    (tokens[get_current_token_index()].val == "+".to_string() ||
    tokens[get_current_token_index()].val == "-".to_string()) &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut arith_term_node, tokens, symbol_table) &&
    parse(&mut arith_subexpr_node, tokens, symbol_table) {
        
        //First 2 rules + semantic checking
        current_node.properties.insert("operator".to_string(), operator_node.properties["value"].clone());
        current_node.children.push(operator_node);

        current_node.children.push(arith_term_node);
        current_node.children.push(arith_subexpr_node);

        return true;
    }
    else if parse(&mut arith_term_node, tokens, symbol_table) {

        //3rd rule
        current_node.children.push(arith_term_node);
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        return true;
    }
    else if is_operator(&tokens[get_current_token_index()].val) {
        return true;
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        
        //This is equivalent to the empty character case because ; signifies the end of the expression
        return true;
    }

    return false;
}

fn parse_arith_term(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    arith_term -> constant | identifier subarith_term
     */

    let mut arith_factor_node : Node = create_node(NodeType::Arith_Factor);
    let mut arith_subterm_node : Node = create_node(NodeType::Arith_Subterm);

    if 
    parse(&mut arith_factor_node, tokens, symbol_table)  && 
    parse(&mut arith_subterm_node, tokens, symbol_table){
        
        current_node.children.push(arith_factor_node);
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());

        current_node.children.push(arith_subterm_node);
        if current_node.children[1].properties.contains_key("operator") {
            current_node.properties.insert("operator".to_string(), current_node.children[1].properties["operator"].clone());
        }
        return true;
    }

    return false;
}

fn parse_arith_subterm(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    arith_subterm -> [* arith_factor arith_subterm] | [/ arith_factor arith_subterm] | arith_factor | empty
     */
    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut arith_factor_node : Node = create_node(NodeType::Arith_Factor);
    let mut arith_subterm_node : Node = create_node(NodeType::Arith_Subterm);
    if 
    (tokens[get_current_token_index()].val == "*".to_string() ||
    tokens[get_current_token_index()].val == "/".to_string()) &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut arith_factor_node, tokens, symbol_table) &&
    parse(&mut arith_subterm_node, tokens, symbol_table) {
        
        //First 2 rules + semantic checking
        
        current_node.properties.insert("operator".to_string(), operator_node.properties["value"].clone());
        current_node.children.push(operator_node);

        current_node.children.push(arith_factor_node);
        current_node.children.push(arith_subterm_node);

        return true;
    }
    else if parse(&mut arith_factor_node, tokens, symbol_table) {

        //3rd rule
        current_node.children.push(arith_factor_node);
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        return true;
    }
    else if is_operator(&tokens[get_current_token_index()].val) {
        return true;
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        
        //This is equivalent to the empty character case because ; signifies the end of the expression
        return true;
    }

    return false;
}

fn parse_arith_factor(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    Arith_Factor -> constant | identifier | (expression)
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

fn parse_bool_epxr(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    let mut bool_term_node : Node = create_node(NodeType::Bool_Term);
    let mut bool_subexpr_node : Node = create_node(NodeType::Bool_Subexpr);
    
    if
    parse(&mut bool_term_node, tokens, symbol_table) &&
    parse(&mut bool_subexpr_node, tokens, symbol_table) {

        current_node.children.push(bool_term_node);

        current_node.children.push(bool_subexpr_node);

        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }

        return true;
    }

    return false;
}

fn parse_bool_subepxr(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    /* 
    Production rules:
    bool_subexpr -> [|| bool_term bool_subexpr] | empty
     */

    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut bool_term_node : Node = create_node(NodeType::Bool_Term);
    let mut bool_subexpr_node : Node = create_node(NodeType::Bool_Subexpr);
    if
    "||" == tokens[get_current_token_index()].val &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut bool_term_node, tokens, symbol_table) &&
    parse(&mut bool_subexpr_node, tokens, symbol_table) {
        //Or case is successful

        current_node.children.push(operator_node);
        current_node.children.push(bool_term_node);
        current_node.children.push(bool_subexpr_node);

        current_node.properties.insert("operator".to_string(), current_node.children[0].properties["value"].clone());
        return true;
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        return true;
    }
    else if is_operator(&tokens[get_current_token_index()].val) {
        return true;
    }
    return false;
}

fn parse_bool_term(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    let mut bool_factor_node : Node = create_node(NodeType::Bool_Factor);
    let mut bool_subterm_node : Node = create_node(NodeType::Bool_Subterm);
    
    if 
    parse(&mut bool_factor_node, tokens, symbol_table) && 
    parse(&mut bool_subterm_node, tokens, symbol_table) {
        
        current_node.children.push(bool_factor_node);
        current_node.children.push(bool_subterm_node);

        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }

        return true;
    }


    return false;
}

fn parse_bool_subterm(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    bool_subterm -> [&& bool_factor bool_subterm] | empty
     */

    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut bool_factor_node : Node = create_node(NodeType::Bool_Factor);
    let mut bool_subterm_node : Node = create_node(NodeType::Bool_Subterm);

    if
    "&&" == tokens[get_current_token_index()].val &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut bool_factor_node, tokens, symbol_table) &&
    parse(&mut bool_subterm_node, tokens, symbol_table) {
        //And case is successful

        current_node.children.push(operator_node);
        current_node.children.push(bool_factor_node);
        current_node.children.push(bool_subterm_node);

        current_node.properties.insert("operator".to_string(), current_node.children[0].properties["value"].clone());
        return true;
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        return true;
    }
    else if is_operator(&tokens[get_current_token_index()].val) {
        return true;
    }
    return false;
}

fn parse_bool_factor(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    let mut bool_operand_node : Node = create_node(NodeType::Bool_Operand);
    let mut bool_subfactor_node : Node = create_node(NodeType::Bool_Subfactor);

    if 
    parse(&mut bool_operand_node, tokens, symbol_table) && 
    parse(&mut bool_subfactor_node, tokens, symbol_table) {
        
        current_node.children.push(bool_operand_node);
        current_node.children.push(bool_subfactor_node);

        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }

        return true;
    }


    return false;
}

fn parse_bool_subfactor(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    /* 
    Production rules:
    bool_subfactor -> [[== | !=] bool_operand bool_subfactor] | empty
     */

    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut bool_operand_node : Node = create_node(NodeType::Bool_Operand);
    let mut bool_subfactor_node : Node = create_node(NodeType::Bool_Subfactor);

    if
    ("==" == tokens[get_current_token_index()].val ||
    "!=" == tokens[get_current_token_index()].val) &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut bool_operand_node, tokens, symbol_table) &&
    parse(&mut bool_subfactor_node, tokens, symbol_table) {
        //Equals/NotEquals case is successful

        current_node.children.push(operator_node);
        current_node.children.push(bool_operand_node);
        current_node.children.push(bool_subfactor_node);

        current_node.properties.insert("operator".to_string(), current_node.children[0].properties["value"].clone());

        return true
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        return true;
    }
    else if is_operator(&tokens[get_current_token_index()].val) {
        return true;
    }
    return false;
}

fn parse_bool_operand(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    
    let mut operator_node : Node = create_node(NodeType::Operator);
    // let mut bool_expr_node : Node = create_node(NodeType::Bool_Expr);
    let mut keyword_node : Node = create_node(NodeType::Keyword);
    let mut identifier_node : Node = create_node(NodeType::Identifier);

    if 
    "!" == tokens[get_current_token_index()].val &&
    parse(&mut operator_node, tokens, symbol_table) {

        current_node.children.push(operator_node);

        current_node.properties.insert("unary".to_string(), "!".to_string());
    }
    if 
    parse(&mut keyword_node, tokens, symbol_table) {
        //Push the node that allowed for the parse to be succesful.
        current_node.children.push(
            if identifier_node.properties.contains_key("value") {
                identifier_node
            }
            else {
                keyword_node
            }
        );
        let terminal : String;
        if
        current_node.children[current_node.children.len() - 1].properties["value"] == "false".to_string() {
            terminal = "0".to_string();
        }
        else {
            terminal = "1".to_string();
        }
        current_node.properties.insert("terminal".to_string(), terminal);
        return true;
    }

    return false;
}

fn parse_relational_expr(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    let (mut arith_expr_left, mut arith_expr_right) = (create_node(NodeType::Arith_Expr), create_node(NodeType::Arith_Expr));
    let mut operator_node : Node = create_node(NodeType::Operator);

    if
    parse(&mut arith_expr_left, tokens, symbol_table) &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut arith_expr_right, tokens, symbol_table) {

        current_node.children.push(arith_expr_left);
        current_node.children.push(operator_node);
        current_node.children.push(arith_expr_right);

        current_node.properties.insert("operator".to_string(), current_node.children[1].properties["value"].clone());

        return true;
    }

    return false;
}

fn parse_if_stmt(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    let mut keyword_node : Node = create_node(NodeType::Keyword);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut cond_node : Node = create_node(NodeType::Condition_Expr);
    let mut close_paren_node : Node = create_node(NodeType::Separator);
    let mut open_curly_node : Node = create_node(NodeType::Separator);
    let mut body_node : Node = create_node(NodeType::Body);
    let mut close_curly_node : Node = create_node(NodeType::Separator);
    let mut elif_stmt_node : Node = create_node(NodeType::Elif_Stmt);

    if 
    parse(&mut keyword_node, tokens, symbol_table) &&
    parse(&mut open_paren_node, tokens, symbol_table) &&
    parse(&mut cond_node, tokens, symbol_table) &&
    parse(&mut close_paren_node, tokens, symbol_table) &&
    parse(&mut open_curly_node, tokens, symbol_table) &&
    parse(&mut body_node, tokens, symbol_table) &&
    parse(&mut close_curly_node, tokens, symbol_table) &&
    parse(&mut elif_stmt_node, tokens, symbol_table) {
        
        current_node.children.push(keyword_node);
        current_node.children.push(open_paren_node);
        current_node.children.push(cond_node);
        current_node.children.push(close_paren_node);
        current_node.children.push(open_curly_node);
        current_node.children.push(body_node);
        current_node.children.push(close_curly_node);
        current_node.children.push(elif_stmt_node);

        return true;

    }

    return false;
}

fn parse_elif_stmt(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    if tokens[get_current_token_index()].val == "elif".to_string() {
        let mut keyword_node : Node = create_node(NodeType::Keyword);
        let mut open_paren_node : Node = create_node(NodeType::Separator);
        let mut cond_node : Node = create_node(NodeType::Condition_Expr);
        let mut close_paren_node : Node = create_node(NodeType::Separator);
        let mut open_curly_node : Node = create_node(NodeType::Separator);
        let mut body_node : Node = create_node(NodeType::Body);
        let mut close_curly_node : Node = create_node(NodeType::Separator);
        let mut elif_stmt_node : Node = create_node(NodeType::Elif_Stmt);
        
        if 
        parse(&mut keyword_node, tokens, symbol_table) &&
        parse(&mut open_paren_node, tokens, symbol_table) &&
        parse(&mut cond_node, tokens, symbol_table) &&
        parse(&mut close_paren_node, tokens, symbol_table) &&
        parse(&mut open_curly_node, tokens, symbol_table) &&
        parse(&mut body_node, tokens, symbol_table) &&
        parse(&mut close_curly_node, tokens, symbol_table) &&
        parse(&mut elif_stmt_node, tokens, symbol_table) {

            current_node.children.push(keyword_node);
            current_node.children.push(open_paren_node);
            current_node.children.push(cond_node);
            current_node.children.push(close_paren_node);
            current_node.children.push(open_curly_node);
            current_node.children.push(body_node);
            current_node.children.push(close_curly_node);
            current_node.children.push(elif_stmt_node);

            return true;
        }
        return false;
    }
    else if tokens[get_current_token_index()].val == "else".to_string() {
        let mut else_stmt_node : Node = create_node(NodeType::Else_Stmt);

        if parse(&mut else_stmt_node, tokens, symbol_table) {
            current_node.children.push(else_stmt_node);
            return true;
        }
        return false;
    }

    return true;
}

fn parse_else_stmt(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    if tokens[get_current_token_index()].val == "else".to_string() {
        let mut keyword_node : Node = create_node(NodeType::Keyword);
        let mut open_curly_node : Node = create_node(NodeType::Separator);
        let mut body_node : Node = create_node(NodeType::Body);
        let mut close_curly_node : Node = create_node(NodeType::Separator);

        if 
        parse(&mut keyword_node, tokens, symbol_table) &&
        parse(&mut open_curly_node, tokens, symbol_table) &&
        parse(&mut body_node, tokens, symbol_table) &&
        parse(&mut close_curly_node, tokens, symbol_table) {


            current_node.children.push(keyword_node);
            current_node.children.push(open_curly_node);
            current_node.children.push(body_node);
            current_node.children.push(close_curly_node);

            return true;

        }
        return false;
    }

    return true;
}

fn parse_cond_expr(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    let mut bool_expr_node : Node = create_node(NodeType::Bool_Expr);
    let mut rel_expr_node : Node = create_node(NodeType::Relational_Expr);

    if parse(&mut bool_expr_node, tokens, symbol_table) {
        //We have a boolean expression
        current_node.children.push(bool_expr_node);
        return true;
    }
    else if parse(&mut rel_expr_node, tokens, symbol_table) {
        //We have a relational expression
        current_node.children.push(rel_expr_node);
        return true;
    }


    return false;
}

fn parse_while_stmt(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {
    let mut keyword_node : Node = create_node(NodeType::Keyword);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut cond_node : Node = create_node(NodeType::Condition_Expr);
    let mut close_paren_node : Node = create_node(NodeType::Separator);
    let mut open_curly_node : Node = create_node(NodeType::Separator);
    let mut body_node : Node = create_node(NodeType::Body);
    let mut close_curly_node : Node = create_node(NodeType::Separator);

    if 
    parse(&mut keyword_node, tokens, symbol_table) &&
    parse(&mut open_paren_node, tokens, symbol_table) &&
    parse(&mut cond_node, tokens, symbol_table) &&
    parse(&mut close_paren_node, tokens, symbol_table) &&
    parse(&mut open_curly_node, tokens, symbol_table) &&
    parse(&mut body_node, tokens, symbol_table) &&
    parse(&mut close_curly_node, tokens, symbol_table) {

        current_node.children.push(keyword_node);
        current_node.children.push(open_paren_node);
        current_node.children.push(cond_node);
        current_node.children.push(close_paren_node);
        current_node.children.push(open_curly_node);
        current_node.children.push(body_node);
        current_node.children.push(close_curly_node);

        return true;
    }


    return false;
}

fn parse_for_stmt(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    /* This is the structure of a for loop:
    for(init_expr ; expr ; next_expr) {
        body
    }
    init_expr and next_expr can both be empty.
    If expr is left empty, it is assumed to be true.
     */
    let mut keyword_node : Node = create_node(NodeType::Keyword);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut optional_1_node : Node = create_node(NodeType::Optional_Expr);
    let mut semicolon_1_node : Node = create_node(NodeType::Separator);
    let mut optional_2_node : Node = create_node(NodeType::Optional_Expr);
    let mut semicolon_2_node : Node = create_node(NodeType::Separator);
    let mut optional_3_node : Node = create_node(NodeType::Optional_Expr);
    let mut close_paren_node : Node = create_node(NodeType::Separator);
    let mut open_curly_node : Node = create_node(NodeType::Separator);
    let mut body_node : Node = create_node(NodeType::Body);
    let mut close_curly_node : Node = create_node(NodeType::Separator);


    if 
    parse(&mut keyword_node, tokens, symbol_table) &&
    parse(&mut open_paren_node, tokens, symbol_table) &&
    parse(&mut optional_1_node, tokens, symbol_table) &&
    parse(&mut semicolon_1_node, tokens, symbol_table) &&
    parse(&mut optional_2_node, tokens, symbol_table) &&
    parse(&mut semicolon_2_node, tokens, symbol_table) &&
    parse(&mut optional_3_node, tokens, symbol_table) &&
    parse(&mut close_paren_node, tokens, symbol_table) &&
    parse(&mut open_curly_node, tokens, symbol_table) &&
    parse(&mut body_node, tokens, symbol_table) &&
    parse(&mut close_curly_node, tokens, symbol_table) {
        
        current_node.children.push(keyword_node);
        current_node.children.push(open_paren_node);
        current_node.children.push(optional_1_node);
        current_node.children.push(semicolon_1_node);
        current_node.children.push(optional_2_node);
        current_node.children.push(semicolon_2_node);
        current_node.children.push(optional_3_node);
        current_node.children.push(close_paren_node);
        current_node.children.push(open_curly_node);
        current_node.children.push(body_node);
        current_node.children.push(close_curly_node);


    }

    return false;
}

fn parse_optional_expr(current_node : &mut Node, tokens : &Vec<token_c::Token>, symbol_table : &mut STManager) -> bool {

    let mut arith_expr : Node = create_node(NodeType::Arith_Expr);
    let mut conditional_expr : Node = create_node(NodeType::Condition_Expr);

    if parse(&mut arith_expr, tokens, symbol_table) {
        current_node.children.push(arith_expr);
        return true;
    }
    else if parse(&mut conditional_expr, tokens, symbol_table) {
        current_node.children.push(conditional_expr);
        return true;
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        return true;
    }

    return false;
}