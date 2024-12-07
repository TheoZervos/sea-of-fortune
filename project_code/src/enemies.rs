use bevy::prelude::*;

use crate::bat::components::*;
use crate::data::gameworld_data::*;
use crate::ghost_ship::components::*;
use crate::hitbox_system::components::*;
use crate::kraken::components::*;
use crate::player::components::*;
use crate::rock::components::*;
use crate::skeleton::components::*;
use crate::whirlpool::components::*;
use crate::whirlpool::components::Lifetime;
use crate::whirlpool::components::WHIRLPOOL_LIFETIME;

pub enum Enemy {
    Bat,
    Kraken,
    GhostShip,
    Rock,
    Skeleton,
    Skel2,
    Whirlpool,
}

pub fn spawn_enemy(
    commands: &mut Commands,
    enemy: Enemy,
    transform: Transform,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    match enemy {
        Enemy::Whirlpool => {
            let whirlpool_texture_asset: Handle<Image> = asset_server.load("s_whirlpool.png");

            commands.spawn((
                SpriteBundle{
                    texture: whirlpool_texture_asset,
                    transform,
                    ..default()
                },
                Whirlpool {
                    rotation_speed: f32::to_radians(90.0),
                    current_hp: WHIRLPOOL_HP,
                    max_hp: WHIRLPOOL_HP,
                },
                Lifetime(WHIRLPOOL_LIFETIME),
                Hurtbox {
                    size: Vec2::new(400., 290.),
                    offset: Vec2::splat(0.),
                    colliding: false,
                    entity: WHIRLPOOL,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                }, 
            ));
        },
        Enemy::Bat => {
            let bat_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 3, 1, None, None);

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("s_bat.png"),
                    transform,
                    ..default()
                },
                Bat {
                    //Setting default stats
                    rotation_speed: f32::to_radians(90.0),
                    current_hp: BAT_MAX_HP,
                    max_hp: BAT_MAX_HP,
                },
                TextureAtlas {
                    layout: texture_atlases.add(bat_layout.clone()),
                    index: 0,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                AnimationTimer::new(Timer::from_seconds(
                    BAT_ANIMATION_TIME,
                    TimerMode::Repeating,
                )),
                AnimationFrameCount::new(3),
                Velocity::new(),
                Hurtbox {
                    size: Vec2::splat(25.),
                    offset: Vec2::splat(0.),
                    colliding: false,
                    entity: BAT,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
            ));
        }
        Enemy::Kraken => {
            let kraken_texture_handle = asset_server.load("s_kraken.png");

            commands.spawn((
                SpriteBundle {
                    texture: kraken_texture_handle,
                    transform,
                    ..default()
                },
                Kraken {
                    //Setting default stats
                    rotation_speed: f32::to_radians(90.0),
                    current_hp: KRAKEN_MAX_HP,
                    max_hp: KRAKEN_MAX_HP,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                Hurtbox {
                    size: Vec2::new(160., 90.),
                    offset: Vec2::splat(0.),
                    colliding: false,
                    entity: KRAKEN,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
            ));
        }
        Enemy::GhostShip => {
            let ghostship_texture_handle = asset_server.load("s_ghost_ship.png");

            commands.spawn((
                SpriteBundle {
                    texture: ghostship_texture_handle,
                    transform,
                    ..default()
                },
                GhostShip {
                    //Setting default stats
                    rotation_speed: f32::to_radians(90.0),
                    current_hp: GHOSTSHIP_MAX_HP,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                Hurtbox {
                    size: Vec2::new(160., 90.),
                    offset: Vec2::splat(0.),
                    colliding: false,
                    entity: GHOSTSHIP,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
            ));
        }
        Enemy::Skeleton => {
            let skeleton_layout = TextureAtlasLayout::from_grid(
                UVec2::new(31, 32), // SpriteSheet 1 pixel off, maybe fix later? it works like this though
                6,                  // Columns
                1,                  // Rows
                None,               // Padding
                None,               // Spacing
            );

            // Add the texture atlas to the resource

            // Spawn the skeleton entity
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("s_skeleton.png"), // This uses the TextureAtlas handle
                    transform,
                    ..default()
                },
                Skeleton {
                    rotation_speed: 0.0,
                    current_hp: SKELETON_MAX_HP,
                    max_hp: SKELETON_MAX_HP,
                },
                TextureAtlas {
                    layout: texture_atlases.add(skeleton_layout.clone()),
                    index: 0,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                AnimationTimer::new(Timer::from_seconds(
                    SKELETON_ANIMATION_TIME,
                    TimerMode::Repeating,
                )),
                AnimationFrameCount::new(6),
                Velocity::new(),
                Hurtbox {
                    size: Vec2::new(32., 32.), // Adjust as needed
                    offset: Vec2::ZERO,
                    colliding: false,
                    entity: SKELETON,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
            ));
        }
        Enemy::Rock => {
            let rock_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE * 2), 2, 1, None, None);

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("s_rock.png"),
                    transform,
                    ..default()
                },
                Rock {
                    //Setting default stats
                    current_hp: ROCK_MAX_HP,
                },
                TextureAtlas {
                    layout: texture_atlases.add(rock_layout.clone()),
                    index: 0,
                },
                AnimationTimer::new(Timer::from_seconds(
                    ROCK_ANIMATION_TIME,
                    TimerMode::Repeating,
                )),
                AnimationFrameCount::new(2),
                Velocity::new(),
                Hurtbox {
                    size: Vec2::splat(50.),
                    offset: Vec2::splat(0.),
                    colliding: false,
                    entity: ROCK,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
                Hitbox {
                    size: Vec2::splat(40.),
                    offset: Vec2::splat(0.),
                    lifetime: Some(Timer::from_seconds(1000000., TimerMode::Repeating)),
                    projectile: false,
                    entity: ROCK,
                    enemy: true,
                },
            ));
        }
        Enemy::Skel2 => {}
    }
}
