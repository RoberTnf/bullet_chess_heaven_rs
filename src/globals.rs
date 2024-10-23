// Window dimensions
pub const WINDOW_WIDTH: f32 = 1280.0; // Width of the window
pub const WINDOW_HEIGHT: f32 = 720.0; // Height of the window

// Target pixel settings
pub const TARGET_PIXEL_WIDTH: f32 = 320.0; // Target pixel width for scaling

// Tile settings
pub const TILE_SIZE: u32 = 20; // Size of each tile in pixels

// Z-index settings for rendering order
pub const PLAYER_Z_INDEX: f32 = 20.0; // Z-index for player entities
pub const ENEMY_Z_INDEX: f32 = 19.0; // Z-index for enemy entities
pub const BOARD_Z_INDEX: f32 = 1.0; // Z-index for the game board
pub const HIGHLIGHT_Z_INDEX: f32 = 2.0; // Z-index for highlighted tiles
pub const HEALTH_Z_INDEX: f32 = 25.0; // Z-index for health bars
pub const HEALTH_CHANGE_TEXT_Z_INDEX: f32 = 26.0; // Z-index for health change text
pub const EMPTY_HEALTHBAR_Z_INDEX: f32 = 29.0; // Z-index for empty healthbars
pub const HEALTHBAR_Z_INDEX: f32 = 30.0; // Z-index for healthbars

pub const BOARD_SIZE: i32 = 8; // Size of the game board (8x8)

// Animation speeds
pub const TWEEN_MOVE_ANIMATION_SPEED: f32 = 10.0; // Speed of tween move animations
pub const PULSE_ANIMATION_SPEED: f32 = 4.0; // Speed of pulse animations
pub const HEALTH_CHANGE_TEXT_ANIMATION_SPEED: f32 = 15.0; // Speed of health change text animations

// Animation durations
pub const HEALTH_CHANGE_TEXT_ANIMATION_DURATION: f32 = 0.5; // Duration of health change text animations
pub const DEATH_ANIMATION_DURATION: f32 = 0.5; // Duration of death animations
pub const ATTACK_ANIMATION_DURATION: f32 = 0.5; // Duration of attack animations

// Player settings
pub const PLAYER_HEALTH: u64 = 100; // Health of the player
pub const PLAYER_DAMAGE: u64 = 1; // Damage of the player

// Enemy settings
pub const PAWN_HEALTH: u64 = 2; // Health of the enemy
pub const PAWN_DAMAGE: u64 = 1; // Damage of the enemy

// Spritesheet settings
pub const HIGHLIGHT_ATLAS_INDEX: usize = 3; // Index of the highlight sprite in the spritesheet
pub const HIGHLIGHT_ATTACK_ATLAS_INDEX: usize = 6; // Index of the highlight attack sprite in the spritesheet
pub const FULL_HEALTHBAR_ATLAS_INDEX: usize = 7; // Index of the full healthbar sprite in the spritesheet
pub const EMPTY_HEALTHBAR_ATLAS_INDEX: usize = 8; // Index of the empty healthbar sprite in the spritesheet

// Spawner settings
pub const TARGET_NUM_ENEMIES: usize = 10; // Max number of enemies on the board
pub const PER_TURN_ENEMY_SPAWN_COUNT: usize = 2; // Number of enemies to spawn per turn

// UI Settings
pub const UI_FONT_SIZE: f32 = 4.0; // Font size for UI elements
pub const UI_FONT: &str = "fonts/Press_Start_2P/PressStart2P-Regular.ttf"; // Font for UI elements
pub const UI_HEADER_FONT_SIZE: f32 = UI_FONT_SIZE * 1.2; // Font size for UI headers
