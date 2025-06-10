/* 
This file will be responsible for creating a parser which takes in a list of 
tokens from the lexer, and returns a parse tree. This file will contain all code
necessary to syntactically analyze the source file.
*/

/* 
This is the grammer that is currently being implemeted by the parser:

    x before a rule means it was noted but not used.
    Assume that functions don't have parameters/arguments.

    program_start -> function declaration
x   function declaration -> primitive identifier () ;
    function declaration -> primitive identifier () {body}
    body -> keyword constant ;
*/

static mut CURRENT_TOKEN_INDEX : u32 = 0; 

#[allow(non_camel_case_types)]
enum NodeType {
    
    Program_Start,
    Function_Declaration,
    Primitive,
    Identifier,
    Open_Paren,
    Close_Paren,
    Open_Curly,
    Body,
    Close_Curly,
    Return, //return is a reserved keyword
    Constant,
    Semicolon
}

struct Node {
    is_terminal : bool,
    node_type : NodeType,
    children : Vec<Node>
}

fn create_start_node() -> Node {
    return Node {
        is_terminal : false,
        node_type : NodeType::Program_Start,
        children : Vec::new()
    }
}

fn create_func_decl_node() -> Node {
    return Node {
        is_terminal : false,
        node_type : NodeType::Function_Declaration,
        children : Vec::new()
    }
}

fn create_primitive_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Primitive,
        children : Vec::new()
    }
}

fn create_identifier_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Identifier,
        children : Vec::new()
    }
}

fn create_open_paren_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Open_Paren,
        children : Vec::new()
    }
}

fn create_close_paren_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Close_Paren,
        children : Vec::new()
    }
}

fn create_body_node() -> Node {
    return Node {
        is_terminal : false,
        node_type : NodeType::Body,
        children : Vec::new()
    }
}

fn create_open_curly_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Open_Curly,
        children : Vec::new()
    }
}

fn create_close_curly_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Close_Curly,
        children : Vec::new()
    }
}

fn create_return_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Return,
        children : Vec::new()
    }
}

fn create_constant_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Constant,
        children : Vec::new()
    }
}

fn create_semicolon_node() -> Node {
    return Node {
        is_terminal : true,
        node_type : NodeType::Semicolon,
        children : Vec::new()
    }
}

fn parse(mut current_node : Node) -> Node {

    match current_node.node_type {

        NodeType::Program_Start => {

            //Create a new node of type function declaration            
            let function_declaration_node : Node = create_func_decl_node();

            //Parse and assign new node to be child of current_node
            current_node.children.push(parse(function_declaration_node));

            
            //Return current_node
            return current_node;
        }
        
        NodeType::Function_Declaration => {

            let primitive_node : Node = create_primitive_node();
            let identifier_node : Node = create_identifier_node();
            let open_paren_node : Node = create_open_paren_node();
            let close_paren_node : Node = create_close_paren_node();
            let open_curly_node : Node = create_open_curly_node();
            let body_node : Node = create_body_node();
            let close_curly_node : Node = create_close_curly_node();
            
            current_node.children.push(parse(primitive_node));
            current_node.children.push(parse(identifier_node));
            current_node.children.push(parse(open_paren_node));
            current_node.children.push(parse(close_paren_node));
            current_node.children.push(parse(open_curly_node));
            current_node.children.push(parse(body_node));
            current_node.children.push(parse(close_curly_node));

            return current_node;
        }

        NodeType::Primitive => {

            return current_node;
        }

        NodeType::Identifier => {

            return current_node;
        }

        NodeType::Open_Paren => {

            return current_node;
        }

        NodeType::Close_Paren => {

            return current_node;
        }

        NodeType::Open_Curly => {

            return current_node;
        }

        NodeType::Body => {

            let return_node : Node = create_return_node();
            let constant_node : Node = create_constant_node();
            let semicolon_node : Node = create_semicolon_node();

            current_node.push(parse(return_node));
            current_node.push(parse(constant_node));
            current_node.push(parse(semicolon_node));

            return current_node;
        }

        NodeType::Close_Curly => {

            return current_node;
        }

        NodeType::Return => {

            return current_node;
        }

        NodeType::Constant => {

            return current_node;
        }

        NodeType::Semicolon => {

            return current_node;
        }

    }

}

fn main() {
    println!("Just for compliation");
}
