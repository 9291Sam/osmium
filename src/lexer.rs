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
    fn parse_from(tokens: Vec<FileToken>) -> Result<Vec<Lexeme>, LexerError>
    {
        todo!();
    }
}

