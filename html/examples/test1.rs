use html;

fn main() -> html::Result<()> {
  let document = html::parse(std::str::from_utf8(include_bytes!("test1.html")).unwrap())?;

  document.dump();

  Ok(())
}
