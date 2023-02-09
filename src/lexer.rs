use crate::tokenizer::Token;

pub enum Lexeme<'s>
{
    Import(&'s str),
    FunctionDefinition{name: &'s str, data: Vec<Lexeme<'s>>},
    FunctionCall{name: &'s str, args: Vec<Lexeme<'s>>}
}

impl<'s> Lexeme<'s>
{
    pub fn lex(tokens: Vec<Token>) -> Vec<Lexeme<'s>>
    {
        
        todo!();
    }
}