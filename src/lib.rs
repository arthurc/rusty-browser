mod error;

use amethyst::core::shrev::EventChannel;
use amethyst::{
  core::transform::{Transform, TransformBundle},
  input::{get_key, is_close_requested, is_key_down, InputBundle, StringBindings, VirtualKeyCode},
  prelude::*,
  renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
  },
  ui::UiBundle,
};
use log::info;
use web::WebBundle;

pub use error::{Error, Result};

#[derive(Debug)]
struct BrowserState {}
impl SimpleState for BrowserState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let url = web::Url::from_directory_path(std::env::current_dir().unwrap()).unwrap();
    let url = url.join(&std::env::args().nth(1).unwrap()).unwrap();

    data
      .world
      .fetch_mut::<EventChannel<web::WebEvent>>()
      .single_write(web::WebEvent::Url(url))
  }

  fn handle_event(
    &mut self,
    mut _data: StateData<'_, GameData<'_, '_>>,
    event: StateEvent,
  ) -> SimpleTrans {
    if let StateEvent::Window(event) = &event {
      if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
        return Trans::Quit;
      }

      if let Some(event) = get_key(&event) {
        info!("handling key event: {:?}", event);
      }
    }

    Trans::None
  }
}

pub fn run() -> Result<()> {
  amethyst::start_logger(Default::default());

  let mut game = init_game(BrowserState {})?;
  game.run();

  Ok(())
}

fn init_game<'a>(state: BrowserState) -> Result<Application<'a, amethyst::GameData<'a, 'a>>> {
  let mut display_config: amethyst::window::DisplayConfig = Default::default();
  display_config.title = "Browser".to_owned();

  let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(InputBundle::<StringBindings>::new())?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config(display_config).with_clear([1.0, 1.0, 1.0, 1.0]))
        .with_plugin(RenderFlat2D::default()),
    )?
    .with_bundle(WebBundle::new())?;

  Ok(Application::new(".", state, game_data)?)
}
