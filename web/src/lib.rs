mod bundle;
mod error;
mod layout;
mod system;

pub use bundle::WebBundle;
pub use url;
pub use url::Url;

pub use error::Error;
pub use error::Result;

use std::io::prelude::*;

pub enum WebEvent {
  Url(Url),
}

pub fn load_source(url: &Url) -> Result<String> {
  match url.scheme() {
    "file" => {
      let mut source = String::new();
      std::io::BufReader::new(std::fs::File::open(url.path())?).read_to_string(&mut source)?;
      return Ok(source);
    }
    _ => unimplemented!(),
  }
}
