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

pub enum Token<'s>
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
    EndOfFile,
    StringLiteral(&'s str),
    Identifier(&'s str)
}

pub enum TokenizationError
{
    UnclosedDelimiter
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
            Self::EndOfFile => write!(f, "~EOF"),
            Self::StringLiteral(s) => write!(f, "~\"{s}\""),
            Self::Identifier(s) => write!(f, "|{s}|"),
        }
    }
}


impl<'s> Token<'s>
{
    fn find(string: &str) -> Result<(&str, Token), TokenizationError>
    {
        if string.is_empty()
        {
            return Ok(("", Token::EndOfFile));
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

    pub fn parse(string: &str) -> Result<Vec<Token>, TokenizationError>
    {
        let mut output: Vec<Token> = Vec::new();

        let mut rest_str: &str = string;

        loop 
        {
            if rest_str.is_empty()
            {
                break;
            }
            let (next_str, t) = Token::find(rest_str)?;
            
            rest_str = next_str.trim_start();
            output.push(t); 
        }
        
        Ok(output)
    }
}