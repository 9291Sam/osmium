#[derive(Clone)]
pub struct FileToken<'t>
{
    pub line: usize,
    pub column: usize,
    pub token: Token<'t>,
}

impl std::fmt::Display for FileToken<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Token @ {}:{} -> {}", self.line, self.column, self.token)
    }
}

#[derive(Debug)]
pub struct FileTokenizationError
{
    pub line: usize,
    pub column: usize,
    pub error: TokenizationError,
}

impl std::fmt::Display for FileTokenizationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(
            f, 
            "Tokenization Error: {} @ {}:{}",
            self.error,
            self.line,
            self.column
        )
    }
}

#[derive(Debug)]
pub enum TokenizationError
{
    UnclosedDelimiter,
}

impl std::fmt::Display for TokenizationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", 
            match *self
            {
                TokenizationError::UnclosedDelimiter => "Unclosed Delimiter!",  
            }
        )
    }
}


#[derive(Clone)]
pub enum Token<'s>
{
    Import,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftCurlyBrace,
    RightCurlyBrace,
    DoubleColon,
    SemiColon,
    Comma, 
    EndOfFile,
    StringLiteral(&'s str),
    Identifier(&'s str),
}

impl std::fmt::Display for Token<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let string: String = match *self
        {
            Token::Import           => "~Import".to_owned(),
            Token::LeftParen        => "~(".to_owned(),
            Token::RightParen       => "~)".to_owned(),
            Token::LeftBracket        => "~[".to_owned(),
            Token::RightBracket       => "~]".to_owned(),
            Token::LeftCurlyBrace   => "~{".to_owned(),
            Token::RightCurlyBrace  => "~}".to_owned(),
            Token::DoubleColon      => "~::".to_owned(),
            Token::SemiColon        => "~;".to_owned(),
            Token::Comma            => "~,".to_owned(),
            Token::EndOfFile        => "~EOF".to_owned(),
            Token::StringLiteral(s) => format!("Lit |{s}|"),
            Token::Identifier(i)    => format!("ident |{i}|"),
        };

        write!(f, "{}", &string)
    }
}

impl<'s> Token<'s>
{
    fn size(&self) -> usize
    {
        match *self
        {
            Token::Import           => 6,
            Token::LeftParen        => 1,
            Token::RightParen       => 1,
            Token::LeftBracket        => 1,
            Token::RightBracket       => 1,
            Token::LeftCurlyBrace   => 1,
            Token::RightCurlyBrace  => 1,
            Token::DoubleColon      => 2,
            Token::SemiColon        => 1,
            Token::Comma            => 1,
            Token::EndOfFile        => 0,
            Token::StringLiteral(s) => s.len(),
            Token::Identifier(i)    => i.len(),
        }
    }

    /// Tries to find the first keyword token available in the string
    /// returns `None` when there is no token available
    fn try_get_keyword(string: &str) -> Option<Token>
    {
        macro_rules! keyword {
            ($working_string:ident, $match_string:literal, $match_token:expr) =>{
                if let Some(_) = $working_string.strip_prefix($match_string) {
                    return Some($match_token);
                }
            };
        }

        keyword!(string, "import", Token::Import);
        keyword!(string, "(", Token::LeftParen);
        keyword!(string, ")", Token::RightParen);
        keyword!(string, "[", Token::LeftBracket);
        keyword!(string, "]", Token::RightBracket);
        keyword!(string, "{", Token::LeftCurlyBrace);
        keyword!(string, "}", Token::RightCurlyBrace);
        keyword!(string, "::", Token::DoubleColon);
        keyword!(string, ";", Token::SemiColon);
        keyword!(string, ",", Token::Comma);

        None
    }

    /// Attempts to parse a string literal from the beginning of this string
    /// 
    /// Ok(Token)
    /// Err(TokenizationError) - unclosed delimiter
    /// Err(None) - no
    /// 
    fn try_parse_string_literal(input: &str) -> Option<Result<Token, TokenizationError>>
    {
        if !input.starts_with('"') {
            return None;
        }

        let mut iter = input[1..].char_indices();
        let mut last_char = None;

        loop {
            match iter.next()
            {
                Some((index, ch)) =>
                {
                    if ch == '"' && last_char != Some('\\')
                    {
                        return Some(Ok(Token::StringLiteral(&input[..index + 2])));
                    }
                    else if ch == '\n'
                    {
                        return Some(Err(TokenizationError::UnclosedDelimiter))
                    }
                    else
                    {
                        last_char = Some(ch);
                        continue;
                    }
                }
                None => 
                {
                    return Some(Err(TokenizationError::UnclosedDelimiter));
                }
            }
        }
    }

    /// Returns the first token found 
    fn find(string: &str) -> Result<Token, TokenizationError> 
    {
        if string.is_empty()
        {
            return Ok(Token::EndOfFile);
        }

        // Keywords
        if let Some(token) = Self::try_get_keyword(string)
        {
            return Ok(token);
        }

        // Literals
        if let Some(token) = Self::try_parse_string_literal(string)
        {
            return token;
        }

        // Identifier
        // keep incrementing the string up until we find another keyword
        let mut identifier_size: usize = 0;

        loop
        {
            // The string ends with an identifier
            if identifier_size > string.len()
            {
                return Ok(Token::Identifier(string));
            }

            // We found another identifier
            if string[..identifier_size].ends_with([' ', '\n'])
            {
                return Ok(Token::Identifier(&string[..identifier_size - 1]));
            }

            // There's an upcoming keyword, everything before is an identifier
            if Self::try_get_keyword(&string[identifier_size..]).is_some()
            {
                return Ok(Token::Identifier(&string[..identifier_size]));
            }

            // There's an upcoming literal, everything before is an identifier
            if Self::try_parse_string_literal(&string[identifier_size..]).is_some()
            {
                return Ok(Token::Identifier(&string[..identifier_size]));
            }

            identifier_size += 1;
        }
    }

    pub fn parse(raw_string: &str) -> Result<Vec<FileToken>, FileTokenizationError>
    {
        let mut output: Vec<FileToken> = Vec::new();
        let mut current_idx: usize = 0;

        let mut idx_of_current_line: usize = 0;
        let mut current_line_number: usize = 1;

        loop
        {
            if current_idx >= raw_string.len()
            {
                break;
            }

            if raw_string[idx_of_current_line..current_idx].contains('\n')
            {
                current_line_number += raw_string[idx_of_current_line..current_idx].matches('\n').count();
                idx_of_current_line = current_idx;
            }

            let column = match raw_string[..current_idx].rfind('\n')
            {
                Some(idx) => raw_string[..current_idx].len() - idx,
                None => current_idx + 1,
            };

            output.push(
                FileToken
                {
                    line: current_line_number,
                    column,
                    token: Token::find(&raw_string[current_idx..])
                        .map_err(|_| 
                            FileTokenizationError
                            {
                                line: current_line_number,
                                column,
                                error: TokenizationError::UnclosedDelimiter,
                            }
                        )?
                }
            );

            current_idx += output.last().unwrap().token.size();
            current_idx += raw_string[current_idx..].len() - raw_string[current_idx..].trim_start().len();
        }

        Ok(output)
    }
}