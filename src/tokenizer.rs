fn between<'a>(source: &'a str, start: &'a str, end: &'a str) -> Option<&'a str>
{
    if let Some(mut start_position) = source.find(start)
    {
        start_position += start.len();

        let source = &source[start_position..];
        let end_position = source.find(end)?;

        Some(&source[..end_position])
    }
    else
    {
        None
    }
}

pub enum Token<'s>
{
    Import,
    Fn,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    ThinArrow,
    DoubleColon,
    SemiColon,
    EndOfFile,
    StringLiteral(&'s str),
    Identifier(&'s str)
}

pub enum TokenizationError
{
    UnclosedDelimiter
}

impl std::fmt::Display for TokenizationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match *self
        {
            TokenizationError::UnclosedDelimiter => write!(f, "Unclosed Delimiter!")
        }
    }
}

impl<'s> std::fmt::Debug for Token<'s>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Import => write!(f, "~Import"),
            Self::Fn => write!(f, "~Fn"),
            Self::LeftParen => write!(f, "~("),
            Self::RightParen => write!(f, "~)"),
            Self::LeftBrace => write!(f, "~["),
            Self::RightBrace => write!(f, "~]"),
            Self::LeftCurlyBrace => write!(f, "~{{"),
            Self::RightCurlyBrace => write!(f, "~}}"),
            Self::ThinArrow => write!(f, "~->"),
            Self::DoubleColon => write!(f, "~::"),
            Self::SemiColon => write!(f, ";"),
            Self::EndOfFile => write!(f, "~EOF"),
            Self::StringLiteral(s) => write!(f, "~\"{s}\""),
            Self::Identifier(s) => write!(f, "|{s}|")
        }
    }
}

impl<'s> Token<'s>
{
    fn try_get_keyword(string: &str) -> Option<(&str, Token)>
    {
        macro_rules! keyword {
            ($working_string:ident, $match_string:literal, $match_token:expr) => {
                if let Some(s) = $working_string.strip_prefix($match_string)
                {
                    return Some((s, $match_token));
                }
            };
        }

        keyword!(string, "import", Token::Import);
        keyword!(string, "fn", Token::Fn);
        keyword!(string, "(", Token::LeftParen);
        keyword!(string, ")", Token::RightParen);
        keyword!(string, "[", Token::LeftBrace);
        keyword!(string, "]", Token::RightBrace);
        keyword!(string, "{", Token::LeftCurlyBrace);
        keyword!(string, "}", Token::RightCurlyBrace);
        keyword!(string, "->", Token::ThinArrow);
        keyword!(string, "::", Token::DoubleColon);

        None
    }

    fn find(string: &str) -> Result<(&str, Token), (TokenizationError, &str)>
    {
        if string.is_empty()
        {
            return Ok(("", Token::EndOfFile));
        }

        if let Some((s, t)) = Self::try_get_keyword(string)
        {
            return Ok((s, t));
        }

        // Literals
        if string.starts_with('\"')
        {
            let found_literal: &str = match between(string, "\"", "\"")
            {
                Some(s) => s,
                None => return Err((TokenizationError::UnclosedDelimiter, string))
            };
            return Ok((&string[found_literal.len() + 2..], Token::StringLiteral(found_literal)));
        }

        // Identifier
        // keep incrementing the string up until we find another keyword
        let mut identifier_size: usize = 0;

        loop
        {
            if identifier_size > string.len()
            {
                return Ok(("", Token::Identifier(string)));
            }

            if string[..identifier_size].ends_with([' ', '\n'])
            {
                return Ok((
                    &string[identifier_size - 1..],
                    Token::Identifier(&string[..identifier_size - 1])
                ));
            }

            if let Some((..)) = Self::try_get_keyword(&string[identifier_size..])
            {
                return Ok((
                    &string[identifier_size..],
                    Token::Identifier(&string[..identifier_size])
                ));
            }

            identifier_size += 1;
        }
    }

    // TODO: return a Vec<(Token, usize)> as a marker of where we are in the string
    pub fn parse(string: &str) -> Result<Vec<Token>, (TokenizationError, &str)>
    {
        let mut output: Vec<Token> = Vec::new();
        
        let mut rest_str: &str = string;
        let mut current_idx: usize = 0;

        loop
        {
            if rest_str.is_empty()
            {
                break;
            }
            let (next_str, t) = Token::find(rest_str)?;

            let rest_size: usize = rest_str.len();

            let trimmed_next: &str = next_str.trim_start();
            let next_size: usize = trimmed_next.len();

            rest_str = trimmed_next;
            output.push(t);
            current_idx += (rest_size - next_size);
        }

        Ok(output)
    }
}
