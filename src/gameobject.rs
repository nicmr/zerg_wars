use traits::{Position};
use gamestate::{Side};
use constants::{MOVEMENT_SPEED, DAMAGE_SCALE, MAP_SCALE};

use ggez::graphics;
use ggez::{Context, GameResult};
use ggez;


/// The stats of a character
#[derive(Debug, Clone)]
pub struct CharStats{
    pub cost: u32,
    pub hp: f32,
    damage: f32,
    speed: f32,
    range: f32,
    targets: usize,   
}






/// A `GameChar` represents what once would consider a `unit` in RTS. 
#[derive(Debug, Clone)]
pub struct GameChar{
    pub name: String,
    pub stats: CharStats,
    position: f32,
    pub sprite: graphics::Image,
    side: Side,
}



impl GameChar{
    /// Returns a GameChar that represents a zergling
    /// Fast, cheap, meelee fighter
    pub fn ling(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
        let mut sprite = graphics::Image::new(ctx, "/ggez_zergling_right.png")?;
        let mut position = 1.0;
        if side == Side::Left {
            sprite = graphics::Image::new(ctx, "/ggez_zergling_left.png")?;
            position = 0.0;
        }
        Ok(GameChar{
            name: String::from("ling"),
            stats: CharStats{
                cost: 10,
                speed: 2.0,
                damage: 4.0,
                hp: 10.0,
                range: 3.0,
                targets: 1,
            },
            position: position,
            sprite: sprite,
            side: side,
        })
        
    }
    /// Returns a GameChar that represents a hydra
    /// Ranged damage
    pub fn hydra(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
        let mut sprite = graphics::Image::new(ctx, "/ggez_hydra_right.png")?;
        let mut position = 1.0;
        if side == Side::Left {
            sprite = graphics::Image::new(ctx, "/ggez_hydra_left.png")?;
            position = 0.0;
        }
        Ok(GameChar{
            name: String::from("hydra"),
            stats: CharStats{
                cost: 20,
                speed: 1.0,
                damage: 4.0,
                hp: 12.0,
                range: 150.0,
                targets: 1,
            },
            position: position,
            sprite: sprite,
            side: side,
        })

    }

    /// Returns a GameChar that represents a baneling
    /// High splash damage, slow, decent HP
    pub fn bane(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
        let mut sprite = graphics::Image::new(ctx, "/ggez_bane_right.png")?;
        let mut position = 1.0;

        if side == Side::Left {
            sprite = graphics::Image::new(ctx, "/ggez_bane_left.png")?;
            position = 0.0;

        }
        Ok(GameChar{
            name: String::from("bane"),
            stats: CharStats{
                cost: 30,
                speed: 0.5,
                damage: 10.0,
                hp: 15.0,
                range: 3.0,
                targets: 1,
            },
            position: position,
            sprite: sprite,
            side: side,
        })
    }

    fn in_range<T: Position>(&self, other: &T) -> bool {
        (self.position() - other.position()).abs() < self.stats.range
    }

    /// Returns the damage value of the `GameChar`, multiplicated with
    /// the DAMAGE_SCALE consant
    fn damage(&self) -> f32{
        self.stats.damage*DAMAGE_SCALE
    }

    

    /// Returns the (horizontal) speed of the `GameChar`, multiplicated with
    /// the MOVEMENT_SPEED constant
    fn speed(&self) -> f32 {
        self.stats.speed*MOVEMENT_SPEED
    }

    /// Moves self forward on the x-scale, away from its own base
    fn move_forward(&mut self, ctx: &mut Context){
        match self.side{
            Side::Left => self.position += self.speed()*(ggez::timer::get_delta(ctx).subsec_nanos() as f32/1e8),
            Side::Right => self.position -= self.speed()*(ggez::timer::get_delta(ctx).subsec_nanos() as f32/1e8),
        }
    }

    /// Makes &mut self attack as many `GameChar`s in enemies as possible,
    /// until it has reached the maximum number of targets
    /// If there was no target to attack, it will move by calling self.move_forward()
    pub fn attack_move(&mut self, ctx: &mut Context, enemies: &mut Vec<GameChar>, base: &mut Base){

        //mobile means the opposite of immobile
        //mobile is truue when there has been no attack        
        let mut mobile = true;
        let mut attack_count = 0;
        for enemy_unit in enemies {
            //ensure unit attacks only one target
            if attack_count == self.stats.targets{
                break;
            }
            //determine if target is in range
            if self.in_range(enemy_unit) {
                attack_count += 1;    
                mobile = false;
                enemy_unit.stats.hp = enemy_unit.stats.hp.zero_saturating_sub(self.damage());
            }
        }
        if attack_count < self.stats.targets && self.in_range(base) {
            mobile = false;
            base.hp = base.hp.zero_saturating_sub(self.damage());
        }
        //move, if possible
        if mobile{
            self.move_forward(ctx);
        }
    }
}



/// The base each player has to defend
#[derive(Debug, Clone)]
pub struct Base{
    pub sprite: graphics::Image,
    pub hp: f32,
    position: f32,
}
impl Base {
    ///Returns a new `Base` on the specified side of the map
    pub fn new(ctx: &mut Context, side: Side) -> GameResult<Base> {
        let sprite = graphics::Image::new(ctx, "/hatchery.png")?;
        let mut position = 1.0; //All the way to the right
        if side == Side::Left { 
            position = 0.0;     //All the way to the left
        }
        Ok(Base{
            sprite: sprite,
            hp: 100.0,
            position: position,
        })
    }
}
impl Position for Base {
    /// Returns the (x-Axis) position of the `GameChar`, multiplicated with
    /// the MAP_SCALE constant
    fn position(&self) -> f32 {
        self.position * MAP_SCALE
    }
}




impl Position for GameChar { 
    /// Returns the (x-Axis) position of the `GameChar`, multiplicated with
    /// the MAP_SCALE constant
    fn position(&self) -> f32 {
        self.position * MAP_SCALE
    }
}


/// Zero-saturating mathematical operations that are not provided by default 
trait ZeroSaturationOps{
    fn zero_saturating_sub(&self, rhs: f32) -> f32;
}
impl ZeroSaturationOps for f32 {
    /// Saturating float subtraction. Computes self - rhs, saturating at zero instead of returning
    /// a negative value
    fn zero_saturating_sub(&self, rhs: f32) -> f32 {
        let result = self - rhs;
        if result<0.0 {0.0} else {result}
    }
}