/* This file will contain all necessary code to parse all types of expressions */
use crate::{parse_c::{ Node, NodeType, parse, create_node, get_current_token_index, prev_token_index}, token_c::{is_operator, is_separator, Token}};
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
    let identifier_parse : bool = parse(&mut identifier_node, tokens, symbol_table);
    let constant_parse : bool = parse(&mut constant_node, tokens, symbol_table);
    if identifier_parse != constant_parse {
        current_node.children.push(if identifier_parse {identifier_node} else {constant_node});
        current_node.properties.insert("terminal".to_string(), current_node.children[0].properties["value"].clone());

        return true;
    }

    return false;
}

pub fn parse_bool_epxr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

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

pub fn parse_bool_subepxr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

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

pub fn parse_bool_term(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

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

pub fn parse_bool_subterm(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
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

pub fn parse_bool_factor(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

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

pub fn parse_bool_subfactor(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
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

pub fn parse_bool_operand(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
    
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
    else if 
    parse(&mut identifier_node, tokens, symbol_table) {
        if symbol_table.scope_lookup(&identifier_node.properties["value"]).unwrap().primitive == "bool".to_string() {
            current_node.properties.insert("terminal".to_string(), identifier_node.properties["value"].clone());
            current_node.children.push(identifier_node);
            return true;
        }
        prev_token_index();
        return false;
    }
    return false;
}

pub fn parse_relational_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {

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

pub fn parse_cond_expr(current_node : &mut Node, tokens : &Vec<Token>, symbol_table : &Rc<STNode>) -> bool {
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