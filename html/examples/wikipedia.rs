fn main() -> html::Result<()> {
  let document = html::parse(std::str::from_utf8(include_bytes!("Wikipedia.html")).unwrap())?;

  let mut indent = 0;
  for traversal in document.traverse() {
    match traversal {
      html::Traversal::Up(_) => indent -= 1,
      html::Traversal::Down(_) => indent += 1,
      html::Traversal::Node(node) => println!(
        "{}{}",
        std::iter::repeat(" ").take(4 * indent).collect::<String>(),
        node
      ),
    }
  }

  Ok(())
}
