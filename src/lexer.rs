#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Type,
    pub literal: String,
}

fn new(token_type: Type, literal: String) -> Token {
    Token {
        token_type,
        literal,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Identifier,
    Integer,
    Whitespace,
    LineBreak,
    Colon,
    Error
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

                new(Type::Identifier, src[start..pos].to_string())
            } else if character.is_numeric() {
                let start = pos;

                loop {
                    if let Some(c) = chars.get(pos) {
                        if c.is_numeric() {
                            pos += 1;
                            continue;
                        }
                    }
                    break;
                }

                new(Type::Integer, src[start..pos].to_string())
            } else {
                let token = match character {
                    ':' => new(Type::Colon, character.to_string()),
                    character if character.is_whitespace() => {
                        new(Type::Whitespace, character.to_string())
                    }
                    '\r' | '\n' => new(Type::LineBreak, character.to_string()),
                    _ => new(Type::Error, character.to_string()),
                };

                pos += 1;

                token
            };

            tokens.push(token);
        } else {
            break;
        }
    }

    // add line break token
    tokens.push(new(Type::LineBreak, "\n".to_string()));

    tokens
}
