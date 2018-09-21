use ggez::event::{Keycode, Mod};
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;
use std::time;
use ggez;

use player::Player;
use gameobject::{GameChar};
use constants::SPEED;
use traits::Position;

/// Tracks the global game state
pub struct GameState {
    players: Vec<Player>,
    offset: f32,
    last_start: time::Duration,
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
            last_start: time::Duration::from_secs(0),
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
                unit.attack_move(ctx, &mut player_1[0].units, &mut player_1[0].base);
            }

            // Right player's units deal damage and move
            for unit in &mut player_1[0].units{
                unit.attack_move(ctx, &mut player_0[0].units, &mut player_0[0].base);
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side{
    Left,
    Right,
}