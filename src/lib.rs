pub mod tokenizer;
pub mod lexer;

// pub enum CompileError
// {
//     UnclosedDelimiter
// }

// impl From<tokenizer::TokenizationError> for CompileError
// {
//     fn from(e: tokenizer::TokenizationError) -> Self {
//         match e
//         {
//             tokenizer::TokenizationError::UnclosedDelimiter => Self::UnclosedDelimiter,
//             tokenizer::TokenizationError::FileEndedWithIdentifier => todo!(),
//         }
//     }
// }

// impl std::fmt::Debug for CompileError
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::UnclosedDelimiter => write!(f, "UnclosedDelimiter"),
//         }
//     }
// }