use bevy::app::Plugin;
use crosshair::CrosshairPlugin;
use settings::fps::FPSDisplayPlugin;

pub mod crosshair;
pub mod settings;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(CrosshairPlugin)
            .add_plugins(FPSDisplayPlugin);
    }
}
