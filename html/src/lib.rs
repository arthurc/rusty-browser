#[macro_use]
extern crate pest_derive;

mod node;
mod parser;

pub use node::{Node, Traversal};
pub use parser::parse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Pest(#[from] pest::error::Error<parser::Rule>),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
