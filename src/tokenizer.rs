use lazy_static::lazy_static;
use regex::Regex;

fn between<'a>(source: &'a str, start: &'a str, end: &'a str) -> Option<&'a str> {
    if let Some(mut start_position) = source.find(start) {
        start_position += start.len();

        let source = &source[start_position..];
        let end_position = source.find(end)?;

        Some(&source[..end_position])
    } else {
        None
    }
}

pub struct PinnedToken<'s> {
    pub location: usize,
    pub token: Token<'s>,
}

impl<'s> std::fmt::Debug for PinnedToken<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {:?}", self.location, self.token)
    }
}

pub enum Token<'s> {
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
    Identifier(&'s str),
}

pub struct PinnedTokenizationError {
    pub location: usize,
    pub token: TokenizationError,
}

impl std::fmt::Debug for PinnedTokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "loc: {}, tok: {}", self.location, self.token)
    }
}

pub enum TokenizationError {
    UnclosedDelimiter,
}

impl std::fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenizationError::UnclosedDelimiter => write!(f, "Unclosed Delimiter!"),
        }
    }
}

impl<'s> std::fmt::Debug for Token<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::SemiColon => write!(f, "~;"),
            Self::EndOfFile => write!(f, "~EOF"),
            Self::StringLiteral(s) => write!(f, "|\"{s}\"|"),
            Self::Identifier(s) => write!(f, "|{s}|"),
        }
    }
}

impl<'s> Token<'s> {
    // Token and how many characters said token is
    fn try_get_keyword(string: &str) -> Option<(Token, usize)> {
        macro_rules! keyword {
            ($working_string:ident, $match_string:literal, $match_token:expr) => {
                if let Some(s) = $working_string.strip_prefix($match_string) {
                    return Some(($match_token, $match_string.len()));
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
        keyword!(string, ";", Token::SemiColon);

        None
    }

    fn try_parse_string_literal(string: &str) -> Option<Result<(Token, usize), TokenizationError>> {
        if let Some(s) = string.strip_prefix('\"') {
            lazy_static! {
                static ref RE: Regex = Regex::new(r#"(?:[^"\\]|\\.)*"#).unwrap();
            }

            if let Some(l) = RE.find(s) 
            {
                // This + 2 is for the two quotes matched at the beginning an end of the string
                Some(Ok((Token::StringLiteral(l.as_str()), l.as_str().len() + 2)))
            } 
            else
            {
                Some(Err(TokenizationError::UnclosedDelimiter))
            }
        } else {
            None
        }
    }

    // returns token and the length of said token
    fn find(string: &str) -> Result<(Token, usize), TokenizationError> {
        if string.is_empty() {
            return Ok((Token::EndOfFile, 0));
        }

        if let Some(t_s) = Self::try_get_keyword(string) {
            return Ok(t_s);
        }

        // Literals
        if let Some(t_s) = Self::try_parse_string_literal(string) {
            return t_s;
        }

        // Identifier
        // keep incrementing the string up until we find another keyword
        let mut identifier_size: usize = 0;

        loop {
            if identifier_size > string.len() {
                return Ok((Token::Identifier(string), string.len()));
            }

            // We found another identifier
            if string[..identifier_size].ends_with([' ', '\n']) {
                let found_identifier: &str = &string[..identifier_size - 1];
                return Ok((Token::Identifier(found_identifier), found_identifier.len()));
            }

            // There's an upcoming keyword, everything before is an identifier
            if Self::try_get_keyword(&string[identifier_size..]).is_some() {
                let found_identifier = &string[..identifier_size];
                // this + 2 is for the two encompassing quotes
                return Ok((Token::Identifier(found_identifier), found_identifier.len()));
            }

            identifier_size += 1;
        }
    }

    pub fn parse(string: &str) -> Result<Vec<PinnedToken>, PinnedTokenizationError> {
        let mut output: Vec<PinnedToken> = Vec::new();

        let mut current_idx: usize = 0;

        loop {
            let rest_str: &str = &string[current_idx..];

            if rest_str.is_empty() {
                break;
            }

            match Token::find(rest_str) {
                Ok((t, size)) => {
                    output.push(PinnedToken { location: current_idx, token: t });
                    current_idx += size;
                    current_idx +=
                        string[current_idx..].len() - string[current_idx..].trim_start().len();
                },
                Err(e) => {
                    return Err(PinnedTokenizationError { location: current_idx, token: e });
                },
            };
        }

        Ok(output)
    }
}
