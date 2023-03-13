use crate::tokenizer::{FileToken, Token};

pub enum LexerError
{
    UnexpectedToken,
    NoRemainingTokens
}

pub struct FileLexerError
{
    line:   usize,
    column: usize,
    error:  LexerError
}

pub enum Lexeme<'a>
{
    Import
    {
        library_to_import: &'a str,
    },
    Function
    {
        return_type: &'a str,
        name:        &'a str,
        parameters:  &'a str,
        body:        &'a str,
    },
}

impl<'a> Lexeme<'a>
{
    pub fn parse_from(tokens: &'a Vec<FileToken>) -> Result<Vec<Lexeme<'a>>, FileLexerError>
    {
        let mut lexemes: Vec<Lexeme> = Vec::new();

        let mut iterator = tokens.iter().peekable();

        loop
        {
            if iterator.peek().is_none()
            {
                return Ok(lexemes);
            }
            
            match iterator.peek().unwrap().token
            {
                Token::Import =>
                {
                    lexemes.push(lex_import_statement(&mut iterator)?)
                },
                Token::Identifier(i) =>
                {
                    lexemes.push(lex_function(&mut iterator)?)
                },
                Token::EndOfFile => return Ok(lexemes),
                _ => return Err(FileLexerError { 
                    line: iterator.peek().unwrap().line,
                    column: iterator.peek().unwrap().column,
                    error: LexerError::UnexpectedToken 
                })
            }
        }
    }
}
