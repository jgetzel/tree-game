use bevy::{prelude::*, utils::HashMap};


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

#[derive(PartialEq, Eq, Hash)]
pub enum SpriteEnum {
    TrunkJr
}

#[derive(Default, Resource)]
pub struct GameAssets {
    pub map: HashMap<SpriteEnum, Handle<Image>>
}

#[derive(Default, Resource)]
pub struct AssetsLoading(Vec<HandleUntyped>);

pub fn load_assets(
    mut assets: ResMut<GameAssets>,
    mut loading: ResMut<AssetsLoading>,
    asset_server: Res<AssetServer>,
) {
    assets.map.insert(SpriteEnum::TrunkJr, asset_server.load("sprites/trunk-jr.png"));

    for (_, asset) in assets.map.iter() {
        loading.0.push(asset.clone_untyped());
    }
}