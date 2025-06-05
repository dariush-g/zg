use bevy::prelude::*;
use bevy::ui::Val;

use crate::gamestate::AppState;

pub struct CrosshairPlugin;
impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_crosshair);
    }
}

#[derive(Clone)]
pub enum CrossHairStyle {
    Cross,
    Dot,
}

#[derive(Component, Clone)]
pub struct Crosshair {
    pub color: Color,
    pub style: CrossHairStyle,
    pub size: f32,
}
#[allow(unused)]
impl Crosshair {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            size: 1.,
            style: CrossHairStyle::Dot,
        }
    }
}

fn spawn_crosshair(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Px(2.0),
            height: Val::Px(2.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            margin: UiRect {
                left: Val::Px(-1.),
                top: Val::Px(-1.),
                ..Default::default()
            },
            ..Default::default()
        },
        BackgroundColor(Color::WHITE),
    ));
}

//fn update_crosshair() {}
