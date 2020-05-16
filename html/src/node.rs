use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub enum Node<'a> {
  Document,
  Element {
    tag_name: &'a str,
    attributes: Vec<(&'a str, &'a str)>,
  },
  Text(&'a str),
  Comment(&'a str),
}
impl<'a> Node<'a> {
  pub fn is_empty(&self) -> bool {
    match self {
      Self::Document => false,
      Self::Element { tag_name, .. } => match *tag_name {
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "keygen" | "link"
        | "meta" | "param" | "source" | "track" | "wbr" => true,
        _ => false,
      },
      _ => true,
    }
  }
}
impl fmt::Display for Node<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Node::Element {
        tag_name,
        attributes,
        ..
      } => write!(f, "Element: {} ({} attributes)", tag_name, attributes.len()),
      Node::Document => write!(f, "/Document/"),
      Node::Comment(text) => write!(f, "Comment (length={})", text.len()),
      Node::Text(text) => write!(f, "Text (length={})", text.len()),
    }
  }
}

#[derive(Debug)]
pub enum Traversal<'a> {
  Up(usize),
  Down(usize),
  Node(&'a Node<'a>),
}
pub struct Traverse<'a> {
  document: &'a Document<'a>,
  stack: VecDeque<Traversal<'a>>,
}
impl<'a> Traverse<'a> {
  fn new(document: &'a Document) -> Self {
    Traverse {
      document,
      stack: vec![Traversal::Down(0)].into(),
    }
  }
}
impl<'a> Iterator for Traverse<'a> {
  type Item = Traversal<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(traversal) = self.stack.pop_front() {
      match traversal {
        Traversal::Down(parent_node_id) => {
          // Will go back up some time
          self.stack.push_front(Traversal::Up(parent_node_id));

          for (_, nid) in self
            .document
            .parents
            .iter()
            .filter(|(parent_id, _)| *parent_id == parent_node_id)
          {
            self.stack.push_front(Traversal::Down(*nid));
            self
              .stack
              .push_front(Traversal::Node(&self.document.nodes[*nid]));
          }
        }
        Traversal::Up(_) | Traversal::Node(_) => (),
      };

      Some(traversal)
    } else {
      None
    }
  }
}

#[derive(Debug, Default)]
pub struct Document<'a> {
  nodes: Vec<Node<'a>>,
  parents: Vec<(usize, usize)>,
}
impl<'a> Document<'a> {
  pub fn new(nodes: Vec<Node<'a>>, parents: Vec<(usize, usize)>) -> Self {
    Document { nodes, parents }
  }

  pub fn nodes(&self) -> &[Node] {
    &self.nodes
  }

  pub fn traverse(&self) -> Traverse {
    Traverse::new(self)
  }
}
