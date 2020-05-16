use crate::node;
use crate::Result;
use pest::Parser;

#[derive(Parser)]
#[grammar = "html.pest"]
struct HtmlParser;

pub fn parse<'a>(text: &'a str) -> Result<node::Document<'a>> {
  let pairs = HtmlParser::parse(Rule::document, text)?;

  let mut nodes = vec![node::Node::Document];
  let mut element_id_stack: Vec<usize> = Vec::new();
  let mut parents: Vec<(usize, usize)> = Vec::new();

  macro_rules! current_node_id {
    () => {
      nodes.len() - 1
    };
  };

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

        let node = node::Node::Element {
          tag_name,
          attributes,
        };
        let is_empty = is_empty || node.is_empty();
        nodes.push(node);
        let node_id = current_node_id!();

        if is_empty {
          // Parent the node to 0 (document) if no nodes in the stack
          parents.push((
            *element_id_stack.last().expect("Expected a parent"),
            node_id,
          ));
        } else {
          element_id_stack.push(node_id);
        }
      }
      Rule::end_tag => {
        let mut inner_rules = pair.into_inner();
        let ended_tag_name = inner_rules.next().unwrap().as_str();

        // Get to the element that matches the ended tag name
        loop {
          match element_id_stack.pop().map(|i| (i, &nodes[i])) {
            Some((node_id, node::Node::Element { tag_name, .. })) => {
              parents.push((*element_id_stack.last().unwrap_or(&0), node_id));

              if *tag_name == ended_tag_name {
                break;
              }
            }
            n @ _ => panic!("Unknown node: {:?}", n),
          }
        }
      }
      Rule::text => {
        let text = pair.as_str();

        nodes.push(node::Node::Text(text));

        parents.push((
          *element_id_stack.last().expect("Expected node for text"),
          current_node_id!(),
        ));
      }
      Rule::comment => {
        let text = pair.into_inner().next().unwrap().as_str();

        nodes.push(node::Node::Comment(text));

        parents.push((
          *element_id_stack.last().expect("Expected node for comment"),
          current_node_id!(),
        ));
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

  // Turn the order around as the nodes get inserted backwards
  // because of the stack
  parents.reverse();

  Ok(node::Document::new(nodes, parents))
}
