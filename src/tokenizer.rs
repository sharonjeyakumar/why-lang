
#[derive(Debug)]
pub enum Token {
    Number(i32),
    Keyword(String),
    Identifier(String),
    Symbol(char),
    StringLiteral(String),
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let keywords = ["why","say","integer", "boolen"];
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();

    while let Some((start_idx, c)) = chars.next() {
        if c.is_whitespace() {
            continue;
        } else if c == '"' {
            let mut end_idx = start_idx + 1;
            while let Some(&(i, next)) = chars.peek() {
                end_idx = i;
                chars.next();
                if next =='"' {
                    let string = &source[start_idx+1..end_idx];
                    tokens.push(Token::StringLiteral(string.to_string()));
                    break;
                }
            }
        } else if c.is_ascii_digit() {
            let mut end_idx = start_idx;
            while let Some(&(i, next)) = chars.peek() {
                if next.is_ascii_digit() {
                    end_idx = i;
                    chars.next();
                } else {
                    break;
                }
            }
            let number_str = &source[start_idx..=end_idx];
            let number = number_str.parse::<i32>().unwrap();
            tokens.push(Token::Number(number));
        }else if "=+-*/".contains(c) {
            tokens.push(Token::Symbol(c));
        } else {
            // identifier or keyword
            let mut end_idx = start_idx;
            while let Some(&(i, next)) = chars.peek() {
                if next.is_whitespace() || "=+-*/\"".contains(next) {
                    break;
                } else {
                    end_idx = i + next.len_utf8();
                    chars.next();
                }
            }
            let word = &source[start_idx..end_idx];
            if keywords.contains(&word) {
                tokens.push(Token::Keyword(word.to_string()));
            } else {
                tokens.push(Token::Identifier(word.to_string()));
            }
        }
    }
    // println!("{:?}", tokens);
    tokens
}
    