
use std::collections::HashMap;

pub struct Symbol {
    pub primitive : String, 
    pub addr : u32,
    pub size : u32,
    pub register : i32,
    pub func : bool
}

pub struct SymbolTable {
    pub symbol_table : HashMap<String, Symbol>,
    pub ordinal : u32
}


impl SymbolTable {
    /* Handles updating address for each local variable from base of stack frame
    for easier assembly generation */
    pub fn insert(&mut self, identifier : &String, prim : &String, func : bool) {
        //Construct symbol
        self.symbol_table.insert(identifier.clone(), Symbol{primitive : prim.clone(), addr : self.ordinal * 8, size : 8, register : -1, func : func});

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
            register: register,
            func : self.symbol_table[identifier].func
        });
    }

}
/*
TODO: Will need to fix scoping issues by implementing a doubly linked
spaghetti stack (Parent Pointer Tree). For now all scoping will be shared
globally.
 */
pub struct STManager {
    pub stack : Vec<SymbolTable>
}

impl STManager {
    pub fn scope_enter(&mut self, new_frame : bool) {
        let mut new_st : SymbolTable;
        if new_frame {
            new_st = SymbolTable{symbol_table : HashMap::new(), ordinal : 1};
        }
        else {
            assert!(self.stack.len() > 0);
            new_st = SymbolTable{symbol_table : HashMap::new(), ordinal : self.stack[self.stack.len() - 1].ordinal}
        }
        self.stack.push(new_st);
    }

    pub fn scope_exit(&mut self) {
        self.stack.pop();
    }

    pub fn scope_level(&self) -> u32{
        return self.stack.len() as u32;
    }

    pub fn scope_bind(&mut self, identifier : &String, prim : &String, func : bool) {
        let index : usize = self.stack.len() - 1;
        self.stack[index].insert(identifier, prim, func);
    }

    pub fn scope_lookup(&self, identifier :&String) -> Option<&Symbol>{
        let mut index : usize = self.stack.len() - 1;
        while index > 0 {
            if let Option::Some(symbol) = self.stack[index].query(identifier) {
                return Option::Some(symbol);
            }
            index -= 1;
        }
        return Option::None;
    }

    pub fn scope_lookup_current(&self, identifier : &String) -> Option<&Symbol> {
        let index : usize = self.stack.len() - 1;
        if let Option::Some(symbol) = self.stack[index].query(identifier) {
            return Option::Some(symbol);
        }
        return Option::None;
    }
}