use ggez::GameResult;
use ggez::Context;

use gamestate::Side;
use gameobject::{GameChar, Base};

/// A player, controlled by either human or AI
#[derive(Debug, Clone)]
pub struct Player{
    pub units: Vec<GameChar>,
    pub minerals: u32,
    pub base: Base,
}


impl Player{
    pub fn new(ctx: &mut Context, side: Side) -> GameResult<Player>{
        Ok(Player{
            units: Vec::with_capacity(50),
            minerals: 0,
            base: Base::new(ctx, side)?,
        })
    } 
}