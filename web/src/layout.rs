use amethyst::ecs::prelude::*;
use specs_hierarchy::Parent as HParent;

#[derive(Debug)]
pub struct Parent(Entity);

impl HParent for Parent {
  fn parent_entity(&self) -> Entity {
    self.0
  }
}

impl Component for Parent {
  type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
