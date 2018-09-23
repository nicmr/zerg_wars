use ggez::{GameResult, GameError};
use ggez::Context;

use gamestate::Side;
use gameobject::{GameChar, Base};

/// A player, controlled by either human or AI
#[derive(Debug, Clone)]
pub struct Player{
    pub units: Vec<GameChar>,
    pub minerals: u32,
    pub base: Base,
    controltype: Controltype,
}


impl Player{
    /// Returns a new `Player` struct, controlled by a human
    pub fn new(ctx: &mut Context, side: Side) -> GameResult<Player>{
        Ok(Player{
            units: Vec::with_capacity(50),
            minerals: 0,
            base: Base::new(ctx, side)?,
            controltype: Controltype::Human
        })
    }
    /// Returns a new `Player` struct, controlled by a simple AI
    pub fn new_ai(ctx: &mut Context, side: Side) -> GameResult<Player>{
        Ok(Player{
            units: Vec::with_capacity(50),
            minerals: 0,
            base: Base::new(ctx, side)?,
            controltype: Controltype::SimpleAI
        })
    }
    pub fn build_unit(&mut self, unit: GameChar) -> GameResult<()>{
        if unit.stats.cost <= self.minerals {
            self.minerals -= unit.stats.cost;
            self.units.push(unit);

            Ok(())
        }else{
            Err(GameError::IntegerError(String::from("Not enough minerals")))
        }        
    }


    //better: compose with `Controller` struct of AI type, delegate build_decision to composite struct
    //for AI
    pub fn build_decision(&self, opponent: &Player){
        if self.controltype == Controltype::SimpleAI {

            // get these from somewhere else
            let lingcost = 200;
            let hydracost = 300;
            let banecost = 400;

            let mut my = Unitcount::new(0,0,0);
        
            let mut opposing = Unitcount::new(0,0,0);

            for unit in &self.units{
                //better solution: add unit_type enum to the units, match that enum
                match unit.name.as_str() {
                    "ling" => my.lingcount += 1,
                    "hydra" => my.hydracount += 1,
                    "baneling" => my.banecount += 1,
                    _ => ()
                }
            }
            for unit in &opponent.units {
                //better solution: add unit_type enum to the units, match that enum
                match unit.name.as_str() {
                    "ling" => opposing.lingcount += 1,
                    "hydra" => opposing.hydracount += 1,
                    "baneling" => opposing.banecount += 1,
                    _ => ()
                }
            }

            if opposing.total() == 0 && self.minerals > lingcost + hydracost{
                //build hydra and ling
            }        
            if my.lingcount < opposing.hydracount && opposing.hydracount >= opposing.banecount
                && self.minerals > lingcost {
                self.build_unit(GameChar::ling(ctx, self.side))
            }
            if my.hydracount < opposing.banecount && opposing.lingcount <= opposing.banecount
                && self.minerals > hydracost {
                    //build hydra
            }
            if my.banecount < opposing.lingcount &&  opposing.lingcount >= opposing.hydracount
                && self.minerals > banecost{
                    //build bane
            }
        }
        
        
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Controltype{
    Human, SimpleAI,
}

struct Unitcount{
    lingcount: u32,
    hydracount: u32,
    banecount: u32
}
impl Unitcount{
    fn new(lingcount: u32, hydracount: u32, banecount: u32) -> Unitcount{
        Unitcount{
            lingcount,
            hydracount,
            banecount
        }
    }
    fn total(&self) -> u32{
        self.lingcount + self.hydracount + self.banecount
    }
}
