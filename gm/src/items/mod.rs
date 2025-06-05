use bevy::prelude::*;

pub mod clothing;
pub mod consumable;
pub mod weapons;

#[derive(Component)]
pub struct Item {
    pub name: &'static str,
    pub item_type: ItemType,
    pub item_info: ItemInfo,
}

impl Item {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        parent: &Entity,
    ) -> Entity {
        commands
            .spawn((
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset(self.item_info.model_path)),
                ),
                Transform::from_xyz(1.2, -1.5, -1.9).with_scale(Vec3::new(0.7, 0.7, 0.7)),
            ))
            .set_parent(*parent)
            .id()
    }
}

pub struct ItemInfo {
    pub model_path: &'static str,
    pub sound_path: &'static str,
    pub icon_path: &'static str,
}

#[derive(Clone, PartialEq)]
#[allow(warnings)]
pub enum ItemType {
    Firearm {
        damage: f32,
        mag_size: u32,
        fire_rate: f32,
        durability: f32,
    },
    Melee {
        damage: f32,
        attack_rate: f32,
        durability: f32,
    },
    Consumable {
        saturation: Option<f32>,
        healing: Option<f32>,
    },
    Ammunition,
}
