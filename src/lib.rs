use amethyst::{
  core::transform::TransformBundle,
  input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
  prelude::*,
  renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
  },
};
use log::info;
use std::{fmt, io::prelude::*};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  HtmlError(#[from] html::Error),
  #[error(transparent)]
  UrlParseError(#[from] web::url::ParseError),
  #[error(transparent)]
  AmethystError(#[from] AmethystError),
}
impl From<amethyst::Error> for Error {
  fn from(error: amethyst::Error) -> Self {
    Error::AmethystError(AmethystError(error))
  }
}

#[derive(Debug)]
pub struct AmethystError(amethyst::Error);
impl fmt::Display for AmethystError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
impl std::error::Error for AmethystError {}

pub type Result<T> = std::result::Result<T, Error>;

struct BrowserState {}
impl SimpleState for BrowserState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    //world.register();

    //world.create_entity().with().build();
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

  let url = web::Url::from_directory_path(std::env::current_dir().unwrap()).unwrap();
  let url = url.join(&std::env::args().nth(1).unwrap())?;

  let html_view = web::HtmlView::new();
  html_view.load(&url);

  let mut source = String::new();
  match url.scheme() {
    "file" => {
      std::io::BufReader::new(std::fs::File::open(url.path()).unwrap())
        .read_to_string(&mut source)
        .unwrap();
    }
    _ => unimplemented!(),
  }

  let document = html::parse(&source).unwrap();
  let mut game = init_game(BrowserState {})?;
  game.run();

  Ok(())
}

fn init_game<'a>(state: BrowserState) -> Result<Application<'a, amethyst::GameData<'a, 'a>>> {
  let mut display_config: amethyst::window::DisplayConfig = Default::default();
  display_config.title = "Browser".to_owned();

  let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config(display_config).with_clear([1.0, 1.0, 1.0, 1.0]))
        .with_plugin(RenderFlat2D::default()),
    )?;

  Ok(Application::new(".", state, game_data)?)
}
