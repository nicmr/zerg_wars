extern crate ggez;

use ggez::event;
use ggez::event::{Keycode, Mod};
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::conf;

use std::{env, path};

/// Constant that currently has no real use case
const SPEED: f32 = 8.0;

/// Constant that scales the movement speed of units
const MOVEMENT_SPEED: f32 = 0.01;

/// Constant that scales the positioning of units and bases
const MAP_SCALE: f32 =  500.0;


/// Tracks the global game state
struct GameState {
    players: Vec<Player>,
    offset: f32,
    last_start: std::time::Duration,

    debug: bool,
    debug_once: bool,
}

impl GameState {
    /// Returns a new GameState struct
    pub fn new(ctx: &mut Context) -> GameResult<GameState>{
        Ok(GameState{
            players: vec!(Player::new(ctx, Side::Left)?, Player::new(ctx, Side::Right)?),
            offset: 0.0,
            last_start: std::time::Duration::from_secs(0),

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
                self.players[0].units.push(GameChar::ling(ctx, Side::Left).unwrap());
            }
            // Spawn a hydra for Player 0
            Keycode::Num2 => {
                self.players[0].units.push(GameChar::hydra(ctx, Side::Left).unwrap());
            }
            // Spawn a baneling for Player 0
            Keycode::Num3 => {
                self.players[0].units.push(GameChar::bane(ctx, Side::Left).unwrap());
            }

            // Spawn a zergling for Player 1
            Keycode::Kp1 => {
                self.players[1].units.push(GameChar::ling(ctx, Side::Right).unwrap());
            }
            // Spawn a hydra for Player 1
            Keycode::Kp2 => {
                self.players[1].units.push(GameChar::hydra(ctx, Side::Right).unwrap());
            }
            // Spawn a baneling for Player 1
            Keycode::Kp3 => {
                self.players[1].units.push(GameChar::bane(ctx, Side::Right).unwrap());
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

        // issue: we can not mutably borrow two different array indices at a time
        // possible workarounds: 
        // A - split into immutable check range loop and mutable move loop, add bool field to struct
        //      that tracks whether the the unit should move
        // B - use split at mut to generate two separate, mutable slice views on the vec
        //
        // Here, I've decided to use th Option B
        {
            let (player_0, player_1) = &mut self.players[..].split_at_mut(1);

            for unit in &mut player_0[0].units { 
                let mut mobile = true;
                for enemy_unit in &player_1[0].units{
                    if unit.in_range(enemy_unit){
                        mobile = false;
                    }
                }
                if mobile{
                    unit.position += unit.stats.speed*MOVEMENT_SPEED*((ggez::timer::get_delta(ctx).subsec_nanos() as f32/1e8));

                }
            }

            for unit in &mut player_1[0].units{
                let mut mobile = true; //mobile meaning the opposite of immobile, not a mobile phone
                for enemy_unit in &player_0[0].units {
                    if unit.in_range(enemy_unit) {
                        mobile = false;
                    }
                }
                if mobile{
                    unit.position -= unit.stats.speed*MOVEMENT_SPEED*((ggez::timer::get_delta(ctx).subsec_nanos() as f32/1e8));
                }
            }
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
                dest: graphics::Point2::new(player.base.position*MAP_SCALE, 200.0),
                scale: graphics::Point2::new(0.25, 0.25),
                rotation: 0.0,
                ..Default::default()
            };

            graphics::draw_ex(ctx, &player.base.sprite, p)?;


            //draw all units!
            for unit in &player.units{
                
                let p = graphics::DrawParam {
                dest: graphics::Point2::new(unit.position*MAP_SCALE, 400.0),
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

///The base each player has to defend
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
}




#[derive(Debug, Clone)]
struct CharStats{
    cost: u32,
    hp: f32,
    speed: f32,
    damage: f32,
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
                speed: 1.0,
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
                speed: 2.0,
                damage: 4.0,
                hp: 12.0,
                range: 10.0,
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
                speed: 1.0,
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
        (self.position - other.position).abs()*MAP_SCALE < self.stats.range
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


