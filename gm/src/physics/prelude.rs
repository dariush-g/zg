pub use super::{bodies::*, collisions::*, joints::*};
use bevy::app::App;

pub struct ZphyPlugin;
impl bevy::app::Plugin for ZphyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CollisionPlugin, RigidBodyPlugin, JointPlugin));
    }
}
