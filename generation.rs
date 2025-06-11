/* 
This is the file that will be responsible for assembly code generation. For now
it will combine both the intermediate and target code generation into one unit of
logic
*/

use std::fs;

pub fn generate_code(filename : &String) {
    let mut program_string : String = "".to_string();

    generate_start_stub(&mut program_string);

    fs::write(filename, program_string).expect("Unable to write to file");

}

fn generate_start_stub(program_string : &mut String) {
    program_string.push_str("global _start\n_start:\n");
}