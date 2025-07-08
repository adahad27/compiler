/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::{fs, rc::Rc};

use crate::{parse_c::{ Node, NodeType}, token_c::is_identifier};
use crate::symbol_table_c::{*};

static mut CURRENT_LABEL_INDEX : u32 = 0;

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
            ir_string.push_str(format!("define {} @{}()", symbol_table.scope_lookup(&func_name).unwrap().primitive, func_name).as_str());

            ir_string.push_str("{\n");

            generate_children(ir_string, current_node, symbol_table);
            
            ir_string.push_str("}\n");
        }
        NodeType::Func_Call => {

        }
        NodeType::Assign_Expr => {

        }
        NodeType::Arith_Expr => {

        }
        NodeType::Arith_Subexpr => {

        }
        NodeType::Arith_Term => {
            
        }
        NodeType::Arith_Subterm => {
            
        }
        NodeType::Arith_Factor => {
            
        }
        NodeType::Bool_Expr => {
            
        }
        NodeType::Bool_Subexpr => {
            
        }
        NodeType::Bool_Term => {
            
        }
        NodeType::Bool_Subterm => {
            
        }
        NodeType::Bool_Factor => {
            
        }
        NodeType::Bool_Subfactor => {
            
        }
        NodeType::Bool_Operand => {
            
        }
        NodeType::Relational_Expr => {
            
        }
        NodeType::Condition_Expr => {
            

        }
        NodeType::Expression => {
            

        }
        NodeType::If_Stmt => {

            
        }
        NodeType::Elif_Stmt => {
            
        }
        NodeType::While_Stmt => {

        }
        NodeType::For_Stmt => {

        }
        NodeType::Return_Stmt => {
            
            
            
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
