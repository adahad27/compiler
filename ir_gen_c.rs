/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::{fs, rc::Rc};

use crate::{parse_c::{Node, NodeType}, token_c::is_identifier};
use crate::symbol_table_c::{*};

static mut CURRENT_TEMP_REG : u128 = 0;


fn get_reg_and_inc() -> u128 {
    unsafe {
        CURRENT_TEMP_REG += 1;
        return CURRENT_TEMP_REG - 1;
    }
}


pub fn generate_ir(filename : &String, current_node : &mut Node, symbol_table : &Rc<STNode>) {
    let mut ir_string : String = "".to_string();

    gen_ir(&mut ir_string, current_node, symbol_table);

    fs::write(filename, ir_string).expect("Unable to write to file");

}


fn gen_ir(ir_string : &mut String, current_node : &mut Node, symbol_table : &Rc<STNode>) {
    match current_node.node_type {
        NodeType::Func_Decl => {
            let func_name : String = current_node.children[1].properties["value"].clone();
            //TODO: Need to mangle return type correctly to match LLVM syntax
            let primitive : String = symbol_table.scope_lookup(&func_name).unwrap().primitive.clone();
            ir_string.push_str(format!("define {} @{}()", to_primitive(primitive), func_name).as_str());

            ir_string.push_str("{\n");

            let current_symbol_table: &Rc<STNode> = &symbol_table.children.borrow()[*symbol_table.scope_index.borrow()];
            generate_children(ir_string, current_node, current_symbol_table);
            *symbol_table.scope_index.borrow_mut() += 1;
            
            ir_string.push_str("}\n");
        }
        NodeType::Arguments => {
            if current_node.properties.contains_key("primtive") {
                ir_string.push_str(format!("{} {}", current_node.properties["primitive"], current_node.properties["identifier"]).as_str());
                if current_node.children[3].children.len() > 0 {
                    //If there are more arguments to print, insert a comma
                    ir_string.push_str(",");
                }
            }
        }
        NodeType::Assign_Expr => {
            let arith_expr : &mut Node = &mut current_node.children[2];
            gen_ir(ir_string, arith_expr, symbol_table);
            let reg_name  = arith_expr.properties["register"].clone();

            let identifier_node : &mut Node = &mut current_node.children[0];
            let identifier : String = identifier_node.properties["value"].clone();

            ir_string.push_str(format!("\t%{} = {}\n", identifier, reg_name).as_str());

            current_node.properties.insert("register".to_string(), reg_name.clone());
            
        }
        NodeType::Arith_Expr => {
            assert!(current_node.properties.contains_key("terminal"));

            //Left operand is a constant
                
            //Move it into a register
            let factor_node : &mut Node = &mut current_node.children[0];
            gen_ir(ir_string, factor_node, symbol_table);
            let reg_name : String = factor_node.properties["register"].clone();

            let mut subterm_node : &mut Node = &mut current_node.children[1];

            subterm_node.properties.insert("prev_register".to_string(), reg_name.clone());

            gen_ir(ir_string, &mut subterm_node, symbol_table);
            let result_reg : String = 
            if current_node.children[1].properties.contains_key("register") {
                current_node.children[1].properties["register"].clone()
            }
            else {
                reg_name
            };
            current_node.properties.insert("register".to_string(), result_reg);

        }
        NodeType::Arith_Subexpr => {
            //Addition and subtraction happen here
            if current_node.properties.contains_key("operator") {
                let operator : String = current_node.properties["operator"].clone();

                /* Generate term node completely, so all multiplication, division, and parenthesis
                are given priority before addition, subtraction */
                let term_node : &mut Node = &mut current_node.children[1];
                gen_ir(ir_string, term_node, symbol_table);

                let result_reg : String = term_node.properties["register"].clone();
                let prev_reg : String = current_node.properties["prev_register"].clone();

                ir_string.push_str(format!("\t{} = {} {} {}\n",prev_reg, to_operator(operator), result_reg, prev_reg).as_str());
                current_node.children[2].properties.insert("prev_register".to_string(), prev_reg.clone());
                current_node.properties.insert("register".to_string(), prev_reg);

                let subexpr_node : &mut Node = &mut current_node.children[2];
                gen_ir(ir_string, subexpr_node, symbol_table);
            }
            else if current_node.properties.contains_key("terminal"){
                let term_node : &mut Node = &mut current_node.children[0];
                gen_ir(ir_string, term_node, symbol_table);

                let result_reg : String = term_node.properties["register"].clone();
                current_node.properties.insert("register".to_string(), result_reg.clone());
            }
        }
        NodeType::Arith_Term => {
            assert!(current_node.properties.contains_key("terminal"));

            //Left operand is a constant
                
            //Move it into a register
            let factor_node : &mut Node = &mut current_node.children[0];
            gen_ir(ir_string, factor_node, symbol_table);
            let reg_name : String = factor_node.properties["register"].clone();

            let mut subterm_node : &mut Node = &mut current_node.children[1];

            subterm_node.properties.insert("prev_register".to_string(), reg_name.clone());

            gen_ir(ir_string, &mut subterm_node, symbol_table);
            let result_reg : String = 
            if current_node.children[1].properties.contains_key("register") {
                current_node.children[1].properties["register"].clone()
            }
            else {
                reg_name
            };
            current_node.properties.insert("register".to_string(), result_reg);
        }
        NodeType::Arith_Subterm => {
            //Multiplication and division happen here
            if current_node.properties.contains_key("operator") {
                let operator : String = current_node.properties["operator"].clone();

                /* Generate term node completely, so all multiplication, division, and parenthesis
                are given priority before addition, subtraction */
                let factor_node : &mut Node = &mut current_node.children[1];
                gen_ir(ir_string, factor_node, symbol_table);

                let result_reg : String = factor_node.properties["register"].clone();
                let prev_reg : String = current_node.properties["prev_register"].clone();

                ir_string.push_str(format!("\t{} = {} {} {}\n",prev_reg, to_operator(operator), result_reg, prev_reg).as_str());
                current_node.children[2].properties.insert("prev_register".to_string(), prev_reg.clone());
                current_node.properties.insert("register".to_string(), prev_reg);

                let subterm_node : &mut Node = &mut current_node.children[2];
                gen_ir(ir_string, subterm_node, symbol_table);
            }
            else if current_node.properties.contains_key("terminal"){
                let factor_node : &mut Node = &mut current_node.children[0];
                gen_ir(ir_string, factor_node, symbol_table);

                let result_reg : String = factor_node.properties["register"].clone();
                current_node.properties.insert("register".to_string(), result_reg.clone());
            }
        }
        NodeType::Arith_Factor => {
            let current_reg : u128 = get_reg_and_inc();
            let mut factor : String = "".to_string();
            let operand : String = current_node.properties["terminal"].clone();
            if is_identifier(&operand) {
                //We have different behavior according to whether or not this factor is a function
                if symbol_table.scope_lookup(&operand).unwrap().func {
                    //TODO: Insert a function call here
                }
                else {
                    factor = format!("%{}", operand);
                }
            }
            else {
                factor = format!("s{}", operand);
            }
            ir_string.push_str(format!("\t%{} = {}\n", current_reg.to_string(), factor).as_str());
            current_node.properties.insert("register".to_string(), format!("%{}", current_reg));

        }
        _ => {
            generate_children(ir_string, current_node, symbol_table);
        }

    }
}

fn generate_children(ir_string : &mut String, current_node : &mut Node, symbol_table : &Rc<STNode>) {
    for mut node in &mut current_node.children {
        gen_ir(ir_string, &mut node, symbol_table);
    }
}

fn to_operator(operation : String) -> String {
    let op = operation.as_str();
    match op {
        "+" => "add".to_string(),
        "-" => "sub".to_string(),
        "*" => "mul".to_string(),
        "/" => "div".to_string(),
        _ => "Error: Invalid operation passed".to_string()
    }
}

fn to_primitive(primitive : String) -> String {
    let op = primitive.as_str();
    match op {
        "int" => "i32".to_string(),
        "bool" => "i1".to_string(),
        _ => "Error: Invalid primitive passed".to_string()
    }
}