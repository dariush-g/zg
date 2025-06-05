pub mod controller;
pub mod player_data;
pub mod player_info;
pub mod player_stats;

use bevy::prelude::*;
use controller::ControllerPlugin;

use crate::ui::UiPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ControllerPlugin, UiPlugin));
    }
}
