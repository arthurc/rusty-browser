use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  HtmlError(#[from] html::Error),
  #[error(transparent)]
  UrlParseError(#[from] web::url::ParseError),
  #[error(transparent)]
  AmethystError(#[from] AmethystError),
}
impl From<amethyst::Error> for Error {
  fn from(error: amethyst::Error) -> Self {
    Error::AmethystError(AmethystError(error))
  }
}

#[derive(Debug)]
pub struct AmethystError(amethyst::Error);
impl fmt::Display for AmethystError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
impl std::error::Error for AmethystError {}

pub type Result<T> = std::result::Result<T, Error>;
