extern crate ggez;

use ggez::event;
use ggez::event::{Keycode, Mod};
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::conf;

use std::{env, path};


/// The following constants globally scale values such as speed, damage and size/position on the map
/// They can be tweaked in order to make the game easier to observe by a human player
/// 
/// Please note that the realtionship of movement speed and damage scale constants severely affect
/// the balance between melee and ranged units
/// 
/// Constant that scales the movement speed of units
const MOVEMENT_SPEED: f32 = 0.01;

/// Constant that scales the damage each unit does
const DAMAGE_SCALE: f32 = 0.01;

/// Constant that scales the positioning of units and bases
const MAP_SCALE: f32 =  500.0;

/// Constant that currently has no real use case
const SPEED: f32 = 8.0;


/// Tracks the global game state
struct GameState {
    players: Vec<Player>,
    offset: f32,
    last_start: std::time::Duration,
    font: graphics::Font,

    //only for quick debugging, consider removing in final release
    debug: bool,
    debug_once: bool,
}

impl GameState {
    /// Returns a new GameState struct
    pub fn new(ctx: &mut Context) -> GameResult<GameState>{
        let font = graphics::Font::new(ctx, "/Roboto-Regular.ttf", 16)?;
        Ok(GameState{
            players: vec!(Player::new(ctx, Side::Left)?, Player::new(ctx, Side::Right)?),
            offset: 0.0,
            last_start: std::time::Duration::from_secs(0),
            font: font,
            debug: true,
            debug_once: true,
        })
    }
}

impl event::EventHandler for GameState {
    /// Handles key press events
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool){
        match keycode{
            // Spawn a zergling for Player 0
            Keycode::Num1 => {
                if self.players[0].minerals>200{
                    self.players[0].minerals -= 200;
                    self.players[0].units.push(GameChar::ling(ctx, Side::Left).unwrap());

                }
            }
            // Spawn a hydra for Player 0
            Keycode::Num2 => {
                if self.players[0].minerals>500{
                    self.players[0].minerals -= 300;
                    self.players[0].units.push(GameChar::hydra(ctx, Side::Left).unwrap());
                }
            }
            // Spawn a baneling for Player 0
            Keycode::Num3 => {
                if self.players[0].minerals>300{
                    self.players[0].minerals -= 300;
                    self.players[0].units.push(GameChar::bane(ctx, Side::Left).unwrap());
                    
                }
            }

            // Spawn a zergling for Player 1
            Keycode::Kp1 => {
                if self.players[1].minerals>200{
                    self.players[1].minerals -= 200;
                    self.players[1].units.push(GameChar::ling(ctx, Side::Right).unwrap());

                }
            }
            // Spawn a hydra for Player 1
            Keycode::Kp2 => {
                if self.players[1].minerals>500{
                    self.players[1].minerals -= 300;
                    self.players[1].units.push(GameChar::hydra(ctx, Side::Right).unwrap());
                }
            }
            // Spawn a baneling for Player 1
            Keycode::Kp3 => {
                if self.players[1].minerals>300{
                    self.players[1].minerals -= 300;
                    self.players[1].units.push(GameChar::bane(ctx, Side::Right).unwrap());
                    
                }
            }

            // Replace the unit vector with a new, empty one, effectively removing all units
            Keycode::Backspace => {
                for player in &mut self.players{
                    player.units = Vec::with_capacity(50);
                }
            }

            _ => (),  // Unknown key, do nothing
        }
       
    }

    /// Handles key release events
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool){

    }

    /// Updates gameplay elements, such as `GameChar`s
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut elapsed_time =ggez::timer::duration_to_f64(ggez::timer::get_time_since_start(ctx)) as f32; 
        elapsed_time -= ggez::timer::duration_to_f64(self.last_start) as f32;
        self.offset -= SPEED*(((ggez::timer::get_delta(ctx)).subsec_nanos() as f32)/1e8)+elapsed_time*(1.0/24.0);



        // Deal damage
        // The following implementation seems to favour the player whose units attack first
        // But as units with 0 hp are only sorted out at the end of the tick, this is not an issue
        // (effectively allowing units whose hp were reduced to 0 or bellows to attack 1 more time)
        {
            let (player_0, player_1) = &mut self.players[..].split_at_mut(1);

            // Left player's units deal damage and move
            for unit in &mut player_0[0].units { 
                let mut mobile = true;  //mobile meaning the opposite of immobile
                for enemy_unit in &mut player_1[0].units{
                    if unit.in_range(enemy_unit){
                        mobile = false;

                        enemy_unit.stats.hp = enemy_unit.stats.hp.zero_saturating_sub(unit.damage());
                    }
                }
                if mobile{
                    unit.position += unit.stats.speed*MOVEMENT_SPEED*((ggez::timer::get_delta(ctx).subsec_nanos() as f32/1e8));

                }
            }

            // Right player's units deal damage and move
            for unit in &mut player_1[0].units{
                let mut mobile = true;  //mobile meaning the opposite of immobile
                for enemy_unit in &mut player_0[0].units {
                    if unit.in_range(enemy_unit) {
                        mobile = false;
                        enemy_unit.stats.hp = enemy_unit.stats.hp.zero_saturating_sub(unit.damage());
                    }
                }
                if mobile{
                    unit.position -= unit.stats.speed*MOVEMENT_SPEED*((ggez::timer::get_delta(ctx).subsec_nanos() as f32/1e8));
                }
            }
        }
        
        //this could potentially be adapted to be multithreaded
        for player in &mut self.players{
            //Gain resources
            player.minerals += 1;

            //remove dead units
            let mut living_units = Vec::with_capacity(player.units.capacity());
            for unit in &player.units{
                if unit.stats.hp > 0.0{
                    living_units.push(unit.clone());
                }
            }
            player.units = living_units;
        }

        

        // for unit in &mut self.players[1].units{
        //     //determine if any enemy unit can be attacked
        //     for (k, enemy_unit) in self.players[0].units.iter().enumerate(){

        //         if unit.in_range(enemy_unit){
        //             //do not change position
        //         }
        //     }
        //     unit.position -= unit.stats.speed*MOVEMENT_SPEED*((ggez::timer::get_delta(ctx).subsec_nanos() as f32/1e8));
        // }
        

        Ok(())
    }

    /// Draws gameplay elements, such as `GameChar`s
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        if self.debug_once && self.debug{
            self.debug_once = false;
            println!("{:?}", self.players);
        }
        for player in & self.players {

            //draw each players' base!
            let p = graphics::DrawParam {
                dest: graphics::Point2::new(player.base.position(), 200.0),
                scale: graphics::Point2::new(0.25, 0.25),
                rotation: 0.0,
                ..Default::default()
            };

            graphics::draw_ex(ctx, &player.base.sprite, p)?;

             //draw each player's minerals
            {
               
                let s = format!("Minerals: {}", player.minerals);
                let text = graphics::Text::new(ctx, s.as_str(), &self.font)?;
                let dest_point = graphics::Point2::new(player.base.position(), 100.0);
                graphics::draw(ctx, &text, dest_point, 0.0)?;
            }
            

            //draw all units!
            for unit in &player.units{
                
                let p = graphics::DrawParam {
                dest: graphics::Point2::new(unit.position(), 400.0),
                scale: graphics::Point2::new(0.15, 0.15),
                rotation: 0.0,
                ..Default::default()
                };

                graphics::draw_ex(ctx, &unit.sprite, p)?;

            }

        }

        graphics::present(ctx);
        Ok(())
    }
}


/// Used to identify the two sides of the battlefield, Left and Right
#[derive(Debug, Clone, PartialEq)]
enum Side{
    Left,
    Right,
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

/// A player, controlled by either human or AI
#[derive(Debug, Clone)]
struct Player{
    units: Vec<GameChar>,
    minerals: u32,
    base: Base,
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

/// The base each player has to defend
#[derive(Debug, Clone)]
struct Base{
    sprite: graphics::Image,
    hp: f32,
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
    /// Returns the base's (x-Axis) Position, multiplied with 
    /// the MAP_SCALE constant
    pub fn position(&self) -> f32 {
        self.position * MAP_SCALE
    }
}



/// The stats of a character
#[derive(Debug, Clone)]
struct CharStats{
    cost: u32,
    hp: f32,
    damage: f32,
    speed: f32,
    range: f32,
    targets: usize,   
}

///A `GameChar` represents what once would consider a `unit` in RTS. 
#[derive(Debug, Clone)]
struct GameChar{
    name: String,
    stats: CharStats,
    position: f32,
    sprite: graphics::Image
}

impl GameChar{
    /// Returns a GameChar that represents a zergling
    /// Fast, cheap, meelee fighter
    fn ling(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
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
        })
        
    }
    /// Returns a GameChar that represents a hydra
    /// Ranged damage
    fn hydra(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
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
        })

    }

    /// Returns a GameChar that represents a baneling
    /// High splash damage, slow, decent HP
    fn bane(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
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
        })
    }

    fn in_range(&self, other: &GameChar) -> bool {
        (self.position() - other.position()).abs() < self.stats.range
    }

    /// Returns the damage value of the `GameChar`, multiplicated with
    /// the DAMAGE_SCALE consant
    fn damage(&self) -> f32{
        self.stats.damage*DAMAGE_SCALE
    }

    /// Returns the (x-Axis) position of the `GameChar`, multiplicated with
    /// the MAP_SCALE constant
    fn position(&self) -> f32 {
        self.position * MAP_SCALE
    }
}



fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    //mount the assets folder into the ggez filesystem
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR"){
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        println!("{:?}", path);
        ctx.filesystem.mount(&path, true)
    }

    //create gamestate
    let state = &mut GameState::new(ctx).unwrap();

    //run gamestate
    if let Err(e) = event::run(ctx, state){
        println!("Error encountered: {}", e);
    }else{
        println!("Game exited cleanly.");
    }
}


