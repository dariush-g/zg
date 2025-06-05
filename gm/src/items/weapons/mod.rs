use super::Item;

pub mod reload;

pub const PISTOL: Item = Item {
    name: "Pistol",
    item_type: super::ItemType::Firearm {
        damage: 10.,
        mag_size: 10,
        fire_rate: 1.,
        durability: 100.,
    },
    item_info: super::ItemInfo {
        model_path: "",
        sound_path: "",
        icon_path: "",
    },
};
