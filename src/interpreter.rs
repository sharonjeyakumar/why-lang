use crate::parser::ASTNode;

pub fn interpret(ast: &[ASTNode]) {
    for node in ast {
        match node {
            ASTNode::ExplainType { typename } => {
               match typename.as_str() {
                    "say" => println!("say is a keyword that is used to print data"),
                    "integer" => println!("integer is a datatype that is used to store whole numbers"),
                    "why" => println!("An educational Programming language"),
                    "boolean" => println!("Boolean is a datatype that is used to store two values, True or False"),
                    _ => println!("Unknown type: {}", typename),
                }
            }
            ASTNode::VarDeclaration { name, value } => {
                println!("Declared variable '{}' with value {}", name, value);
            }
            ASTNode::PrintStatement { name } => {
                println!("{}", name);
            }
        }
    }
}
