pub mod connection;
pub mod gamestate;
pub mod items;
pub mod physics;
pub mod player;
pub mod ui;
use bevy::{prelude::*, window::PresentMode};
use connection::join::MPlayerPlugin;
use gamestate::GameStatePlugin;
use physics::prelude::ZphyPlugin;
use player::PlayerPlugin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "".to_string(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MPlayerPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(ZphyPlugin)
        .add_plugins(PlayerPlugin)
        .run();
    Ok(())
}
