/* This file will contain all necessary code to parse all types of expressions */
use crate::{parse_c::{ create_node, get_current_token_index, parse, prev_token_index, Node, NodeType}, token_c::{is_identifier, is_operator, is_separator, Token}};
use crate::symbol_table_c::{*};
use std::rc::Rc;

fn parse_non_terminal_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>, expr_type : NodeType, subexpr_type : NodeType) -> bool {
    let mut expr_node : Node = create_node(expr_type);
    let mut subexpr_node : Node = create_node(subexpr_type);
    
    if
    parse(&mut expr_node, tokens, symbol_table) &&
    parse(&mut subexpr_node, tokens, symbol_table) {

        current_node.children.push(expr_node);
        current_node.children.push(subexpr_node);

        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }
        return true;
    }

    return false;
}

fn semantic_check(tokens : &Vec<Token>, semantic_requirements : &Vec<String>) -> bool {
    for requirement  in semantic_requirements {
        if &tokens[get_current_token_index()].val == requirement {
            return true;
        }
    }
    return false;
}

fn parse_non_terminal_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>, expr_type : NodeType, subexpr_type : NodeType, semantic_requirements : &Vec<String>) -> bool {
    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut expr_node : Node = create_node(expr_type);
    let mut subexpr_node : Node = create_node(subexpr_type);

    if
    semantic_check(tokens, semantic_requirements) &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut expr_node, tokens, symbol_table) &&
    parse(&mut subexpr_node, tokens, symbol_table) {

        current_node.children.push(operator_node);
        current_node.children.push(expr_node);
        current_node.children.push(subexpr_node);

        current_node.properties.insert("operator".to_string(), current_node.children[0].properties["value"].clone());
        return true;
    }
    else if parse(&mut expr_node, tokens, symbol_table) {

        current_node.children.push(expr_node);
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
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

pub fn parse_arith_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    return parse_non_terminal_expr(current_node, tokens, symbol_table, NodeType::Arith_Term, NodeType::Arith_Subexpr);
}

pub fn parse_arith_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut semantic_requirements : Vec<String> = Vec::new();
    semantic_requirements.push("+".to_string());
    semantic_requirements.push("-".to_string());

    return parse_non_terminal_subexpr(current_node, tokens, symbol_table, NodeType::Arith_Term, NodeType::Arith_Subexpr, &semantic_requirements);
}

pub fn parse_arith_term(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    return parse_non_terminal_expr(current_node, tokens, symbol_table, NodeType::Arith_Factor, NodeType::Arith_Subterm);
}

pub fn parse_arith_subterm(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut semantic_requirements : Vec<String> = Vec::new();
    semantic_requirements.push("*".to_string());
    semantic_requirements.push("/".to_string());

    return parse_non_terminal_subexpr(current_node, tokens, symbol_table, NodeType::Arith_Factor, NodeType::Arith_Subterm, &semantic_requirements);
}

pub fn parse_arith_factor(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    /* 
    Production rules:
    Arith_Factor -> constant | identifier | (expression)
     */
    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut constant_node : Node = create_node(NodeType::Constant);
    let mut func_call_node : Node = create_node(NodeType::Func_Call);

    if parse(&mut constant_node, tokens, symbol_table) {
        current_node.properties.insert("terminal".to_string(), constant_node.properties["value"].clone());
        current_node.children.push(constant_node);
        return true;
    }
    else if is_identifier(&tokens[get_current_token_index()].val) {
        if
        tokens[get_current_token_index() + 1].val == "(" && 
        parse(&mut func_call_node, tokens, symbol_table) &&
        symbol_table.scope_lookup(&func_call_node.properties["identifier"]).unwrap().primitive == "int".to_string() {
            current_node.properties.insert("terminal".to_string(), func_call_node.properties["identifier"].clone());
            current_node.children.push(func_call_node);
            return true;

        }
        else if 
        parse(&mut identifier_node, tokens, symbol_table) &&
        (symbol_table.scope_lookup(&identifier_node.properties["value"]).unwrap().primitive == "int".to_string() ||
        symbol_table.scope_lookup(&identifier_node.properties["value"]).unwrap().primitive == "bool".to_string()) {
            current_node.properties.insert("terminal".to_string(), identifier_node.properties["value"].clone());
            current_node.children.push(identifier_node);
            return true;
        }
        prev_token_index();
        return false;

        
    }
    return false;
}

pub fn parse_or_epxr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    return parse_non_terminal_expr(current_node, tokens, symbol_table, NodeType::And_Expr, NodeType::Or_Subexpr);
}

pub fn parse_or_subepxr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut semantic_requirements : Vec<String> = Vec::new();
    semantic_requirements.push("||".to_string());

    return parse_non_terminal_subexpr(current_node, tokens, symbol_table, NodeType::And_Expr, NodeType::Or_Subexpr, &semantic_requirements);
}

pub fn parse_and_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    return parse_non_terminal_expr(current_node, tokens, symbol_table, NodeType::Equality_Expr, NodeType::And_Subexpr);
}

pub fn parse_and_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut semantic_requirements : Vec<String> = Vec::new();
    semantic_requirements.push("&&".to_string());

    return parse_non_terminal_subexpr(current_node, tokens, symbol_table, NodeType::Equality_Expr, NodeType::And_Subexpr, &semantic_requirements);

}

pub fn parse_equality_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    return parse_non_terminal_expr(current_node, tokens, symbol_table, NodeType::Relational_Expr, NodeType::Equality_Subexpr);
}

pub fn parse_equality_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut semantic_requirements : Vec<String> = Vec::new();
    semantic_requirements.push("==".to_string());
    semantic_requirements.push("!=".to_string());

    return parse_non_terminal_subexpr(current_node, tokens, symbol_table, NodeType::Relational_Expr, NodeType::Equality_Subexpr, &semantic_requirements);

}

pub fn parse_relational_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {   
    return parse_non_terminal_expr(current_node, tokens, symbol_table, NodeType::Not_Expr, NodeType::Relational_Subexpr);
}

pub fn parse_relational_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut semantic_requirements : Vec<String> = Vec::new();
    semantic_requirements.push("<".to_string());
    semantic_requirements.push(">".to_string());
    semantic_requirements.push("<=".to_string());
    semantic_requirements.push(">=".to_string());

    return parse_non_terminal_subexpr(current_node, tokens, symbol_table, NodeType::Not_Expr, NodeType::Relational_Subexpr, &semantic_requirements);
}

pub fn parse_not_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut keyword_node : Node = create_node(NodeType::Keyword);
    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut func_call_node : Node = create_node(NodeType::Func_Call);
    let mut constant_node : Node = create_node(NodeType::Constant);

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

    else if
    is_identifier(&tokens[get_current_token_index()].val) {
        if
        tokens[get_current_token_index() + 1].val == "(" && 
        parse(&mut func_call_node, tokens, symbol_table) &&
        (symbol_table.scope_lookup(&func_call_node.properties["identifier"]).unwrap().primitive == "bool".to_string() ||
        symbol_table.scope_lookup(&func_call_node.properties["identifier"]).unwrap().primitive == "int".to_string()) {
            current_node.properties.insert("terminal".to_string(), func_call_node.properties["identifier"].clone());
            current_node.children.push(func_call_node);
            return true;

        }
        else if 
        parse(&mut identifier_node, tokens, symbol_table) &&
        (symbol_table.scope_lookup(&identifier_node.properties["value"]).unwrap().primitive == "bool".to_string() ||
        symbol_table.scope_lookup(&identifier_node.properties["value"]).unwrap().primitive == "int".to_string()){
            current_node.properties.insert("terminal".to_string(), identifier_node.properties["value"].clone());
            current_node.children.push(identifier_node);
            return true;
        }
        prev_token_index();
        return false;
    }
    else if
    parse(&mut constant_node, tokens, symbol_table) {
        current_node.children.push(constant_node);
        current_node.properties.insert("terminal".to_string(), current_node.children[current_node.children.len() - 1].properties["value"].clone());
        
        return true;
    }
    return false;
}

pub fn parse_cond_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    let mut or_expr_node : Node = create_node(NodeType::Or_Expr);
    let mut rel_expr_node : Node = create_node(NodeType::Relational_Expr);
    if parse(&mut rel_expr_node, tokens, symbol_table) {
        //We have a relational expression
        current_node.children.push(rel_expr_node);
        return true;
    }
    if parse(&mut or_expr_node, tokens, symbol_table) {
        //We have a boolean expression
        current_node.children.push(or_expr_node);
        return true;
    }


    return false;
}


pub fn parse_optional_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut expr: Node = create_node(NodeType::Expression);
    let mut assign_expr : Node = create_node(NodeType::Assign_Expr);
    let mut var_decl : Node = create_node(NodeType::VarDecl);

    if parse(&mut var_decl, tokens, symbol_table) {
        current_node.children.push(var_decl);
        return true;
    }
    else if parse(&mut assign_expr, tokens, symbol_table) {
        current_node.children.push(assign_expr);
        return true;
    }
    else if parse(&mut expr, tokens, symbol_table) {
        current_node.children.push(expr);
        return true;
    }
    else if is_separator(&tokens[get_current_token_index()].val) {
        return true;
    }

    return false;
}

pub fn parse_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut arith_expr : Node = create_node(NodeType::Arith_Expr);
    let mut conditional_expr : Node = create_node(NodeType::Condition_Expr);
    
    if parse(&mut conditional_expr, tokens, symbol_table) {
        current_node.children.push(conditional_expr);
        return true;
    }
    else if parse(&mut arith_expr, tokens, symbol_table) {
        current_node.children.push(arith_expr);
        return true;
    }
    

    return false;
}

pub fn parse_assign_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    let mut expr_node : Node = create_node(NodeType::Arith_Expr);
    let mut identity_node : Node = create_node(NodeType::Identifier);
    let mut operator_node : Node = create_node(NodeType::Operator);

    
    if parse(&mut identity_node, tokens, symbol_table) {

        if symbol_table.scope_lookup(&identity_node.properties["value"]).unwrap().primitive == "int".to_string() {
            expr_node = create_node(NodeType::Arith_Expr);
        }
        else if symbol_table.scope_lookup(&identity_node.properties["value"]).unwrap().primitive == "bool".to_string() {
            expr_node = create_node(NodeType::Or_Expr);
        }

        if
        tokens[get_current_token_index()].val == "=".to_string() &&
        parse(&mut operator_node, tokens, symbol_table) &&
        parse(&mut expr_node, tokens, symbol_table) {
            current_node.children.push(identity_node);
            current_node.children.push(operator_node);
            current_node.children.push(expr_node);

            current_node.properties.insert("identifier".to_string(), current_node.children[0].properties["value"].clone());

            return true;
        }
        else {
            prev_token_index();
            return false;
        }
    }
    return false;
}

pub fn parse_func_call(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut identifier_node : Node = create_node(NodeType::Identifier);
    let mut open_paren_node : Node = create_node(NodeType::Separator);
    let mut arguments_node : Node = create_node(NodeType::Call_Args);
    let mut close_paren_node : Node = create_node(NodeType::Separator);

    if
    parse(&mut identifier_node, tokens, symbol_table) &&
    parse(&mut open_paren_node, tokens, symbol_table) &&
    parse(&mut arguments_node, tokens, symbol_table) &&
    parse(&mut close_paren_node, tokens, symbol_table) {
        
        current_node.properties.insert("arguments".to_string(), arguments_node.properties["arguments"].clone());

        current_node.children.push(identifier_node);
        current_node.children.push(open_paren_node);
        current_node.children.push(arguments_node);
        current_node.children.push(close_paren_node);
        
        current_node.properties.insert("identifier".to_string(), current_node.children[0].properties["value"].clone());
        return true;

    }


    return false;
}

pub fn parse_call_args(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    let mut expr_node : Node = create_node(NodeType::Expression);
    let mut separator_node : Node = create_node(NodeType::Separator);
    let mut call_arg_node : Node = create_node(NodeType::Call_Args);

    if tokens[get_current_token_index()].val == ")".to_string() {
        current_node.properties.insert("arguments".to_string(), "0".to_string());
        return true;
    }

    if
    parse(&mut expr_node, tokens, symbol_table) {

        current_node.children.push(expr_node);
        if tokens[get_current_token_index()].val == ")".to_string() {
            current_node.properties.insert("arguments".to_string(), "1".to_string());
            return true;
        }
        if 
        parse(&mut separator_node, tokens, symbol_table) &&
        parse(&mut call_arg_node, tokens, symbol_table) {

            let arg_num: i32 = call_arg_node.properties["arguments"].clone().parse::<i32>().unwrap() + 1;
            current_node.properties.insert("arguments".to_string(), arg_num.to_string());

            current_node.children.push(separator_node);
            current_node.children.push(call_arg_node);

            return true;
        }

        
        

        return true;
    }

    return false;
}