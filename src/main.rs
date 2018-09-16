extern crate ggez;

use ggez::event;
use ggez::event::{Keycode, Mod};
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::conf;

use std::{env, path};

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

struct GameState {
    player_1: Player,
    player_2: Player,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState>{
        Ok(GameState{
            player_1: Player::new(ctx, Side::Left)?,
            player_2: Player::new(ctx, Side::Right)?,
        })
    }
}

impl event::EventHandler for GameState {
    // ideas: show card effects while up key is pressed
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool){
        

    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool){

    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}



#[derive(Debug, Clone, PartialEq)]
enum Side{
    Left,
    Right,
}

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
            base: Base::new(ctx)?,
        })
    } 
}


struct Base{
    sprite: graphics::Image,
    hp: f32,   
}
impl Base {
    pub fn new(ctx: &mut Context) -> GameResult<Base> {
        let sprite = graphics::Image::new(ctx, "/ggezzergling.png")?;
        Ok(Base{
            sprite: sprite,
            hp: 100.0,
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
#[derive(Debug, Clone)]
struct GameChar{
    name: String,
    stats: CharStats,
    position: f32,
    sprite: graphics::Image
}

impl GameChar{
    fn ling(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
        let mut sprite = graphics::Image::new(ctx, "/ggez_zergling_right.png")?;
        if side == Side::Left {
            sprite = graphics::Image::new(ctx, "/ggez_zergling_left.png")?;
        }
        Ok(GameChar{
            name: String::from("ling"),
            stats: CharStats{
                cost: 10,
                speed: 5.0,
                damage: 4.0,
                hp: 10.0,
                range: 1.0,
                targets: 1,
            },
            position: 0.0,
            sprite: sprite,
        })
        
    }
    fn hydra(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
        let mut sprite = graphics::Image::new(ctx, "/ggez_hydra_right.png")?;
        if side == Side::Left {
            sprite = graphics::Image::new(ctx, "/ggez_hydra_left.png")?;
        }
        Ok(GameChar{
            name: String::from("hydra"),
            stats: CharStats{
                cost: 20,
                speed: 5.0,
                damage: 4.0,
                hp: 12.0,
                range: 1.0,
                targets: 1,
            },
            position: 0.0,
            sprite: sprite,
        })

    }

    fn bane(ctx: &mut Context, side: Side) -> GameResult<GameChar>{
        let mut sprite = graphics::Image::new(ctx, "/ggez_zergling_right.png")?;
        if side == Side::Left {
            sprite = graphics::Image::new(ctx, "/ggez_zergling_left.png")?;
        }
        Ok(GameChar{
            name: String::from("bane"),
            stats: CharStats{
                cost: 30,
                speed: 2.0,
                damage: 10.0,
                hp: 10.0,
                range: 1.0,
                targets: 1,
            },
            position: 0.0,
            sprite: sprite,
        })
    }
}



