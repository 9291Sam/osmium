pub mod tokenizer;

pub enum CompileError
{
    UnclosedDelimiter
}

impl From<tokenizer::TokenizationError> for CompileError
{
    fn from(e: tokenizer::TokenizationError) -> Self {
        match e
        {
            tokenizer::TokenizationError::UnclosedDelimiter => Self::UnclosedDelimiter,
        }
    }
}

impl std::fmt::Debug for CompileError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnclosedDelimiter => write!(f, "UnclosedDelimiter"),
        }
    }
}