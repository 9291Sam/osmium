use crate::tokenizer::{FileToken, Token};

pub enum LexerError
{
    UnexpectedToken,
    NoRemainingTokens
}

pub enum Lexeme
{

}

impl Lexeme
{
    pub fn parse_from(tokens: Vec<FileToken>) -> Result<Vec<Lexeme>, LexerError>
    {
        let mut it = tokens.iter();

        match it.next().ok_or(LexerError::NoRemainingTokens)?.token
        {
            Token::Import => todo!(),
            Token::LeftParen => todo!(),
            Token::RightParen => todo!(),
            Token::LeftBrace => todo!(),
            Token::RightBrace => todo!(),
            Token::LeftCurlyBrace => todo!(),
            Token::RightCurlyBrace => todo!(),
            Token::DoubleColon => todo!(),
            Token::SemiColon => todo!(),
            Token::EndOfFile => todo!(),
            Token::StringLiteral(_) => todo!(),
            Token::Identifier(_) => todo!(),
        }
    }
}
