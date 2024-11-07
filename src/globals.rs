use bevy::prelude::*;

// Window dimensions
pub const WINDOW_WIDTH: f32 = 1280.0; // Width of the window
pub const WINDOW_HEIGHT: f32 = 720.0; // Height of the window

// Target pixel settings
pub const TARGET_PIXEL_WIDTH: f32 = 320.0; // Target pixel width for scaling
pub const TARGET_PIXEL_HEIGHT: f32 = 180.0; // Target pixel height for scaling

// Tile settings
pub const TILE_SIZE: usize = 20; // Size of each tile in pixels

// Z-index settings for rendering order
pub const BOARD_Z_INDEX: f32 = 1.0; // Z-index for the game board
pub const HIGHLIGHT_Z_INDEX: f32 = 2.0; // Z-index for highlighted tiles
pub const ENEMY_Z_INDEX: f32 = 19.0; // Z-index for enemy entities
pub const PLAYER_Z_INDEX: f32 = 20.0; // Z-index for player entities
pub const HEALTH_Z_INDEX: f32 = 25.0; // Z-index for health bars
pub const HEALTH_CHANGE_TEXT_Z_INDEX: f32 = 26.0; // Z-index for health change text
pub const EMPTY_HEALTHBAR_Z_INDEX: f32 = 29.0; // Z-index for empty healthbars
pub const HEALTHBAR_Z_INDEX: f32 = 30.0; // Z-index for healthbars
pub const GOLD_Z_INDEX: f32 = 31.0;

pub const BOARD_SIZE: i32 = 8; // Size of the game board (8x8)

// Animation speeds
pub const TWEEN_MOVE_ANIMATION_SPEED: f32 = 10.0; // Speed of tween move animations
pub const PULSE_ANIMATION_SPEED: f32 = 4.0; // Speed of pulse animations
pub const HEALTH_CHANGE_TEXT_ANIMATION_SPEED: f32 = 15.0; // Speed of health change text animations
pub const GOLD_ANIMATION_SPEED: f32 = 15.0; // Speed of gold animation

// Animation durations
pub const HEALTH_CHANGE_TEXT_ANIMATION_DURATION: f32 = 0.5; // Duration of health change text animations
pub const DEATH_ANIMATION_DURATION: f32 = 0.5; // Duration of death animations
pub const ATTACK_ANIMATION_DURATION: f32 = 0.5; // Duration of attack animations
pub const GOLD_ANIMATION_DURATION: f32 = 1.5; // Duration of gold animation
pub const GOLD_UI_COLOR_DURATION: f32 = 0.5; // Duration of gold UI color

// Player settings
pub const PLAYER_HEALTH: usize = 5; // Health of the player
pub const PLAYER_DAMAGE: usize = 1; // Damage of the player
pub const PRIMARY_COLOR: Color = Color::srgba(94.0 / 255.0, 205.0 / 255.0, 228.0 / 255.0, 1.0);
pub const SECONDARY_COLOR: Color = Color::srgba(0.674, 0.192, 0.192, 1.0);

pub const PRIMARY_COLOR_GRAYED: Color = Color::hsl(190.0, 1.0, 0.1);
pub const PRIMARY_COLOR_GRAYED_BRIGHTER: Color = Color::hsl(190.0, 1.0, 0.4);
pub const DARKER_PRIMARY_COLOR: Color = Color::srgba(
    94.0 * 0.9 / 255.0,
    205.0 * 0.9 / 255.0,
    228.0 * 0.9 / 255.0,
    1.0,
);
// Enemy settings
pub const ENEMY_BASE_HEALTH: usize = 3; // Health of the enemy
pub const ENEMY_BASE_DAMAGE: usize = 1; // Damage of the enemy

// Enemy spawn settings
pub const KING_SPAWN_TURN: usize = 15; // Turn number to spawn kings
pub const KNIGHT_SPAWN_TURN: usize = 30; // Turn number to spawn knights
pub const BISHOP_SPAWN_TURN: usize = 45; // Turn number to spawn bishops
pub const ROOK_SPAWN_TURN: usize = 60; // Turn number to spawn rooks
pub const QUEEN_SPAWN_TURN: usize = 75; // Turn number to spawn queens

pub const KING_SPAWN_WEIGHT: f32 = 1.0; // Weight of the king spawn
pub const PAWN_SPAWN_WEIGHT: f32 = 1.0; // Weight of the pawn spawn
pub const KNIGHT_SPAWN_WEIGHT: f32 = 1.0; // Weight of the knight spawn
pub const BISHOP_SPAWN_WEIGHT: f32 = 1.0; // Weight of the bishop spawn
pub const ROOK_SPAWN_WEIGHT: f32 = 1.0; // Weight of the rook spawn
pub const QUEEN_SPAWN_WEIGHT: f32 = 1.0; // Weight of the queen spawn

// Spritesheet settings
pub const HIGHLIGHT_ATLAS_INDEX: usize = 3; // Index of the highlight sprite in the spritesheet
pub const HIGHLIGHT_ATTACK_ATLAS_INDEX: usize = 6; // Index of the highlight attack sprite in the spritesheet
pub const FULL_HEALTHBAR_ATLAS_INDEX: usize = 7; // Index of the full healthbar sprite in the spritesheet
pub const EMPTY_HEALTHBAR_ATLAS_INDEX: usize = 8; // Index of the empty healthbar sprite in the spritesheet
pub const PLAYER_ATLAS_INDEX: usize = 20; // Index of the player sprite in the spritesheet
                                          // Spawner settings
pub const TARGET_NUM_ENEMIES: usize = 10; // Max number of enemies on the board
pub const PER_TURN_ENEMY_SPAWN_COUNT: usize = 2; // Number of enemies to spawn per turn
pub const SPRITESHEET_WIDTH: usize = 20; // Width of the spritesheet
pub const SPRITESHEET_HEIGHT: usize = 20; // Height of the spritesheet
pub const WIP_SPRITE_INDEX: usize = 19; // Index of the wip sprite in the spritesheet

// UI Settings
pub const UI_FONT_SIZE: f32 = 8.0; // Font size for UI elements
pub const UI_FONT: &str = "fonts/monogram/ttf/monogram-extended.ttf"; // Font for UI elements
pub const UI_HEADER_FONT_SIZE: f32 = 12.0; // Font size for UI headers
pub const HEALTH_CHANGE_TEXT_FONT_SIZE: f32 = 12.0; // Font size for health change text
pub const GOLD_FONT_SIZE: f32 = 12.0; // Font size for gold text

// UI Defeat
pub const DEFEAT_HEADER_FONT_SIZE: f32 = 36.0; // Font size for defeat text
pub const DEFEAT_SCORE_FONT_SIZE: f32 = 24.0; // Font size for score text

// UI Shop
pub const SHOP_FONT_SIZE: f32 = 16.0; // Font size for shop text
pub const SHOP_HEADER_FONT_SIZE: f32 = 32.0; // Font size for shop header text
pub const SHOP_PIECE_VALUE_GOLD_MULTIPLIER: f32 = 10.0; // Multiplier for the value of pieces in the shop
pub const SHOP_UPGRADES_COUNT_MOVEMENT: usize = 2;
pub const SHOP_UPGRADES_COUNT_STATS: usize = 3;
pub const STARTING_GOLD: usize = 100; // Starting gold

// Keyboard settings
pub const SHOP_KEY: KeyCode = KeyCode::KeyS; // Key to toggle the shop
