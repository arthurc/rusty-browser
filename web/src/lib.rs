mod layout;

pub use url;
pub use url::Url;

use std::io::prelude::*;

pub struct HtmlView {}
impl HtmlView {
  pub fn new() -> Self {
    HtmlView {}
  }

  pub fn load(&self, url: &Url) {
    println!("Loading {}", url);
  }
}
