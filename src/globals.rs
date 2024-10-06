// Window dimensions
pub const WINDOW_WIDTH: f32 = 1280.0; // Width of the window
pub const WINDOW_HEIGHT: f32 = 720.0; // Height of the window

// Target pixel settings
pub const TARGET_PIXEL_WIDTH: f32 = 320.0; // Target pixel width for scaling

// Tile settings
pub const TILE_SIZE: u32 = 16; // Size of each tile in pixels

// Z-index settings for rendering order
pub const PLAYER_Z_INDEX: f32 = 10.0; // Z-index for player entities
pub const BOARD_Z_INDEX: f32 = 1.0; // Z-index for the game board
pub const HIGHLIGHT_Z_INDEX: f32 = 2.0; // Z-index for highlighted tiles

// Board settings
pub const BOARD_SIZE: i32 = 8; // Size of the game board (8x8)

// Animation speeds
pub const TWEEN_MOVE_ANIMATION_SPEED: f32 = 10.0; // Speed of tween move animations
pub const PULSE_ANIMATION_SPEED: f32 = 4.0; // Speed of pulse animations

// Player settings
pub const PLAYER_HEALTH: u64 = 100; // Health of the player
pub const PLAYER_DAMAGE: u64 = 1; // Damage of the player

// Enemy settings
pub const PAWN_HEALTH: u64 = 2; // Health of the enemy
pub const PAWN_DAMAGE: u64 = 1; // Damage of the enemy
