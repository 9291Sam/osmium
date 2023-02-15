use std::{slice::{Iter, IterMut}, rc::Rc, cell::{UnsafeCell, Cell, RefCell}, borrow::BorrowMut, fs::File};

use crate::tokenizer::{FileToken, Token};

pub enum Lexeme<'s> {
    Import(&'s str),
    FunctionDefinition { name: &'s str, data: Vec<Lexeme<'s>> },
    FunctionCall { name: &'s str, args: Vec<Lexeme<'s>> },
}

pub enum LexingError<'a>
{
    UnexpectedToken(FileToken<'a>)
}

fn lex_function<'a>(iterator: &mut Iter<FileToken>) -> Result<Lexeme<'a>, LexingError<'a>>
{
    todo!();
}

impl<'s> Lexeme<'s>
{
    pub fn lex(tokens: Vec<FileToken>) -> Result<Vec<Lexeme<'s>>, LexingError>
    {
        let mut output: Vec<Lexeme> = Vec::new();
        let mut next: Iter<FileToken> = tokens.iter();
        
        loop
        {
            output.push(
                match next.borrow_mut().next()
                {
                    Some(global_token) =>
                    {
                        match global_token.token
                        {
                            Token::Import    => lex_function(next.borrow_mut())?,
                            Token::Fn        => todo!(),
                            Token::EndOfFile => break,
                            _                => return Err(LexingError::UnexpectedToken(global_token.clone()))
                        }
                    },
                    None => todo!(),
                }
            );
        }
        
        Ok(output)
    }
}
