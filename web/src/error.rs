#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  HtmlError(#[from] html::Error),
  #[error(transparent)]
  IoError(#[from] std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
