use std::mem::ManuallyDrop;

#[derive(Debug)]
pub enum Token {
    Ok(Type, String),
    Err(String),
}

#[derive(Debug)]
pub enum Type {
    Identifier,
    Colon,
}

pub fn lex(src: String) -> Vec<Token> {
    let len = src.len();
    let mut pos: usize = 0;
    let mut chars = src.chars();
    let mut tokens = Vec::<Token>::new();

    while pos < len {
        let opt_character = chars.next();

        if let Some(character) = opt_character {
            tokens.push(if character.is_alphabetic() {
                let start = pos;
    
                loop {
                    if let Some(c) = chars.next() {
                        if c.is_alphanumeric() {
                            pos += 1;
                            continue;
                        }
                    }
                    break;
                }

                Token::Ok(
                    Type::Identifier,
                    src[start..=pos].to_string(),
                )
            } else {
                let token = match character {
                    ':' => Token::Ok(Type::Colon, character.to_string()),
                    _ => Token::Err(character.to_string()),
                };

                pos += 1;
                
                token
            });
        } else {
            break;
        }
    }

    return tokens;
}
