/* This file will contain the code necessary to parse statements or 
bodies of statements */
use std::rc::Rc;
use crate::{parse_c::{ create_node, get_current_token_index,  parse, Node, NodeType}, token_c::{is_identifier, is_primitive, Token}};
use crate::symbol_table_c::{*};

pub fn parse_statement(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) ->bool {
    /* Include all rules for CFGs that have statements on the LHS here. */
    if tokens[get_current_token_index()].val == "return".to_string() {
        let mut return_node : Node = create_node(NodeType::Return_Stmt);

        if parse(&mut return_node, tokens, symbol_table) 
        {
            current_node.children.push(return_node);
            return true;
        }
    }
    else if 
    is_primitive(&tokens[get_current_token_index()].val) {
        //Then we have found a variable declaration
        let mut var_decl : Node = create_node(NodeType::VarDecl);
        let mut semicolon_node : Node = create_node(NodeType::Separator);

        if 
        parse(&mut var_decl, tokens, symbol_table) &&
        parse(&mut semicolon_node, tokens, symbol_table)
        {
            current_node.children.push(var_decl);
            current_node.properties.insert("var_alloc".to_string(), "1".to_string());
            return true;
        }
        
    }
    else if 
    is_identifier(&tokens[get_current_token_index()].val) {
        /* 
        Then we have either found an assignment expression OR we have found a 
        function call.
        */
        let mut assign_expr : Node = create_node(NodeType::Assign_Expr);
        let mut func_call_node : Node = create_node(NodeType::Func_Call);
        let mut semicolon_node : Node = create_node(NodeType::Separator);

        if
        tokens[get_current_token_index() + 1].val == "(" &&
        parse(&mut func_call_node, tokens, symbol_table) &&
        parse(&mut semicolon_node, tokens, symbol_table) {
            //Then this must be a function call
            current_node.children.push(func_call_node);
            current_node.children.push(semicolon_node);
            return true;
        }
        else if parse(&mut assign_expr, tokens, symbol_table) &&
        parse(&mut semicolon_node, tokens, symbol_table){
            //Otherwise this must be an assignment
            current_node.children.push(assign_expr);
            current_node.children.push(semicolon_node);
            return true;
        }

        return false;
        
    }
    else if tokens[get_current_token_index()].val == "if".to_string() {
        let mut if_stmt : Node = create_node(NodeType::If_Stmt);
        if parse(&mut if_stmt, tokens, symbol_table) 
        {
            current_node.children.push(if_stmt);
            return true;
        }
    }
    else if tokens[get_current_token_index()].val == "for".to_string() {
        let mut for_stmt : Node = create_node(NodeType::For_Stmt);
        if parse(&mut for_stmt, tokens, symbol_table) 
        {
            current_node.children.push(for_stmt);
            return true;
        }
    }
    else if tokens[get_current_token_index()].val == "while".to_string() {
        let mut while_stmt : Node = create_node(NodeType::While_Stmt);
        if parse(&mut while_stmt, tokens, symbol_table) 
        {
            current_node.children.push(while_stmt);
            return true;
        }
    }
    return false;
}

pub fn parse_var_decl(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) ->bool {
    let mut primitive_node : Node = create_node(NodeType::Primitive);
    let mut identifier_node : Node = create_node(NodeType::Identifier);
    
    

    if parse(&mut primitive_node, tokens, symbol_table) {
        
        let mut expr_node : Node = create_node(NodeType::Assign_Expr);
        current_node.children.push(primitive_node);

        if is_identifier(&tokens[get_current_token_index()].val) {
            symbol_table.bind(&tokens[get_current_token_index()].val, &current_node.children[0].properties["value"], 0,false);
        }

        if parse(&mut expr_node, tokens, symbol_table) {
            current_node.children.push(expr_node);

            current_node.properties.insert("identifier".to_string(), current_node.children[1].properties["identifier"].clone());
            return true;

        }
        else if parse(&mut identifier_node, tokens, symbol_table) {
            current_node.children.push(identifier_node);

            current_node.properties.insert("value".to_string(), "0".to_string());
            current_node.properties.insert("identifier".to_string(), current_node.children[1].properties["value"].clone());

            let mut s_bracket_open_node : Node = create_node(NodeType::Separator);
            let mut constant_node : Node = create_node(NodeType::Constant);
            let mut s_bracket_close_node : Node = create_node(NodeType::Separator);

            if 
            tokens[get_current_token_index()].val == "[" &&
            parse(&mut s_bracket_open_node, tokens, symbol_table) &&
            parse(&mut constant_node, tokens, symbol_table) &&
            parse(&mut s_bracket_close_node, tokens, symbol_table){
            
                current_node.children.push(s_bracket_open_node);
                current_node.children.push(constant_node);
                current_node.children.push(s_bracket_close_node);

            }

            return true;

        }
        else{
            
            return false;
        }

    }
    return false;
}

pub fn parse_ret_stmt(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) ->bool {
    let mut return_node : Node = create_node(NodeType::Keyword);
    let mut expr_node : Node = create_node(NodeType::Arith_Expr);
    let mut semicolon_node : Node = create_node(NodeType::Separator);



    if
    parse(&mut return_node, tokens, symbol_table) &&
    parse(&mut expr_node, tokens, symbol_table) &&
    parse(&mut semicolon_node, tokens, symbol_table) {
        
        current_node.children.push(return_node);
        current_node.children.push(expr_node);
        current_node.children.push(semicolon_node);

        return true;
    }

    return false;

    
   
}

pub fn parse_body(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) ->bool {
    let mut var_alloc : u32 = 0;
    while tokens[get_current_token_index()].val != "}".to_string(){
        let mut stmt_node : Node = create_node(NodeType::Statement);

        if current_node.properties.contains_key("return_type") {
            stmt_node.properties.insert("return_type".to_string(), current_node.properties["return_type"].clone());
        }
        
        if parse(&mut stmt_node, tokens, symbol_table) {
            if stmt_node.properties.contains_key("var_alloc") {
                var_alloc += 1;
            }
            current_node.children.push(stmt_node);
        }
        else {
            return false;
        }

    }
    current_node.properties.insert("var_alloc".to_string(), var_alloc.to_string());
    return true;
}



pub fn parse_if_stmt(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    let mut elif_stmt_node : Node = create_node(NodeType::Elif_Stmt);

    if 
    handle_if_block(current_node, tokens, symbol_table) &&
    parse(&mut elif_stmt_node, tokens, symbol_table) {

        current_node.children.push(elif_stmt_node);
        return true;

    }

    return false;
}

fn handle_if_block(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    symbol_table.push_child(symbol_table.get_ordinal());
    let current_table = &symbol_table.children.borrow()[symbol_table.children.borrow().len() - 1];
    let mut keyword_node : Node = create_node(NodeType::Keyword);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut cond_node : Node = create_node(NodeType::Condition_Expr);
    let mut close_paren_node : Node = create_node(NodeType::Separator);
    let mut open_curly_node : Node = create_node(NodeType::Separator);
    let mut body_node : Node = create_node(NodeType::Body);
    let mut close_curly_node : Node = create_node(NodeType::Separator);
    if 
    parse(&mut keyword_node, tokens, current_table) &&
    parse(&mut open_paren_node, tokens, current_table) &&
    parse(&mut cond_node, tokens, current_table) &&
    parse(&mut close_paren_node, tokens, current_table) &&
    parse(&mut open_curly_node, tokens, current_table) &&
    parse(&mut body_node, tokens, current_table) &&
    parse(&mut close_curly_node, tokens, current_table) {

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

pub fn parse_elif_stmt(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    //New scope made here
    if tokens[get_current_token_index()].val == "else".to_string() {
        let mut else_stmt_node : Node = create_node(NodeType::Else_Stmt);

        if parse(&mut else_stmt_node, tokens, symbol_table) {
            current_node.children.push(else_stmt_node);
            return true;
        }
        return false;
    }

    


    if tokens[get_current_token_index()].val == "elif".to_string() {
        
        let mut elif_stmt_node : Node = create_node(NodeType::Elif_Stmt);
        
        if 
        handle_elif_block(current_node, tokens, symbol_table) &&
        parse(&mut elif_stmt_node, tokens, symbol_table) {

            
            current_node.children.push(elif_stmt_node);

            return true;
        }
        return false;
    }
    

    return true;
}

fn handle_elif_block(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    let mut keyword_node : Node = create_node(NodeType::Keyword);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut cond_node : Node = create_node(NodeType::Condition_Expr);
    let mut close_paren_node : Node = create_node(NodeType::Separator);
    let mut open_curly_node : Node = create_node(NodeType::Separator);
    let mut body_node : Node = create_node(NodeType::Body);
    let mut close_curly_node : Node = create_node(NodeType::Separator);

    symbol_table.push_child(symbol_table.get_ordinal());
    let current_table = &symbol_table.children.borrow()[symbol_table.children.borrow().len() - 1];

    if
    parse(&mut keyword_node, tokens, current_table) &&
    parse(&mut open_paren_node, tokens, current_table) &&
    parse(&mut cond_node, tokens, current_table) &&
    parse(&mut close_paren_node, tokens, current_table) &&
    parse(&mut open_curly_node, tokens, current_table) &&
    parse(&mut body_node, tokens, current_table) &&
    parse(&mut close_curly_node, tokens, current_table) {
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

pub fn parse_else_stmt(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    //New scope made here
    symbol_table.push_child(symbol_table.get_ordinal());
    let current_table = &symbol_table.children.borrow()[symbol_table.children.borrow().len() - 1];

    if tokens[get_current_token_index()].val == "else".to_string() {
        let mut keyword_node : Node = create_node(NodeType::Keyword);
        let mut open_curly_node : Node = create_node(NodeType::Separator);
        let mut body_node : Node = create_node(NodeType::Body);
        let mut close_curly_node : Node = create_node(NodeType::Separator);

        if 
        parse(&mut keyword_node, tokens, current_table) &&
        parse(&mut open_curly_node, tokens, current_table) &&
        parse(&mut body_node, tokens, current_table) &&
        parse(&mut close_curly_node, tokens, current_table) {


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

pub fn parse_while_stmt(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    //New scope made here
    
    symbol_table.push_child(symbol_table.get_ordinal());
    let current_table = &symbol_table.children.borrow()[symbol_table.children.borrow().len() - 1];

    let mut keyword_node : Node = create_node(NodeType::Keyword);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut cond_node : Node = create_node(NodeType::Condition_Expr);
    let mut close_paren_node : Node = create_node(NodeType::Separator);
    let mut open_curly_node : Node = create_node(NodeType::Separator);
    let mut body_node : Node = create_node(NodeType::Body);
    let mut close_curly_node : Node = create_node(NodeType::Separator);

    if 
    parse(&mut keyword_node, tokens, current_table) &&
    parse(&mut open_paren_node, tokens, current_table) &&
    parse(&mut cond_node, tokens, current_table) &&
    parse(&mut close_paren_node, tokens, current_table) &&
    parse(&mut open_curly_node, tokens, current_table) &&
    parse(&mut body_node, tokens, current_table) &&
    parse(&mut close_curly_node, tokens, current_table) {

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

pub fn parse_for_stmt(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    symbol_table.push_child(symbol_table.get_ordinal());
    let current_table = &symbol_table.children.borrow()[symbol_table.children.borrow().len() - 1];

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
    parse(&mut keyword_node, tokens, current_table) &&
    parse(&mut open_paren_node, tokens, current_table) &&
    parse(&mut optional_1_node, tokens, current_table) &&
    parse(&mut semicolon_1_node, tokens, current_table) &&
    parse(&mut optional_2_node, tokens, current_table) &&
    parse(&mut semicolon_2_node, tokens, current_table) &&
    parse(&mut optional_3_node, tokens, current_table) &&
    parse(&mut close_paren_node, tokens, current_table) &&
    parse(&mut open_curly_node, tokens, current_table) &&
    parse(&mut body_node, tokens, current_table) &&
    parse(&mut close_curly_node, tokens, current_table) {
        
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

        return true;
    }

    return false;
}
