use std::{fs::File, io::Read};

fn between<'a>(source: &'a str, start: &'a str, end: &'a str) -> Option<&'a str> {
    let start_position = source.find(start);

    if start_position.is_some() {
        let start_position = start_position.unwrap() + start.len();
        let source = &source[start_position..];
        let end_position = source.find(end)?;
        return Some(&source[..end_position]);
    }
    return None;
}
enum Token<'s>
{
    Import,
    Fn,
    EqualsSign,
    Quote,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    StringLiteral(&'s str),
    Identifier(&'s str)
}

impl<'s> std::fmt::Debug for Token<'s>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Import => write!(f, "~Import"),
            Self::Fn => write!(f, "~Fn"),
            Self::EqualsSign => write!(f, "~EqualsSign"),
            Self::Quote => write!(f, "~\""),
            Self::LeftParen => write!(f, "~("),
            Self::RightParen => write!(f, "~)"),
            Self::LeftBrace => write!(f, "~["),
            Self::RightBrace => write!(f, "~]"),
            Self::LeftCurlyBrace => write!(f, "~{{"),
            Self::RightCurlyBrace => write!(f, "~}}"),
            Self::StringLiteral(s) => write!(f, "~\"{s}\""),
            Self::Identifier(s) => write!(f, "|{s}|"),
        }
    }
}
enum FindReturn
    {
        EndOfInput,
        UnclosedDelimiter,
    }

impl<'s> Token<'s>
{
    
    // &str of the rest of the string
    // Token - the token removed
    fn find(string: &str) -> Result<(&str, Token), FindReturn>
    {
        if string.is_empty()
        {
            return Err(FindReturn::EndOfInput);
        }

        // Key words
        if let Some(string) = string.strip_prefix("import")
        {
            return Ok((string, Token::Import));
        }

        if let Some(string) = string.strip_prefix("fn")
        {
            return Ok((string, Token::Fn));
        }


        // Symbols


        // Literals
        if string.starts_with('\"')
        {
            let found_literal: &str = between(string, "\"", "\"").expect("Unclosed Delimiter");
            return Ok((&string[found_literal.len() + 2..], Token::StringLiteral(found_literal)));
        }

        // if string[0] is


        // the identifers is the start to the first whitespace
        // println!("str: {string}");
        let idx: usize = string.find(' ').expect("No Final Identifier!");
        Ok((&string[idx..], Token::Identifier(&string[..idx])))
    }

    fn parse(string: &str) -> Vec<Token>
    {
        let mut output: Vec<Token> = Vec::new();

        let mut rest_str: &str = string;

        while let Ok((next_str, t)) = Token::find(rest_str)
        {
            rest_str = next_str.trim_start();
            output.push(t);
        }
        
        output
    }
}

fn main()
{
    let mut raw_file: String = String::new();
    File::open("main.osm").unwrap().read_to_string(&mut raw_file).unwrap();
    raw_file = raw_file.replace('\n', " ");
    raw_file = raw_file.replace('(', " ( ");
    raw_file = raw_file.replace(')', " ) ");
    raw_file = raw_file.replace('[', " [ ");
    raw_file = raw_file.replace(']', " ] ");
    raw_file = raw_file.replace('{', " { ");
    raw_file = raw_file.replace('}', " } ");

    let tokens: Vec<Token> = Token::parse(&raw_file);

    
}
