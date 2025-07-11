/* This file will contain all necessary code to parse all types of expressions */
use crate::{parse_c::{ create_node, get_current_token_index, parse, prev_token_index, Node, NodeType}, token_c::{is_identifier, is_operator, is_separator, Token}};
use crate::symbol_table_c::{*};
use std::rc::Rc;


pub fn parse_arith_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

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

pub fn parse_arith_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
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

pub fn parse_arith_term(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
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

pub fn parse_arith_subterm(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
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

pub fn parse_bool_epxr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut and_expr_node : Node = create_node(NodeType::And_Expr);
    let mut or_subexpr_node : Node = create_node(NodeType::Or_Subexpr);
    
    if
    parse(&mut and_expr_node, tokens, symbol_table) &&
    parse(&mut or_subexpr_node, tokens, symbol_table) {

        current_node.children.push(and_expr_node);

        current_node.children.push(or_subexpr_node);

        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }

        return true;
    }

    return false;
}

pub fn parse_bool_subepxr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    /* 
    Production rules:
    or_subexpr -> [|| and_expr or_subexpr] | empty
     */

    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut and_expr_node : Node = create_node(NodeType::And_Expr);
    let mut or_subexpr_node : Node = create_node(NodeType::Or_Subexpr);
    if
    "||" == tokens[get_current_token_index()].val &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut and_expr_node, tokens, symbol_table) &&
    parse(&mut or_subexpr_node, tokens, symbol_table) {
        //Or case is successful

        current_node.children.push(operator_node);
        current_node.children.push(and_expr_node);
        current_node.children.push(or_subexpr_node);

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

pub fn parse_and_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut equality_expr_node : Node = create_node(NodeType::Equality_Expr);
    let mut and_subexpr_node : Node = create_node(NodeType::And_Subexpr);
    
    if 
    parse(&mut equality_expr_node, tokens, symbol_table) && 
    parse(&mut and_subexpr_node, tokens, symbol_table) {
        
        current_node.children.push(equality_expr_node);
        current_node.children.push(and_subexpr_node);

        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }

        return true;
    }


    return false;
}

pub fn parse_and_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    /* 
    Production rules:
    and_subexpr -> [&& equality_expr and_subexpr] | empty
     */

    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut equality_expr_node : Node = create_node(NodeType::Equality_Expr);
    let mut and_subexpr_node : Node = create_node(NodeType::And_Subexpr);

    if
    "&&" == tokens[get_current_token_index()].val &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut equality_expr_node, tokens, symbol_table) &&
    parse(&mut and_subexpr_node, tokens, symbol_table) {
        //And case is successful

        current_node.children.push(operator_node);
        current_node.children.push(equality_expr_node);
        current_node.children.push(and_subexpr_node);

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

pub fn parse_equality_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    let mut relational_expr_node : Node = create_node(NodeType::Relational_Expr);
    let mut equality_subexpr_node : Node = create_node(NodeType::Equality_Subexpr);

    if 
    parse(&mut relational_expr_node, tokens, symbol_table) && 
    parse(&mut equality_subexpr_node, tokens, symbol_table) {
        
        current_node.children.push(relational_expr_node);
        current_node.children.push(equality_subexpr_node);

        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }

        return true;
    }


    return false;
}

pub fn parse_equality_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    /* 
    Production rules:
    equality_subexpr -> [[== | !=] relational_expr equality_subexpr] | empty
     */

    let mut operator_node : Node = create_node(NodeType::Operator);
    let mut relational_expr_node : Node = create_node(NodeType::Relational_Expr);
    let mut equality_subexpr_node : Node = create_node(NodeType::Equality_Subexpr);

    if
    ("==" == tokens[get_current_token_index()].val ||
    "!=" == tokens[get_current_token_index()].val) &&
    parse(&mut operator_node, tokens, symbol_table) &&
    parse(&mut relational_expr_node, tokens, symbol_table) &&
    parse(&mut equality_subexpr_node, tokens, symbol_table) {
        //Equals/NotEquals case is successful

        current_node.children.push(operator_node);
        current_node.children.push(relational_expr_node);
        current_node.children.push(equality_subexpr_node);

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

pub fn parse_relational_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    
    let mut not_expr_node : Node = create_node(NodeType::Not_Expr);
    let mut relational_subexpr_node : Node = create_node(NodeType::Relational_Subexpr);

    if 
    parse(&mut not_expr_node, tokens, symbol_table) && 
    parse(&mut relational_subexpr_node, tokens, symbol_table) {
        
        current_node.children.push(not_expr_node);
        current_node.children.push(relational_subexpr_node);

        if current_node.children[0].properties.contains_key("terminal") {
            current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["terminal"].clone());
        }

        return true;
    }


    return false;
}

pub fn parse_relational_subexpr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

    return false;
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
        let terminal : String = if
        current_node.children[current_node.children.len() - 1].properties["value"] == "0".to_string() {
            "0".to_string()
        }
        else {
            "1".to_string()
        };
        current_node.properties.insert("terminal".to_string(), terminal);
        
        return true;
    }
    return false;
}
// pub fn parse_relational_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

//     let (mut arith_expr_left, mut arith_expr_right) = (create_node(NodeType::Arith_Expr), create_node(NodeType::Arith_Expr));
//     let mut operator_node : Node = create_node(NodeType::Operator);

//     if
//     parse(&mut arith_expr_left, tokens, symbol_table) &&
//     parse(&mut operator_node, tokens, symbol_table) &&
//     parse(&mut arith_expr_right, tokens, symbol_table) {

//         current_node.children.push(arith_expr_left);
//         current_node.children.push(operator_node);
//         current_node.children.push(arith_expr_right);

//         current_node.properties.insert("operator".to_string(), current_node.children[1].properties["value"].clone());

//         return true;
//     }

//     return false;
// }

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
            expr_node = create_node(NodeType::Condition_Expr);
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