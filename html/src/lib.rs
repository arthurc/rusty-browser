#[macro_use]
extern crate pest_derive;

pub mod dom;
pub mod prelude;

use pest::Parser;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Pest(#[from] pest::error::Error<Rule>),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[grammar = "html.pest"]
struct HtmlParser;

pub fn parse<'a>(text: &'a str) -> Result<dom::Document<'a>> {
  let pairs = HtmlParser::parse(Rule::document, text)?;

  let mut node_stack: Vec<dom::Node> = vec![];
  let mut document = dom::Document::new();

  for pair in pairs {
    match pair.as_rule() {
      Rule::tag => {
        let mut inner_rules = pair.into_inner();
        let tag_name = inner_rules
          .next()
          .unwrap()
          .into_inner()
          .next()
          .unwrap()
          .as_str();
        let attributes = inner_rules
          .next()
          .map(|rule| {
            rule
              .into_inner()
              .map(|pair| {
                let mut inner_rules = pair.into_inner();

                (
                  inner_rules.next().unwrap().as_str(),
                  inner_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str(),
                )
              })
              .collect()
          })
          .unwrap_or(vec![]);
        let is_empty = inner_rules.next().unwrap().as_str() == "/>";

        let node = dom::Node::Element {
          tag_name,
          children: Vec::new(),
          html_element: dom::HtmlElement::new(tag_name),
        };

        if is_empty || node.is_empty() {
          if let Some(parent) = node_stack.last_mut() {
            parent.append_child(node);
          } else {
            document.append_child(node);
          }
        } else {
          node_stack.push(node);
        }
      }
      Rule::end_tag => {
        let mut inner_rules = pair.into_inner();
        let ended_tag_name = inner_rules.next().unwrap().as_str();

        // Get to the element that matches the ended tag name
        loop {
          match node_stack.pop() {
            Some(dom::Node::Element {
              tag_name,
              children,
              html_element,
            }) => {
              if let Some(parent) = node_stack.last_mut() {
                parent.append_child(dom::Node::Element {
                  tag_name,
                  children,
                  html_element,
                });
              } else {
                document.append_child(dom::Node::Element {
                  tag_name,
                  children,
                  html_element,
                });
              }

              if tag_name == ended_tag_name {
                break;
              }
            }
            None => break,
            n @ _ => panic!("Unknown node: {:?}", n),
          }
        }
      }
      Rule::text => {
        let text = pair.as_str();

        if let Some(node) = node_stack.last_mut() {
          node.append_child(dom::Node::Text(text));
        } else {
          println!("Expected node for text={}", text);
        }
      }
      Rule::comment => {
        let text = pair.into_inner().next().unwrap().as_str();

        if let Some(node) = node_stack.last_mut() {
          node.append_child(dom::Node::Comment(text));
        } else {
          println!("Expected node for comment={}", text);
        }
      }
      Rule::doctype => {}
      Rule::EOI => break,
      Rule::document
      | Rule::ident
      | Rule::string
      | Rule::string_text
      | Rule::string_char
      | Rule::text_char
      | Rule::comment_text
      | Rule::comment_char
      | Rule::attribute
      | Rule::attributes
      | Rule::tag_start
      | Rule::tag_ending
      | Rule::unquoted_string
      | Rule::quoted_string
      | Rule::WHITESPACE => unreachable!(),
    }
  }

  Ok(document)
}
