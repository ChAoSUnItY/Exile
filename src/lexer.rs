#[derive(Debug, Clone)]
pub enum Token {
    Ok(Type, String),
    Err(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Identifier,
    Whitespace,
    LineBreak,
    Colon,
}

pub fn lex(src: String) -> Vec<Token> {
    let len = src.len();
    let mut pos: usize = 0;
    let chars = src.chars().collect::<Vec<char>>();
    let mut tokens = Vec::<Token>::new();

    while pos <= len {
        let opt_character = chars.get(pos);

        if let Some(character) = opt_character {
            let token = if character.is_alphabetic() {
                let start = pos;
    
                loop {
                    if let Some(c) = chars.get(pos) {
                        if c.is_alphanumeric() {
                            pos += 1;
                            continue;
                        }
                    }
                    break;
                }

                Token::Ok(
                    Type::Identifier,
                    src[start..pos].to_string(),
                )
            } else {
                let token = match character {
                    ':' => Token::Ok(Type::Colon, character.to_string()),
                    character if character.is_whitespace() => Token::Ok(Type::Whitespace, character.to_string()),
                    '\r' | '\n' => Token::Ok(Type::LineBreak, character.to_string()),
                    _ => Token::Err(character.to_string()),
                };

                pos += 1;
                
                token
            };

            tokens.push(token);
        } else {
            break;
        }
    }

    return tokens;
}
