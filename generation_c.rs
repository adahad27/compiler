/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::{fs, rc::Rc};

use crate::{parse_c::{ Node, NodeType}, token_c::is_identifier};
use crate::symbol_table_c::{*};

static mut CURRENT_LABEL_INDEX : u32 = 0;

pub fn generate_code(filename : &String, current_node : &mut Node, symbol_table : &Rc<STNode>) {
    let mut program_string : String = "".to_string();

    generate_start_stub(&mut program_string, symbol_table);

    let mut register_manager = RegisterManager{register_list : Vec::new()};
    register_manager.initialize();

    generate(&mut program_string, current_node, symbol_table, &mut register_manager);

    fs::write(filename, program_string).expect("Unable to write to file");

}

fn generate_start_stub(program_string : &mut String, symbol_table : &Rc<STNode>) {
    
    for (identifier, symbol) in symbol_table.get_table().symbol_table.iter() {
        if symbol.func {
            program_string.push_str(format!("global {}\n", identifier).as_str());
        }
    }
}

fn generate(program_string : &mut String, current_node : &mut Node, symbol_table : &Rc<STNode>, register_manager : &mut RegisterManager) {
    match current_node.node_type {
        NodeType::Func_Decl => {
            program_string.push_str(format!("{}:\n", current_node.children[1].properties["value"]).as_str());
            program_string.push_str(format!("\tpush rbp\n\tmov rbp, rsp\n").as_str());


            //Save arguments to function on stack here


            //Allocate space for all local variables here
            program_string.push_str(format!("\tsub rsp, {}\n", current_node.properties["var_alloc"].parse::<u32>().unwrap() * 8).as_str());

            //Saving all Callee saved registers upon entering a function.
            program_string.push_str("\tpush rbx\n");
            let mut index : u32 = 12;
            while index < 16 {
                program_string.push_str(format!("\tpush r{}\n", index).as_str());
                index += 1;
            }

            //This line is responsible for using the correct child node for the code segment to have it's own scope
            let current_symbol_table: &Rc<STNode> = &symbol_table.children.borrow()[*symbol_table.scope_index.borrow()];
            generate_children(program_string, current_node, current_symbol_table, register_manager);
            //This line updates which children nodes have been used for code generation
            *symbol_table.scope_index.borrow_mut() += 1;
            

            //Restoring all Calle saved registers upon exiting a function.
            index -= 1;
            while index > 11 {
                program_string.push_str(format!("\tpop r{}\n", index).as_str());
                index -= 1;
            }
            program_string.push_str("\tpop rbx\n");

            program_string.push_str(format!("\tmov rsp, rbp\n\tpop rbp\n\tret\n").as_str());
            
        }
        NodeType::Func_Call => {
            /* 
            1.) Store first 6 args in corresponding registers
            2.) Pass excess arguments or arguments that can't fit onto the stack.
            3.) Save all Caller-saved registers
            4.) callq .<FUNCTION_NAME>
            5.) Set properties["register"] = rax
            6.) Restore all Caller-saved registers
            7.) Restore argument registers?
             */

            program_string.push_str(format!("\tcall {}\n", current_node.properties["identifier"]).as_str());
            generate_children(program_string, current_node, symbol_table, register_manager);
            current_node.properties.insert("register".to_string(), "rax".to_string());
        }
        NodeType::Assign_Expr => {
            let arith_expr : &mut Node = &mut current_node.children[2];
            generate(program_string, arith_expr, symbol_table, register_manager);
            let reg_name  = arith_expr.properties["register"].clone();
            

            let identifier : &mut Node = &mut current_node.children[0];

            symbol_table.modify_register(&identifier.properties["value"], register_manager.register_index(&reg_name));
            current_node.properties.insert("register".to_string(), reg_name.clone());

            let offset : i32 = symbol_table.scope_lookup(&current_node.properties["identifier"]).unwrap().addr.clone() as i32;
            
            register_manager.register_free(register_manager.register_index(&reg_name) as u32);

            program_string.push_str(format!("\tmov qword [rbp-{}], {}\n", offset, reg_name).as_str());

        }
        NodeType::Arith_Expr => {

            assert!(current_node.properties.contains_key("terminal"));
            //Left operand is a constant
            
            //Move it into a register
            let term_node : &mut Node = &mut current_node.children[0];
            generate(program_string, term_node, symbol_table, register_manager);
            let reg_name : String = term_node.properties["register"].clone();


            let subexpr_node : &mut Node = &mut current_node.children[1];

            subexpr_node.properties.insert("prev_register".to_string(), reg_name.clone());
            generate(program_string, subexpr_node, symbol_table, register_manager);
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
            if current_node.properties.contains_key("operator") {
                let operator : String = current_node.properties["operator"].clone();

                /* Generate term node completely, so all multiplication, division, and parenthesis
                are given priority before addition, subtraction */
                let term_node : &mut Node = &mut current_node.children[1];
                generate(program_string, term_node, symbol_table, register_manager);
                //The register that stores the result from evaluating term is stored in the result property
                let result_reg : String = term_node.properties["register"].clone();

                program_string.push_str(format!("\t{} {}, {}\n", to_operator(operator), current_node.properties["prev_register"].clone(), result_reg).as_str());

                register_manager.register_free(register_manager.register_index(&result_reg) as u32);

                //Store the results in the next subexpr node, so that it can pick up from where this node left off if needed.
                current_node.children[2].properties.insert("prev_register".to_string(), current_node.properties["prev_register"].clone());
                current_node.properties.insert("register".to_string(), current_node.properties["prev_register"].clone());

                let subexpr_node : &mut Node = &mut current_node.children[2];
                generate(program_string, subexpr_node, symbol_table, register_manager);
            }
            else if current_node.properties.contains_key("terminal") {
                let term_node : &mut Node = &mut current_node.children[0];
                generate(program_string, term_node, symbol_table, register_manager);

                let result_reg : String = term_node.properties["register"].clone();
                current_node.properties.insert("register".to_string(), result_reg.clone());
            }

            //TODO: Add terminal case for when subexprs end
        }
        NodeType::Arith_Term => {
            
            assert!(current_node.properties.contains_key("terminal"));

            //Left operand is a constant
                
            //Move it into a register
            let factor_node : &mut Node = &mut current_node.children[0];
            generate(program_string, factor_node, symbol_table, register_manager);
            let reg_name : String = factor_node.properties["register"].clone();

            let mut subterm_node : &mut Node = &mut current_node.children[1];

            subterm_node.properties.insert("prev_register".to_string(), reg_name.clone());

            generate(program_string, &mut subterm_node, symbol_table, register_manager);
            let result_reg : String = 
            if current_node.children[1].properties.contains_key("register") {
                current_node.children[1].properties["register"].clone()
            }
            else {
                reg_name
            };
            current_node.properties.insert("register".to_string(), result_reg);
            //Free allocated register
        }
        NodeType::Arith_Subterm => {
            if current_node.properties.contains_key("operator") {
                let operator : String = current_node.properties["operator"].clone();

                /* Generate term node completely, so all multiplication, division, and parenthesis
                are given priority before addition, subtraction */
                let factor_node : &mut Node = &mut current_node.children[1];
                generate(program_string, factor_node, symbol_table, register_manager);
                //The register that stores the result from evaluating term is stored in the result property
                let result_reg : String = factor_node.properties["register"].clone();

                /* This operator will always be multiplication or division, so proper assembly needs to be added to 
                facilitate these operations. */

                if operator == "/".to_string() {
                    program_string.push_str("\tmov rdx, 0\n");
                }
                program_string.push_str(format!("\tmov rax, {}\n", current_node.properties["prev_register"].clone()).as_str());
                program_string.push_str(format!("\t{} {}\n", to_operator(operator), result_reg).as_str());
                program_string.push_str(format!("\tmov {}, rax\n", current_node.properties["prev_register"].clone()).as_str());
                //Store the results in the next subexpr node, so that it can pick up from where this node left off if needed.
                current_node.children[2].properties.insert("prev_register".to_string(), current_node.properties["prev_register"].clone());
                current_node.properties.insert("register".to_string(), current_node.properties["prev_register"].clone());
                

                register_manager.register_free(register_manager.register_index(&result_reg) as u32);

                let subterm_node : &mut Node = &mut current_node.children[2];
                generate(program_string, subterm_node, symbol_table, register_manager);
            }
            else if current_node.properties.contains_key("terminal"){
                let factor_node : &mut Node = &mut current_node.children[0];
                generate(program_string, factor_node, symbol_table, register_manager);

                let result_reg : String = factor_node.properties["register"].clone();
                current_node.properties.insert("register".to_string(), result_reg.clone());
            }
            
            //TODO: Add terminal case for when factor_node end
        }
        NodeType::Arith_Factor => {
            let operand : String = current_node.properties["terminal"].clone();
            if is_identifier(&operand) {
                

                let reg_index : u32 = register_manager.register_alloc(0).unwrap();
                let reg_name : String = register_manager.register_name(reg_index);

                let offset : i32 = symbol_table.scope_lookup(&current_node.properties["terminal"]).unwrap().addr.clone() as i32;
                program_string.push_str(format!("\tmov {}, [rbp-{}]\n", reg_name, offset).as_str());
                
                symbol_table.modify_register(&current_node.properties["terminal"], register_manager.register_index(&reg_name.clone()));
                current_node.properties.insert("register".to_string(), reg_name);
            }
            else {
                let reg_index : u32 = register_manager.register_alloc(0).unwrap();
                let reg_name : String = register_manager.register_name(reg_index);
                
                //Move it into a register
                program_string.push_str(format!("\tmov qword {}, {}\n", reg_name, operand).as_str());

                current_node.properties.insert("register".to_string(), reg_name);
            }
        }
        NodeType::Bool_Expr => {
            assert!(current_node.properties.contains_key("terminal"));
            let bool_term_node: &mut Node = &mut current_node.children[0];
            generate(program_string, bool_term_node, symbol_table, register_manager);
            let reg_name : String = bool_term_node.properties["register"].clone();


            let bool_subexpr_node: &mut Node = &mut current_node.children[1];
            bool_subexpr_node.properties.insert("prev_register".to_string(), reg_name.clone());
            generate(program_string, bool_subexpr_node, symbol_table, register_manager);
            
            let result_reg : String = 
            if current_node.children[1].properties.contains_key("register") {
                current_node.children[1].properties["register"].clone()
            }
            else {
                reg_name
            };
            current_node.properties.insert("register".to_string(), result_reg);
        }
        NodeType::Bool_Subexpr => {
            if current_node.properties.contains_key("operator") {
                let operator : String = current_node.properties["operator"].clone();

                let bool_term_node : &mut Node = &mut current_node.children[1];
                generate(program_string, bool_term_node, symbol_table, register_manager);
                let result_reg : String = bool_term_node.properties["register"].clone();

                and_or_generator(program_string, &operator, &current_node.properties["prev_register"], &result_reg);

                register_manager.register_free(register_manager.register_index(&result_reg) as u32);

                current_node.children[2].properties.insert("prev_register".to_string(), current_node.properties["prev_register"].clone());
                current_node.properties.insert("register".to_string(), current_node.properties["prev_register"].clone());

                let bool_subexpr_node : &mut Node = &mut current_node.children[2];
                generate(program_string, bool_subexpr_node, symbol_table, register_manager);
            }
        }
        NodeType::Bool_Term => {
            assert!(current_node.properties.contains_key("terminal"));

            let bool_factor_node: &mut Node = &mut current_node.children[0];
            generate(program_string, bool_factor_node, symbol_table, register_manager);
            let reg_name : String = bool_factor_node.properties["register"].clone();


            let bool_subterm_node: &mut Node = &mut current_node.children[1];
            bool_subterm_node.properties.insert("prev_register".to_string(), reg_name.clone());
            generate(program_string, bool_subterm_node, symbol_table, register_manager);
            
            let result_reg : String = 
            if current_node.children[1].properties.contains_key("register") {
                current_node.children[1].properties["register"].clone()
            }
            else {
                reg_name
            };
            current_node.properties.insert("register".to_string(), result_reg);
        }
        NodeType::Bool_Subterm => {
            if current_node.properties.contains_key("operator") {
                let operator : String = current_node.properties["operator"].clone();

                let bool_factor_node : &mut Node = &mut current_node.children[1];
                generate(program_string, bool_factor_node, symbol_table, register_manager);
                let result_reg : String = bool_factor_node.properties["register"].clone();

                and_or_generator(program_string, &operator, &current_node.properties["prev_register"], &result_reg);

                register_manager.register_free(register_manager.register_index(&result_reg) as u32);

                current_node.children[2].properties.insert("prev_register".to_string(), current_node.properties["prev_register"].clone());
                current_node.properties.insert("register".to_string(), current_node.properties["prev_register"].clone());

                let bool_subterm_node : &mut Node = &mut current_node.children[2];
                generate(program_string, bool_subterm_node, symbol_table, register_manager);
            }
        }
        NodeType::Bool_Factor => {
            assert!(current_node.properties.contains_key("terminal"));
            let bool_operand_node: &mut Node = &mut current_node.children[0];
            generate(program_string, bool_operand_node, symbol_table, register_manager);
            let reg_name : String = bool_operand_node.properties["register"].clone();


            let bool_subfactor_node: &mut Node = &mut current_node.children[1];
            bool_subfactor_node.properties.insert("prev_register".to_string(), reg_name.clone());
            generate(program_string, bool_subfactor_node, symbol_table, register_manager);
            
            let result_reg : String = 
            if current_node.children[1].properties.contains_key("register") {
                current_node.children[1].properties["register"].clone()
            }
            else {
                reg_name
            };
            current_node.properties.insert("register".to_string(), result_reg);
        }
        NodeType::Bool_Subfactor => {
            if current_node.properties.contains_key("operator") {
                let operator : String = current_node.properties["operator"].clone();

                let bool_operand_node : &mut Node = &mut current_node.children[1];
                generate(program_string, bool_operand_node, symbol_table, register_manager);
                let result_reg : String = bool_operand_node.properties["register"].clone();

                equality_generator(program_string, &operator, &current_node.properties["prev_register"], &result_reg);

                register_manager.register_free(register_manager.register_index(&result_reg) as u32);

                current_node.children[2].properties.insert("prev_register".to_string(), current_node.properties["prev_register"].clone());
                current_node.properties.insert("register".to_string(), current_node.properties["prev_register"].clone());

                let bool_subfactor_node : &mut Node = &mut current_node.children[2];
                generate(program_string, bool_subfactor_node, symbol_table, register_manager);
            }
        }
        NodeType::Bool_Operand => {
            let operand : String = current_node.properties["terminal"].clone();
            if is_identifier(&operand) {
                let reg_index : u32 = register_manager.register_alloc(0).unwrap();
                let reg_name : String = register_manager.register_name(reg_index);

                let offset : i32 = symbol_table.scope_lookup(&current_node.properties["terminal"]).unwrap().addr.clone() as i32;
                program_string.push_str(format!("\tmov {}, [rbp-{}]\n", reg_name, offset).as_str());
                
                if current_node.properties.contains_key("unary") {
                    program_string.push_str(format!("\txor {}, 1\n", reg_name).as_str());
                }

                symbol_table.modify_register(&current_node.properties["terminal"], register_manager.register_index(&reg_name.clone()));
                current_node.properties.insert("register".to_string(), reg_name);
            }
            else {
                //Left operand is a constant

                //Allocate register for it
                let reg_index : u32 = register_manager.register_alloc(0).unwrap();
                let reg_name : String = register_manager.register_name(reg_index);
                
                //Move it into a register
                program_string.push_str(format!("\tmov qword {}, {}\n", reg_name, operand).as_str());
                if current_node.properties.contains_key("unary") {
                    program_string.push_str(format!("\txor {}, 1\n", reg_name).as_str());
                }
                current_node.properties.insert("register".to_string(), reg_name);
            }
        }
        NodeType::Relational_Expr => {
            let arith_node_left : &mut Node = &mut current_node.children[0];
            generate(program_string, arith_node_left, symbol_table, register_manager);
            let left_reg : String = arith_node_left.properties["register"].clone();

            let arith_node_right : &mut Node = &mut current_node.children[2];
            generate(program_string, arith_node_right, symbol_table, register_manager);
            let right_reg : String = arith_node_right.properties["register"].clone();

            let operator : String = current_node.properties["operator"].clone();

            let label_true : String = label_name(label_create());
            let label_done : String = label_name(label_create());

            //After doing comparison, the results will be stored in the register named in arith_node_left
            
            program_string.push_str(format!("\tcmp {}, {}\n", left_reg, right_reg).as_str());
            program_string.push_str(format!("\t{} {}\n", jump_command(operator), label_true).as_str());
            program_string.push_str(format!("\tmov {}, 0\n", left_reg).as_str());
            program_string.push_str(format!("\tjmp {}\n", label_done).as_str());
            program_string.push_str(format!("{}:\n", label_true).as_str());
            program_string.push_str(format!("\tmov {}, 1\n", left_reg).as_str());
            program_string.push_str(format!("{}:\n", label_done).as_str());

            current_node.properties.insert("register".to_string(), left_reg);
            
        }
        NodeType::Condition_Expr => {
            let expr_node : &mut Node = &mut current_node.children[0];

            generate(program_string, expr_node, symbol_table, register_manager);

            current_node.properties.insert("register".to_string(), expr_node.properties["register"].clone());

        }
        NodeType::Expression => {
            let expr_node : &mut Node = &mut current_node.children[0];

            generate(program_string, expr_node, symbol_table, register_manager);

            current_node.properties.insert("register".to_string(), expr_node.properties["register"].clone());

        }
        NodeType::If_Stmt => {

            let cond_expr : &mut Node = &mut current_node.children[2];

            //This line is responsible for using the correct child node for the code segment to have it's own scope
            let current_symbol_table: &Rc<STNode> = &symbol_table.children.borrow()[*symbol_table.scope_index.borrow()];
            generate(program_string, cond_expr, current_symbol_table, register_manager);
            let cond_reg : String = cond_expr.properties["register"].clone();

            let end_label : String = label_name(label_create());
            let mut next_label : String = end_label.clone();

            let elif_stmt : &mut Node = &mut current_node.children[7];

            if elif_stmt.children.len() > 0 {
                next_label = label_name(label_create());
            }
            

            program_string.push_str(format!("\tcmp {}, 0\n", cond_reg).as_str());
            program_string.push_str(format!("\tje {}\n", next_label).as_str());

            //Generate code for body and extra statement to allow jumping to end
            let body : &mut Node = &mut current_node.children[5];
            generate(program_string, body, current_symbol_table, register_manager);
            //This line updates which children nodes have been used for code generation
            *symbol_table.scope_index.borrow_mut() += 1;

            program_string.push_str(format!("\tjmp {}\n", end_label).as_str());
            program_string.push_str(format!("{}:\n", next_label).as_str());
            
            let elif_stmt : &mut Node = &mut current_node.children[7];
            //Pass end label to elif and else statements to make sure they can also jump to end
            elif_stmt.properties.insert("end_label".to_string(), end_label.clone());
            //Generate code for elif and else statements
            generate(program_string, elif_stmt, symbol_table, register_manager);

            //Generate the actual end label
            program_string.push_str(format!("{}:\n", end_label).as_str());
        }
        NodeType::Elif_Stmt => {
            if current_node.children.len() == 1 {
                //Then just pass through the generation to the else_stmt node
                let else_stmt : &mut Node = &mut current_node.children[0];
                else_stmt.properties.insert("end_label".to_string(), current_node.properties["end_label"].clone());

                //This line is responsible for using the correct child node for the code segment to have it's own scope
                let current_symbol_table: &Rc<STNode> = &symbol_table.children.borrow()[*symbol_table.scope_index.borrow()];
                generate(program_string, else_stmt, current_symbol_table, register_manager);
                *symbol_table.scope_index.borrow_mut() += 1;
            }
            else if current_node.children.len() > 1{
                let cond_expr : &mut Node = &mut current_node.children[2];

                //This line is responsible for using the correct child node for the code segment to have it's own scope
                let current_symbol_table: &Rc<STNode> = &symbol_table.children.borrow()[*symbol_table.scope_index.borrow()];
                generate(program_string, cond_expr, current_symbol_table, register_manager);
                let cond_reg : String = cond_expr.properties["register"].clone();

                let end_label : String = current_node.properties["end_label"].clone();
                let mut next_label : String = end_label.clone();

                let elif_stmt : &mut Node = &mut current_node.children[7];

                if elif_stmt.children.len() > 0 {
                    next_label = label_name(label_create());
                }
                

                program_string.push_str(format!("\tcmp {}, 0\n", cond_reg).as_str());
                program_string.push_str(format!("\tje {}\n", next_label).as_str());
                
                //Generate code for body and extra statement to allow jumping to end
                let body : &mut Node = &mut current_node.children[5];
                generate(program_string, body, current_symbol_table, register_manager);
                //This line updates which children nodes have been used for code generation
                *symbol_table.scope_index.borrow_mut() += 1;

                program_string.push_str(format!("\tjmp {}\n", end_label).as_str());
                program_string.push_str(format!("{}:\n", next_label).as_str());
                
                let elif_stmt : &mut Node = &mut current_node.children[7];
                //Pass end label to elif and else statements to make sure they can also jump to end
                elif_stmt.properties.insert("end_label".to_string(), end_label.clone());
                //Generate code for elif and else statements
                generate(program_string, elif_stmt, symbol_table, register_manager);
            }
        }
        NodeType::While_Stmt => {
            
            let start_label : String = label_name(label_create());
            let done_label : String = label_name(label_create());

            program_string.push_str(format!("{}:\n", start_label).as_str());

            let cond_expr : &mut Node = &mut current_node.children[2];

            
            //This line is responsible for using the correct child node for the code segment to have it's own scope
            let current_symbol_table: &Rc<STNode> = &symbol_table.children.borrow()[*symbol_table.scope_index.borrow()];
            generate(program_string, cond_expr, current_symbol_table, register_manager);
            let cond_reg : String = cond_expr.properties["register"].clone();

            program_string.push_str(format!("\tcmp {}, 0\n", cond_reg).as_str());
            program_string.push_str(format!("\tje {}\n", done_label).as_str());

            let body_node : &mut Node = &mut current_node.children[5];
            generate(program_string, body_node, current_symbol_table, register_manager);
            //This line updates which children nodes have been used for code generation
            *symbol_table.scope_index.borrow_mut() += 1;

            program_string.push_str(format!("\tjmp {}\n", start_label).as_str());
            program_string.push_str(format!("{}:\n", done_label).as_str());

        }
        NodeType::For_Stmt => {


            let optional_expr_1 : &mut Node = &mut current_node.children[2];

            //This line is responsible for using the correct child node for the code segment to have it's own scope
            let current_symbol_table: &Rc<STNode> = &symbol_table.children.borrow()[*symbol_table.scope_index.borrow()];
            generate(program_string, optional_expr_1, current_symbol_table, register_manager);

            let start_label : String = label_name(label_create());
            let done_label : String = label_name(label_create());

            program_string.push_str(format!("{}:\n", start_label).as_str());

            let optional_expr_2 : &mut Node = &mut current_node.children[4];
            generate(program_string, optional_expr_2, current_symbol_table, register_manager);
            let cond_reg : String = optional_expr_2.children[0].properties["register"].clone();

            program_string.push_str(format!("\tcmp {}, 0\n", cond_reg).as_str());
            program_string.push_str(format!("\tje {}\n", done_label).as_str());

            let body_node : &mut Node = &mut current_node.children[9];
            generate(program_string, body_node, current_symbol_table, register_manager);

            let optional_expr_3 : &mut Node = &mut current_node.children[6];
            generate(program_string, optional_expr_3, current_symbol_table, register_manager);
            //This line updates which children nodes have been used for code generation
            *symbol_table.scope_index.borrow_mut() += 1;

            program_string.push_str(format!("\tjmp {}\n", start_label).as_str());
            program_string.push_str(format!("{}:\n", done_label).as_str());
        }
        // NodeType::VarDecl => {            
        //     program_string.push_str("\tpush 0\n");
        //     generate_children(program_string, current_node, symbol_table, register_manager);
            
            
        // }
        NodeType::Return_Stmt => {
            generate_children(program_string, current_node, symbol_table, register_manager);
            if current_node.children[1].properties.contains_key("terminal") {
                let operand : String = current_node.children[1].properties["terminal"].clone();
                let source : String;
                if is_identifier(&operand) {
                    //Then we have an identifier
                    let register_name : String = register_manager.register_name(symbol_table.scope_lookup(&operand).unwrap().register as u32);
                    register_manager.register_free(register_manager.register_index(&register_name) as u32);
                    source = register_name;
                    
                }
                else {
                    //Then we have an actual number
                    source = operand;
                }
                program_string.push_str(format!("\tmov rax, {}\n", source).as_str());
            }
            
            
        }        
        _ => {
            generate_children(program_string, current_node, symbol_table, register_manager);
        }

    }
}

fn generate_children(program_string : &mut String, current_node : &mut Node, symbol_table : &Rc<STNode>, register_manager : &mut RegisterManager) {
    for mut node in &mut current_node.children {
        generate(program_string, &mut node, symbol_table, register_manager);
    }
}

struct Register {
    name : String,
    in_use : bool,
    addr : u32
}
struct RegisterManager {
    register_list : Vec<Register>
}

impl RegisterManager {

    fn initialize(&mut self) {
        // self.register_list = Vec::new();
        //Hardcoded register names specifically for x86 architecture
        self.register_list.push(Register{name : "rbx".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r10".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r11".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r12".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r13".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r14".to_string(), in_use : false, addr : 0});
        self.register_list.push(Register{name : "r15".to_string(), in_use : false, addr : 0});
    }

    fn register_alloc(&mut self, addr : u32) -> Option<u32> {
        let mut index : usize = 0;

        while index < self.register_list.len() {

            if !self.register_list[index].in_use {
                self.register_list[index].in_use = true;
                self.register_list[index].addr = addr;
                return Option::Some(index as u32);            
            }

            index += 1;
        }
        //Currently throw errors if there are no available registers.
        return Option::None;
    }

    fn register_free(&mut self, reg_index : u32) {
        self.register_list[reg_index as usize].in_use = false;
        self.register_list[reg_index as usize].addr = 0;
    }

    fn register_index(&self, register_name : &String) -> i32 {
        let name = register_name.as_str();
        match name {
            "rbx" => 0,
            "r10" => 1,
            "r11" => 2,
            "r12" => 3,
            "r13" => 4,
            "r14" => 5,
            "r15" => 6,
            _ => -1
        }
    }

    fn register_name(&self, reg_index : u32) -> String {
        return self.register_list[reg_index as usize].name.clone();
    }

}

fn to_operator(operator : String) -> String {
    let op_str = operator.as_str();
    match op_str {
        "+" => "add".to_string(),
        "-" => "sub".to_string(),
        "*" => "imul".to_string(),
        "/" => "idiv".to_string(),
        "&&" => "and".to_string(),
        "||" => "or".to_string(),
        "==" => "cmp".to_string(),
        "!=" => "cmp".to_string(),

        _ => "Error: Incorrect operator found".to_string()
    }
}

fn jump_command(operator : String) -> String {
    let op_str = operator.as_str();
    match op_str {
        "<" => "jl".to_string(),
        "<=" => "jle".to_string(),
        ">" => "jg".to_string(),
        ">=" => "jge".to_string(),

        _ => "Error: Incorrect operator found".to_string()
    }
}


fn label_create() -> u32 {
    unsafe {
        CURRENT_LABEL_INDEX += 1;
        return CURRENT_LABEL_INDEX;
    }
}

fn label_name(index : u32)->String {
    return format!(".L{}", index);
}

fn and_or_generator(program_string : &mut String, operator : &String, prev_reg : &String, next_reg : &String) {
    
    let label_true: String = label_name(label_create());
    let label_done : String = label_name(label_create());

    let short_circuit_op : String;
    let full_eval_op : String;
    if operator == "||" {
        short_circuit_op = "1".to_string();
        full_eval_op = "0".to_string();
    }
    else {
        short_circuit_op = "0".to_string();
        full_eval_op = "1".to_string();
    }
    
    program_string.push_str(format!("\tcmp {}, {}\n", prev_reg, short_circuit_op).as_str());    
    program_string.push_str(format!("\tje {}\n", label_true.clone()).as_str());
    program_string.push_str(format!("\tcmp {}, {}\n", next_reg, short_circuit_op).as_str());
    program_string.push_str(format!("\tje {}\n", label_true.clone()).as_str());
    program_string.push_str(format!("\tmov {}, {}\n", prev_reg, full_eval_op).as_str());
    program_string.push_str(format!("\tjmp {}\n", label_done.clone()).as_str());
    program_string.push_str(format!("{}:\n", label_true).as_str());
    program_string.push_str(format!("\tmov {}, {}\n", prev_reg, short_circuit_op).as_str());
    program_string.push_str(format!("{}:\n", label_done).as_str());
}

fn equality_generator(program_string : &mut String, operator : &String, prev_reg : &String, next_reg : &String) {
    let label_equal: String = label_name(label_create());
    let label_done : String = label_name(label_create());

    let short_circuit_op : String;
    let full_eval_op : String;
    if operator == "!=" {
        short_circuit_op = "1".to_string();
        full_eval_op = "0".to_string();
    }
    else {
        short_circuit_op = "0".to_string();
        full_eval_op = "1".to_string();
    }

    program_string.push_str(format!("\tcmp {}, {}\n", prev_reg, next_reg).as_str());
    program_string.push_str(format!("\tje {}\n", label_equal.clone()).as_str());
    program_string.push_str(format!("\tmov {}, {}\n", prev_reg, short_circuit_op).as_str());
    program_string.push_str(format!("\tjmp {}\n", label_done.clone()).as_str());
    program_string.push_str(format!("{}:\n", label_equal).as_str());
    program_string.push_str(format!("\tmov {}, {}\n", prev_reg, full_eval_op).as_str());
    program_string.push_str(format!("{}:\n", label_done).as_str());
}