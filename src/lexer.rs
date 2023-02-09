pub enum Lexeme<'s>
{
    Import(&'s str),
    FunctionDefinition{name: &str, data: Vec<Lexeme>},
    FunctionCall{name: &str, args: Vec<Lexeme>}
}