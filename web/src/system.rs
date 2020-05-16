use crate::load_source;
use crate::WebEvent;
use amethyst::core::shrev::EventChannel;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::window::ScreenDimensions;

#[derive(SystemDesc)]
#[system_desc(name(WebSystemDesc))]
pub struct WebSystem {
  #[system_desc(event_channel_reader)]
  web_event_channel: ReaderId<WebEvent>,
}
impl WebSystem {
  pub fn new(web_event_channel: ReaderId<WebEvent>) -> Self {
    Self { web_event_channel }
  }

  fn handle_event(
    &mut self,
    event: &WebEvent,
    entities: &Entities,
    mut transforms: &mut WriteStorage<'_, Transform>,
  ) {
    match event {
      WebEvent::Url(url) => {
        let source = load_source(&url).unwrap();
        let document = html::parse(&source).unwrap();

        for node in document.nodes() {
          let entity = entities
            .build_entity()
            .with(Transform::default(), &mut transforms)
            .build();
        }
      }
    };
  }
}

impl<'a> amethyst::ecs::System<'a> for WebSystem {
  type SystemData = (
    Read<'a, EventChannel<WebEvent>>,
    WriteStorage<'a, Transform>,
    Entities<'a>,
  );

  fn run(&mut self, (web_event_channel, mut transforms, entities): Self::SystemData) {
    for event in web_event_channel.read(&mut self.web_event_channel) {
      self.handle_event(event, &entities, &mut transforms);
    }
  }
}
