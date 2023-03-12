use crate::tokenizer::{FileToken, Token};

pub enum LexerError
{
    UnexpectedToken
}

pub enum Lexeme
{

}

impl Lexeme
{
    pub fn parse_from(tokens: Vec<FileToken>) -> Result<Vec<Lexeme>, LexerError>
    {
        let mut it = tokens.iter();

        todo!();
    }
}
