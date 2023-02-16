use std::{slice::Iter, iter::Peekable, fs::File};

use crate::tokenizer::{FileToken, Token};

pub enum Lexeme<'s>
{
    Import(&'s str),
    Type(&'s str),
    FunctionDefinition { name: &'s str, data: Vec<Lexeme<'s>> },
    FunctionCall { name: &'s str, args: Vec<Lexeme<'s>> },
}

pub enum LexingError<'a>
{
    NoRemainingTokens,
    UnexpectedToken(FileToken<'a>)
}

macro_rules! next_should_be {
    ($iterator: expr, $token: pat) =>
    {
        {
            let next_file_token = $iterator.next()
                .ok_or_else(|| LexingError::NoRemainingTokens)?;

            match next_file_token.token
            {
                $token => Ok(next_file_token),
                _ => Err(LexingError::UnexpectedToken(next_file_token.clone())),
            }
        }
    };
}

fn lex_function<'a>(iterator: &mut Peekable<Iter<FileToken<'a>>>) -> Result<Lexeme<'a>, LexingError<'a>>
{
    next_should_be!(iterator, Token::Fn)?;

    let name = next_should_be!(iterator, Token::Identifier(_))?;

    // let args = {
    //     // todo!();
    // };

    let _ = next_should_be!(iterator, Token::ThinArrow)?;
    let return_type = next_should_be!(iterator, Token::Identifier(_))?
    // };

    

    // // try to either parse a return tyoe x

    // let return_type = {
        
    // };
    



    // let function_tokens: Vec<&FileToken> = Vec::new();  

    // loop
    // {
    //     let current_file_token = iterator.next().ok_or_else(|| LexingError::NoRemainingTokens)?;

    //     match current_file_token.token
    //     {
    //         Token::Import => todo!(),
    //         Token::Fn => todo!(),
    //         Token::LeftParen => todo!(),
    //         Token::RightParen => todo!(),
    //         Token::LeftBrace => todo!(),
    //         Token::RightBrace => todo!(),
    //         Token::LeftCurlyBrace => todo!(),
    //         Token::RightCurlyBrace => todo!(),
    //         Token::ThinArrow => todo!(),
    //         Token::DoubleColon => todo!(),
    //         Token::SemiColon => todo!(),
    //         Token::EndOfFile => todo!(),
    //         Token::StringLiteral(_) => todo!(),
    //         Token::Identifier(_) => todo!(),
    //     }
    //     // add on new braces
    //     // decrement on old braces
        
    // }
    
    // // {
    // //     if 
    // // }
    // // Ok(Lexeme::FunctionDefinition { name: iter, data: () })
    todo!()
}

fn lex_import<'a>(iterator: &mut Peekable<Iter<FileToken>>) -> Result<Lexeme<'a>, LexingError<'a>>
{
    todo!();
}

impl<'s> Lexeme<'s>
{
    pub fn lex(tokens: Vec<FileToken<'_>>) -> Result<Vec<Lexeme<'_>>, LexingError>
    {
        let mut output: Vec<Lexeme> = Vec::new();
        let mut next: Peekable<Iter<FileToken>> = tokens.iter().peekable();
        
        loop
        {
            output.push(
                match next.peek()
                {
                    Some(token) => 
                    {
                        let file_token = token;
                        match file_token.token
                        {
                            Token::Import    => lex_import(&mut next)?,
                            Token::Fn        => lex_function(&mut next)?,
                            Token::EndOfFile => break,
                            _                => return Err(LexingError::UnexpectedToken((*file_token).clone()))
                        }
                    },
                    None => todo!(),
                }
            );
        }
        
        Ok(output)
    }
}
