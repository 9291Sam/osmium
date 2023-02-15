use crate::tokenizer::FileToken;

pub enum Lexeme<'s> {
    Import(&'s str),
    FunctionDefinition { name: &'s str, data: Vec<Lexeme<'s>> },
    FunctionCall { name: &'s str, args: Vec<Lexeme<'s>> },
}

impl<'s> Lexeme<'s> {
    pub fn lex(_tokens: Vec<FileToken>) -> Vec<Lexeme<'s>>
    {
        
        todo!();
    }
}
