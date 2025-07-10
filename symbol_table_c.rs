
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut};

#[derive(Clone)]
pub struct Symbol {
    pub primitive : String, 
    pub addr : i32,
    pub size : u32,
    pub register : i32,
    pub func : bool
}

pub struct SymbolTable {
    pub symbol_table : HashMap<String, Symbol>,
    pub ordinal : i32
}


impl SymbolTable {
    /* Handles updating address for each local variable from base of stack frame
    for easier assembly generation */
    pub fn insert(&mut self, identifier : &String, prim : &String, func : bool) {
        //Construct symbol
        self.symbol_table.insert(identifier.clone(), Symbol{primitive : prim.clone(), addr : self.ordinal as i32 * -8, size : 8, register : -1, func : func});

        //Update stack pointer
        self.ordinal += 1;
    }

    pub fn insert_argument(&mut self, identifier : &String, prim : &String, arg_ordinal : i32) {
        self.symbol_table.insert(identifier.clone(), Symbol{primitive : prim.clone(), addr : arg_ordinal as i32 * 8, size : 8, register : -1, func : false});
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

pub struct STNode {
    pub scope_index : RefCell<usize>,
    pub table : RefCell<SymbolTable>,
    pub parent : Option<RefCell<Weak<STNode>>>,
    pub children : RefCell<Vec<Rc<STNode>>>
}

pub trait TreeMethods {
    fn push_child(&self, ordinal : i32);
    
    fn get_table(&self) -> RefMut<SymbolTable>;

    fn scope_lookup(&self, identifier : &String) -> Option<Symbol>;

    fn bind(&self, identifier : &String, prim : &String, func : bool);

    fn bind_arg(&self, identifier : &String, prim : &String, arg_ordinal : i32);

    fn modify_register(&self, identifier : &String, register : i32);

    fn get_ordinal(&self) -> i32;

    fn in_table(&self, identifier : &String) -> bool;
}

pub fn create_new_stnode(ordinal : i32) -> Rc<STNode> {
    let sym_tab: SymbolTable = SymbolTable {
        symbol_table : HashMap::new(),
        ordinal : ordinal
    };

    return Rc::new(STNode {
        scope_index : RefCell::new(0),
        table : RefCell::new(sym_tab),
        parent : Option::Some(RefCell::new(Weak::new())),
        children : RefCell::new(Vec::new())
    })
}

impl TreeMethods for Rc<STNode> {

    fn push_child(&self, ordinal : i32) {
        
        let child: Rc<STNode> = create_new_stnode(ordinal);

        *child.parent.as_ref().unwrap().borrow_mut() = Rc::downgrade(&self);
        self.children.borrow_mut().push(child);
    }


    fn get_table(&self) -> RefMut<SymbolTable> {
        return self.table.borrow_mut();
    }

    fn scope_lookup(&self, identifier : &String) -> Option<Symbol> {
        
        if let Option::Some(symbol) = self.table.borrow().query(&identifier) {
            return Option::Some(symbol.clone());
        }

        let mut current_node: Rc<STNode> = self.clone();

        while let Option::Some(parent_node) = &current_node.parent {
            if let Option::Some(symbol) = parent_node.borrow().upgrade().unwrap().table.borrow().query(&identifier) {
                return Option::Some(symbol.clone());
            }
            
            current_node = parent_node.clone().borrow().upgrade().unwrap();
        }

        return Option::None;
    }

    fn bind(&self, identifier : &String, prim : &String, func : bool) {
        self.table.borrow_mut().insert(identifier, prim, func);
    }

    fn bind_arg(&self, identifier : &String, prim : &String, arg_ordinal : i32) {
        self.table.borrow_mut().insert_argument(identifier, prim, arg_ordinal);
    }

    fn modify_register(&self, identifier : &String, register : i32) {
        if self.in_table(identifier) {
            self.table.borrow_mut().modify_register(identifier, register);
            return;
        }

        let mut current_node: Rc<STNode> = self.clone();

        while let Option::Some(parent_node) = &current_node.parent {
            if parent_node.borrow().upgrade().unwrap().in_table(identifier) {
                parent_node.borrow().upgrade().unwrap().table.borrow_mut().modify_register(identifier, register);
                return;
            }
            
            current_node = parent_node.clone().borrow().upgrade().unwrap();
        }
    }

    fn in_table(&self, identifier : &String) -> bool {
        if let Option::Some(_symbol) = self.table.borrow().query(identifier) {
            return true;
        }
        return false;
    }

    fn get_ordinal(&self) -> i32 {
        return self.table.borrow().ordinal;
    }
}
