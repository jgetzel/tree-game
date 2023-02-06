use bevy::asset::LoadState;
use bevy::{prelude::*, utils::HashMap};
use crate::assets::AudioEnum::MusicMainTheme;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    InGame,
}

#[derive(Default, Resource, Clone)]
pub struct GameAssets {
    pub sprites: HashMap<SpriteEnum, Handle<Image>>,
    pub audio: HashMap<AudioEnum, Handle<AudioSource>>
}

impl GameAssets {
    pub fn get(&self, sprite: SpriteEnum) -> Handle<Image> {
        self.sprites.get(&sprite).unwrap().clone()
    }
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Loading)
            .insert_resource(GameAssets::default())
            .insert_resource(AssetsLoading::default())
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets))
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_assets_loaded),
            );
    }
}

#[derive(Default, Resource)]
pub struct AssetsLoading(Vec<HandleUntyped>);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Reflect)]
pub enum AudioEnum {
    MusicMainTheme
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Reflect)]
pub enum SpriteEnum {
    TrunkIdle1,
    TrunkIdle2,
    TrunkIdle3,
    TrunkWalk1,
    TrunkWalk2,
    TrunkWalk3,
    HouseFront,
    HouseFrontHouse,
    HouseFrontBackground,
    HouseFrontTree1,
    HouseFrontTree2,
    HouseInside,
    RoomBackground,
    RoomBackground2,
    RoomFurniture1,
    RoomFurniture2,
    RoomFurniture3,
    RoomSlidingDoor,
    RoomDoor,
    RoomBirdDoor,
    LadyIdle,
    MouseyIdle1,
    MouseyIdle2,
    MouseyIdle3,
    MouseyWalk1,
    MouseyWalk2,
    MouseyWalk3,
    MouseyWalk4,
    MouseyWalk5,
    Bug1,
    Bug2,
    Bug3,
    Bug4,
    TrashCan,
    Shadow,
    DebugCircle,
}

pub fn load_assets(
    mut assets: ResMut<GameAssets>,
    mut loading: ResMut<AssetsLoading>,
    asset_server: Res<AssetServer>,
) {
    assets.sprites.insert(
        SpriteEnum::HouseFront,
        asset_server.load("sprites/house_front.png"),
    );
    assets.sprites.insert(
        SpriteEnum::HouseFrontBackground,
        asset_server.load("sprites/house_front_background.png"),
    );
    assets.sprites.insert(
        SpriteEnum::HouseFrontHouse,
        asset_server.load("sprites/house_front_house.png"),
    );
    assets.sprites.insert(
        SpriteEnum::HouseFrontTree1,
        asset_server.load("sprites/house_front_tree_1.png"),
    );
    assets.sprites.insert(
        SpriteEnum::HouseFrontTree2,
        asset_server.load("sprites/house_front_tree_2.png"),
    );
    assets.sprites.insert(
        SpriteEnum::HouseInside,
        asset_server.load("sprites/house_inside.png"),
    );
    assets.sprites.insert(
        SpriteEnum::LadyIdle,
        asset_server.load("sprites/lady_idle.png"),
    );
    assets.sprites.insert(
        SpriteEnum::MouseyWalk1,
        asset_server.load("sprites/mousey/mouseywalk1.png"),
    );
    assets.sprites.insert(
        SpriteEnum::MouseyWalk2,
        asset_server.load("sprites/mousey/mouseywalk2.png"),
    );
    assets.sprites.insert(
        SpriteEnum::MouseyWalk3,
        asset_server.load("sprites/mousey/mouseywalk3.png"),
    );
    assets.sprites.insert(
        SpriteEnum::MouseyWalk4,
        asset_server.load("sprites/mousey/mouseywalk4.png"),
    );
    assets.sprites.insert(
        SpriteEnum::MouseyWalk5,
        asset_server.load("sprites/mousey/mouseywalk5.png"),
    );
    assets.sprites.insert(
        SpriteEnum::MouseyIdle1,
        asset_server.load("sprites/mousey/mousey_idle1.png"),
    );
    assets.sprites.insert(
        SpriteEnum::MouseyIdle2,
        asset_server.load("sprites/mousey/mousey_idle2.png"),
    );
    assets.sprites.insert(
        SpriteEnum::MouseyIdle3,
        asset_server.load("sprites/mousey/mousey_idle3.png"),
    );
    assets.sprites.insert(
        SpriteEnum::Bug1,
        asset_server.load("sprites/bug/buggyboo1.png"),
    );
    assets.sprites.insert(
        SpriteEnum::Bug2,
        asset_server.load("sprites/bug/buggyboo2.png"),
    );
    assets.sprites.insert(
        SpriteEnum::Bug3,
        asset_server.load("sprites/bug/buggyboo3.png"),
    );
    assets.sprites.insert(
        SpriteEnum::Bug4,
        asset_server.load("sprites/bug/buggyboo4.png"),
    );
    assets.sprites.insert(
        SpriteEnum::TrashCan,
        asset_server.load("sprites/trash_can.png"),
    );
    assets.sprites.insert(
        SpriteEnum::TrunkWalk1,
        asset_server.load("sprites/trunk/jr_walk1.png"),
    );
    assets.sprites.insert(
        SpriteEnum::TrunkWalk2,
        asset_server.load("sprites/trunk/jr_walk2.png"),
    );
    assets.sprites.insert(
        SpriteEnum::TrunkWalk3,
        asset_server.load("sprites/trunk/jr_walk3.png"),
    );
    assets
        .sprites
        .insert(SpriteEnum::Shadow, asset_server.load("sprites/shadow.png"));
    assets.sprites.insert(
        SpriteEnum::TrunkIdle1,
        asset_server.load("sprites/trunk/jr_idle1.png"),
    );
    assets.sprites.insert(
        SpriteEnum::TrunkIdle2,
        asset_server.load("sprites/trunk/jr_idle2.png"),
    );
    assets.sprites.insert(
        SpriteEnum::TrunkIdle3,
        asset_server.load("sprites/trunk/jr_idle3.png"),
    );
    assets.sprites.insert(
        SpriteEnum::RoomBackground,
        asset_server.load("sprites/room_background.png"),
    );
    assets.sprites.insert(
        SpriteEnum::RoomBackground2,
        asset_server.load("sprites/room_background2.png"),
    );
    assets.sprites.insert(
        SpriteEnum::RoomFurniture1,
        asset_server.load("sprites/room_furniture1.png"),
    );
    assets.sprites.insert(
        SpriteEnum::RoomFurniture2,
        asset_server.load("sprites/room_furniture2.png"),
    );
    assets.sprites.insert(
        SpriteEnum::RoomFurniture3,
        asset_server.load("sprites/room_furniture3.png"),
    );
    assets.sprites.insert(
        SpriteEnum::RoomSlidingDoor,
        asset_server.load("sprites/room_slidingdoor.png"),
    );
    assets.sprites.insert(
        SpriteEnum::RoomDoor,
        asset_server.load("sprites/room_door.png"),
    );
    assets.sprites.insert(
        SpriteEnum::RoomBirdDoor,
        asset_server.load("sprites/Room_birddoor.png"),
    );
    assets.sprites.insert(
        SpriteEnum::DebugCircle,
        asset_server.load("sprites/debug_circle.png"),
    );

    assets.audio.insert(
        MusicMainTheme,
        asset_server.load("audio/tree_game_theme.wav")
    );

    for (_, asset) in assets.sprites.iter() {
        loading.0.push(asset.clone_untyped());
    }
}

pub fn check_assets_loaded(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    match server.get_group_load_state(loading.0.iter().map(|handle| handle.id)) {
        LoadState::Failed => {}
        LoadState::Loaded => {
            commands.remove_resource::<AssetsLoading>();
            state.set(AppState::InGame).unwrap();
        }
        _ => {}
    }
}
