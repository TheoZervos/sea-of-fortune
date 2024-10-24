use bevy::prelude::*;
use crate::shop::components::Inventory;

/// Struct representing the player
#[derive(Component)]
pub struct Player {
    pub animation_state: SpriteState,
    pub timer: Timer,
    pub health: f32,
    pub max_health: f32,
    pub inventory: Inventory,
    pub spawn_position: Vec3,
}

/// Struct representing the sword weapon for the player
#[derive(Component)]
pub struct Sword {
    pub damage: f32,
    pub upgraded: bool,
}

//implementing sword
impl Default for Sword {
    /// Sets default values for the sword
    fn default() -> Sword {
        Sword {
            damage: 1.,         //unupgraded damage
            upgraded: false,    //upgraded damage
        }
    }
}

/// Velocity struct
#[derive(Component)]
pub struct Velocity {
    pub v: Vec2,
}

/// Struct for tracking players last position for out of transition spawning
pub struct PlayerLastPosition {
    pub last_pos: Vec2,
}

/// Velocity implementation
impl Velocity {
    pub fn new() -> Self {
        Self {
            //sets x and y velocity to 0
            v: Vec2::splat(0.),
        }
    }
}

/// Struct for the time between frames of animation
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    /// Initializes the animation timer
    pub fn new(timer: Timer) -> AnimationTimer {
        AnimationTimer(timer)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct TestTimer(Timer);

impl TestTimer {
    pub fn new(timer: Timer) -> TestTimer {
        TestTimer(timer)
    }
}

/// Struct for the count of frames in the players animation
#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

impl AnimationFrameCount {
    /// Initializes the animation frame count
    pub fn new(size: usize) -> AnimationFrameCount {
        AnimationFrameCount(size)
    }
}

/// Struct that represents the current sprite for the players state
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpriteState {
    Idle,
    LeftRun,
    RightRun,
    BackwardRun,
    ForwardRun,
}

impl SpriteState {
    /// Matches the player animation to the current player
    /// state
    pub fn animation_indices(&self) -> std::ops::Range<usize> {
        match self {
            SpriteState::Idle => 0..8,
            SpriteState::LeftRun => 8..16,
            SpriteState::RightRun => 16..24,
            SpriteState::ForwardRun => 24..32,
            SpriteState::BackwardRun => 32..40,
        }
    }

    /// Matches the speed of animation to the animation being
    /// played
    pub fn animation_speed(&self) -> f32 {
        match self {
            SpriteState::Idle => 0.1,
            SpriteState::LeftRun => 0.1,
            SpriteState::RightRun => 0.1,
            SpriteState::BackwardRun => 0.1,
            SpriteState::ForwardRun => 0.1,
        }
    }
}

///Struct that keeps track of the cooldown between attacks
#[derive(Component)]
pub struct AttackCooldown {
    pub remaining: Timer,
}
