mod bat;
mod boat;
mod components;
mod controls;
mod data;
mod enemies;
mod ghost_ship;
mod hitbox_system;
mod kraken;
mod level;
mod player;
mod rock;
mod shop;
mod skeleton;
mod systems;
mod transition_box;
mod wfc;
mod whirlpool;
mod wind;

use bat::BatPlugin;
use bevy::{prelude::*, window::PresentMode};
use boat::systems::move_boat;
use boat::BoatPlugin;
use components::GameState;
use components::GameworldState;
use components::SpawnLocations;
use controls::*;
use data::gameworld_data::*;
use enemies::*;
use ghost_ship::GhostShipPlugin;
use hitbox_system::HitboxPlugin;
use kraken::KrakenPlugin;
use level::LevelPlugin;
use player::systems::move_player;
use player::systems::spawn_player;
use player::PlayerPlugin;
use rock::RockPlugin;
use shop::ShopPlugin;
use skeleton::SkeletonPlugin;
use systems::*;
use wfc::WFCPlugin;
use whirlpool::WhirlpoolPlugin;
use wind::WindPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sea of Fortune | Build 0.2".into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CurrMousePos>()
        .add_systems(Startup, setup_gameworld)
        .add_plugins(PlayerPlugin)
        .add_plugins(BoatPlugin)
        .add_plugins(BatPlugin)
        .add_plugins(KrakenPlugin)
        .add_plugins(SkeletonPlugin)
        .add_plugins(HitboxPlugin)
        .add_plugins(ShopPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(WFCPlugin)
        .add_plugins(GhostShipPlugin)
        .add_plugins(RockPlugin)
        .add_plugins(WindPlugin)
        .add_plugins(WhirlpoolPlugin)
        .add_systems(
            Update,
            move_player_camera.after(move_player).run_if(
                in_state(GameworldState::Island).or_else(in_state(GameworldState::Dungeon)),
            ),
        )
        .add_systems(
            Update,
            move_boat_camera
                .after(move_boat)
                .run_if(in_state(GameworldState::Ocean)),
        )
        .add_systems(Update, change_gameworld_state)
        .add_systems(Update, change_game_state)
        .add_systems(Update, update_mouse_pos)
        .add_systems(Update, check_wall_collisions.after(move_player))
        .add_systems(
            OnEnter(GameworldState::Dungeon),
            handle_dungeon_entry.after(spawn_player),
        )
        .add_systems(OnEnter(GameworldState::Dungeon), handle_door_translation)
        .add_systems(OnEnter(GameworldState::Island), handle_door_translation)
        .add_systems(Update, update_dungeon_collision)
        .insert_state(GameworldState::MainMenu)
        .insert_state(GameState::Running)
        .insert_resource(SpawnLocations::default())
        .run();
}
