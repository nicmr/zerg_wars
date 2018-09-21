/// The following constants globally scale values such as speed, damage and size/position on the map
/// They can be tweaked in order to make the game easier to observe by a human player
/// 
/// Please note that the realtionship of movement speed and damage scale constants severely affect
/// the balance between melee and ranged units
/// 
/// Constant that scales the movement speed of units
pub const MOVEMENT_SPEED: f32 = 0.01;

/// Constant that scales the damage each unit does
pub const DAMAGE_SCALE: f32 = 0.01;

/// Constant that scales the positioning of units and bases
pub const MAP_SCALE: f32 =  500.0;

/// Constant that currently has no real use case
pub const SPEED: f32 = 8.0;