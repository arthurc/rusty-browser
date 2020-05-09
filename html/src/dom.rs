#[derive(Debug)]
pub enum Node<'a> {
  Element {
    tag_name: &'a str,
    children: Vec<Node<'a>>,
    html_element: HtmlElement<'a>,
  },
  Text(&'a str),
  Comment(&'a str),
}
impl<'a> Node<'a> {
  pub fn append_child(&mut self, child: Node<'a>) {
    match self {
      Self::Element { children, .. } => children.push(child),
      Self::Text(_) | Self::Comment(_) => {}
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      Self::Element { tag_name, .. } => match *tag_name {
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "keygen" | "link"
        | "meta" | "param" | "source" | "track" | "wbr" => true,
        _ => false,
      },
      _ => true,
    }
  }

  pub fn dump(&self, indent: usize) {
    print!(
      "{}",
      std::iter::repeat(" ").take(indent * 4).collect::<String>()
    );

    match self {
      Self::Element {
        tag_name, children, ..
      } => {
        println!("Element: {}", tag_name);
        for child in children {
          child.dump(indent + 1);
        }
      }
      Self::Text(text) => println!("Text: {}", text),
      Self::Comment(text) => println!("Comment: {}", text),
    }
  }
}

#[derive(Debug)]
pub enum HtmlElement<'a> {
  HtmlElement,
  HeadElement,
  MetaElement,
  TitleElement,
  ScriptElement,
  LinkElement,
  StyleElement,
  BodyElement,
  HeadingElement(&'a str),
  ImageElement,
  DivElement,
  StrongElement,
  AnchorElement,
  SmallElement,
  Element,
  SpanElement,
  FormElement,
  FieldSetElement,
  InputElement,
  LabelElement,
  SelectElement,
  OptionElement,
  ButtonElement,
  UListElement,
  LiElement,
  HrElement,
  ParagraphElement,
}
impl<'a> HtmlElement<'a> {
  pub fn new(tag_name: &'a str) -> Self {
    match tag_name {
      "html" => Self::HtmlElement,
      "head" => Self::HeadElement,
      "meta" => Self::MetaElement,
      "title" => Self::TitleElement,
      "script" => Self::ScriptElement,
      "link" => Self::LinkElement,
      "style" => Self::StyleElement,
      "body" => Self::BodyElement,
      "h1" | "h2" => Self::HeadingElement(tag_name),
      "img" => Self::ImageElement,
      "div" => Self::DivElement,
      "strong" => Self::StrongElement,
      "a" => Self::AnchorElement,
      "small" => Self::SmallElement,
      "bdi" | "i" => Self::Element,
      "span" => Self::SpanElement,
      "form" => Self::FormElement,
      "fieldset" => Self::FieldSetElement,
      "input" => Self::InputElement,
      "label" => Self::LabelElement,
      "select" => Self::SelectElement,
      "option" => Self::OptionElement,
      "button" => Self::ButtonElement,
      "ul" => Self::UListElement,
      "li" => Self::LiElement,
      "hr" => Self::HrElement,
      "p" => Self::ParagraphElement,
      _ => panic!("Unknown tag={}", tag_name),
    }
  }
}

#[derive(Default)]
pub struct Document<'a> {
  children: Vec<Node<'a>>,
}
impl<'a> Document<'a> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn append_child(&mut self, child: Node<'a>) {
    self.children.push(child)
  }

  pub fn dump(&self) {
    println!("Document");
    for child in &self.children {
      child.dump(1);
    }
  }
}
