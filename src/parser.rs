use crate::tokenizer::Token;

#[derive(Debug)]
pub enum ASTNode {
    ExplainType { typename: String },
    VarDeclaration { name: String, value: i32 },
    PrintStatement { name: String },
}

pub fn parse(tokens: &[Token]) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut i = 0;

    while i < tokens.len(){
        match &tokens[i] {
            Token::Keyword(kw) if kw == "why" => {
                if let Some(next) = tokens.get(i+1){
                    let typename = match next {
                        Token::Identifier(name) => name.clone(),
                        Token::Keyword(name) => name.clone(),
                        //have to add other datatype here like symbol
                        _ => panic!("Expected type name after 'why'"),
                    };
                    ast.push(ASTNode::ExplainType {typename});
                    i +=2;
                } else {
                    panic!("Expected type name after 'why'");
                }
            }
            Token::Keyword(kw) if kw == "say" => {
                if let Some(next) = tokens.get(i+1){
                    let value = match next {
                        Token::StringLiteral(s) => s.clone(),
                        Token::StringLiteral(name) => name.clone(),
                        _=> panic!("Expected string literal or identifier after 'say'"),
                    };
                    ast.push(ASTNode::PrintStatement {name: value});
                    i += 2;
                } else {
                    panic!("Expected value after 'say'");
                }
            }
            _=> {
                panic!("Unexpected token: {:?}", tokens[i]);
            }
        }
    }
    ast
}