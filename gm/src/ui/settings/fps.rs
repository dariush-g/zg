use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};
pub struct FPSDisplayPlugin;

impl Plugin for FPSDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 15.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                    ..Default::default()
                },
                enabled: true,
                ..Default::default()
            },
        });
    }
}

