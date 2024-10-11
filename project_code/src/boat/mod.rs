use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
use crate::GameworldState;
use crate::player::systems::despawn_player;

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    /// Builds the boat plugin
    fn build(&self, app: &mut App) {
            app
                .add_systems(OnEnter(GameworldState::Ocean),spawn_boat.after(despawn_player))
                .add_systems(Update,move_boat.run_if(in_state(GameworldState::Ocean)))
                .add_systems(OnExit(GameworldState::Ocean), despawn_boat);
    }
}