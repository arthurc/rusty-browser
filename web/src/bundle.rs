use crate::layout::Parent;
use crate::system::WebSystemDesc;
use amethyst::core::SystemBundle;
use amethyst::core::SystemDesc;
use amethyst::ecs::prelude::*;
use specs_hierarchy::HierarchySystem;

#[derive(Debug, Default)]
pub struct WebBundle<'a> {
  dep: &'a [&'a str],
}
impl<'a> WebBundle<'a> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn with_dep(mut self, dep: &'a [&'a str]) -> Self {
    self.dep = dep;
    self
  }
}

impl<'a, 'b, 'c> SystemBundle<'a, 'b> for WebBundle<'c> {
  fn build(
    self,
    world: &mut World,
    builder: &mut DispatcherBuilder<'_, '_>,
  ) -> Result<(), amethyst::error::Error> {
    builder.add(
      HierarchySystem::<Parent>::new(world),
      "hierarchy_system",
      self.dep,
    );
    builder.add(WebSystemDesc::default().build(world), "web_system", &[]);
    Ok(())
  }
}
