use std::iter::Peekable;

use crate::lexer::{Token, Type};

#[derive(Debug)]
pub struct Method {
    pub return_type: String,
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Push(Token),
    Ret(),
}

fn assert(token: &Token, token_type: Type) -> &Token {
    if token.token_type == token_type {
        token
    } else {
        panic!(
            "Unexpected token {:?}, expected {:?}",
            token.token_type, token_type
        )
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Method> {
    let mut token_iter = tokens
        .iter()
        .filter(|token| token.token_type != Type::Whitespace)
        .peekable();
    let mut methods = Vec::<Method>::new();

    while token_iter.peek().is_some() {
        methods.push(parse_method(
            token_iter.next(),
            token_iter.next(),
            &mut token_iter,
        ));
    }

    methods
}

fn parse_method<'a>(
    return_type: Option<&Token>,
    method_name: Option<&Token>,
    token_iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Method {
    assert(
        token_iter.next().expect("Expected colon for method"),
        Type::Colon,
    );
    assert(
        token_iter.next().expect("Expected line break"),
        Type::LineBreak,
    );

    let mut instructions = Vec::<Instruction>::new();

    while token_iter.peek().is_some() {
        instructions.push(parse_instruction(token_iter));
        assert(
            token_iter.next().expect("Expected line break"),
            Type::LineBreak,
        );
    }

    Method {
        return_type: assert(
            return_type.expect("Expected idientifier for return type"),
            Type::Identifier,
        )
        .to_owned()
        .literal,
        name: assert(
            method_name.expect("Expected identifier for method name."),
            Type::Identifier,
        )
        .to_owned()
        .literal,
        instructions,
    }
}

fn parse_instruction<'a>(
    token_iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Instruction {
    match token_iter.next() {
        Some(Token {
            token_type,
            literal,
        }) => {
            if *token_type != Type::Identifier {
                panic!("Expected identifier for instruction.")
            }

            match literal.as_str() {
                "push" => Instruction::Push(
                    assert(
                        token_iter
                            .next()
                            .expect("Expected number for push instruction"),
                        Type::Integer,
                    )
                    .to_owned(),
                ),
                "ret" => Instruction::Ret(),
                _ => panic!("Unkown instruction {}", literal),
            }
        }
        _ => panic!("Expected identifier for instruction."),
    }
}