// Все функции, принимающие State, после успешного завершения переводят его на последний токен, который был обработан
// Кроме функции HaveNextTextBlock в selector

pub mod aux;
pub mod data;
pub mod method;
pub mod node;
pub mod selector;
pub mod text;

use crate::{
    Result,
    lexic::token::{BaseToken, Token, TokenKind},
    synt::data::Data,
};
use {aux::State, node::Node};

#[derive(Debug)]
pub struct Let {
    base: BaseToken,
    data: Data,
}

impl Let {
    pub fn new(base: BaseToken, data: Data) -> Self {
        Let { base, data }
    }
}

pub fn parse(tokens: &[Token], source_code: &[u8]) -> Result<Vec<Node>> {
    let mut lets = Vec::new();
    let mut state = State::new(tokens, &mut lets, source_code, 0);
    let mut nodes = Vec::with_capacity(tokens.len() / 5);

    while !state.is_empty() {
        if state[0].kind == TokenKind::Let {
            method::let_parse(&mut state)?;
        } else {
            nodes.push(method::choose_parse(&mut state)?);
        }

        state += 1;
    }

    Ok(nodes)
}
