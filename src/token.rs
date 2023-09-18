use crate::opcodes::{Opcode, UNKNOWN};

const CONSTANT_START: &str = "0x";
const REFERENCE_START: &str = "[";
const VARIABLE_START: &str = "<";

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Constant,
    Opcode,
    Reference,
    Variable,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub opcode: Option<Opcode>, // Only has an opcode if token_type is Opcode
    pub operands: Vec<Token>,
}

impl Token {
    pub fn new() -> Token {
        Token {
            token_type: TokenType::Unknown,
            value: String::new(),
            opcode: Some(UNKNOWN),
            operands: Vec::new(),
        }
    }

    pub fn from_string(word: &str) -> Token {
        let word = word.trim();
        let mut token = Token::new();
        let token_type = match word {
            _ if word.starts_with(CONSTANT_START) => TokenType::Constant,
            _ if word.starts_with(REFERENCE_START) => TokenType::Reference,
            _ if word.starts_with(VARIABLE_START) => TokenType::Variable,
            _ => TokenType::Opcode,
        };

        token.token_type = token_type;
        token.value = word.to_string();

        if token.token_type == TokenType::Opcode {
            token.opcode = Some(Opcode::from_string(word));
        }

        token
    }

    pub fn to_str(&self) -> String {
        match &self.token_type {
            TokenType::Opcode => {
                let operands_str = self
                    .operands
                    .iter()
                    .map(|operand| operand.to_str())
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("{}: {}", self.opcode.as_ref().unwrap().name, operands_str)
            }
            _ => self.value.clone(),
        }
    }
}
