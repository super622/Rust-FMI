use ggez::audio::SoundSource;
use ggez::conf::{ WindowMode, WindowSetup};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::*;
use ggez::graphics::{self, Color};
use ggez::event::{self, KeyCode, KeyMods};


use ggez::mint::{Point2};
use std::path;

use solution::entities::*;
use solution::assets::*;
use solution::helper_functions::*;

struct GameState{
    black_jet: Jet,
    white_jet: Jet,
    white_shots: Vec<Shot>,
    black_shots: Vec<Shot>,
    screen_width: f32,
    screen_height: f32,
    assets: Assets,
    input_white: InputState,
    input_black: InputState
}

impl GameState{
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        welcome_mess();

        let assets = Assets::new(ctx)?;
        let black_jet = Jet::new();
        let white_jet = Jet::new();

        let(width, height) = graphics::drawable_size(ctx);

        Ok(GameState{
            black_jet: black_jet,
            white_jet: white_jet,
            white_shots: Vec::new(),
            black_shots: Vec::new(),
            screen_width: width,
            screen_height: height,
            assets: assets,
            input_white: InputState::default(),
            input_black: InputState::default()
        })
    }
    
    fn collision_handle(&mut self, ctx: &Context){
        for shot in &mut self.white_shots {
            let black_dist = shot.position - self.black_jet.position;

            if black_dist.length() < (shot.shotbox + self.black_jet.hitbox){
                shot.life = 0.0;
                self.black_jet.striked += 1;
                let _ = self.assets.hitted_sound.play(ctx);
            }
        }

        for shot in &mut self.black_shots {
            let white_dist = shot.position - self.white_jet.position;

            if white_dist.length() < (shot.shotbox + self.white_jet.hitbox){
                shot.life = 0.0;
                self.white_jet.striked += 1;
                let _ = self.assets.hitted_sound.play(ctx);
            }
        }
    }

    fn fire_shot_black(&mut self, _ctx: &Context){
        self.black_jet.shot_timeout = SHOT_DELAY;

        let player = &self.black_jet;
        let mut shot = Shot::new();
        shot.position = player.position;
        shot.facing = player.facing;
        let direction = Vec2::new(shot.facing.sin(), shot.facing.cos());

        shot.speed = direction * SHOT_SPEED;

        self.black_shots.push(shot);
    }

    fn fire_shot_white(&mut self, _ctx: &Context){
        self.white_jet.shot_timeout = SHOT_DELAY;

        let player = &self.white_jet;
        let mut shot = Shot::new();
        shot.position = player.position;
        shot.facing = player.facing;
        let direction = Vec2::new(shot.facing.sin(), shot.facing.cos());

        shot.speed =  direction * SHOT_SPEED;

        self.white_shots.push(shot);
    }

    fn clear_shot(&mut self) {
        self.white_shots.retain(|s| s.life > 0.0);
        self.black_shots.retain(|s| s.life > 0.0);
    }
}

impl ggez::event::EventHandler<GameError> for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {

    while timer::check_update_time(ctx, DESIRED_FPS) {
        let seconds = 1.0 / (DESIRED_FPS as f32);

        // Update the player state based on the user input.
        input_handler(&mut self.white_jet, &self.input_white, seconds);
        input_handler(&mut self.black_jet, &self.input_black, seconds);

        self.white_jet.shot_timeout -= seconds;
        self.black_jet.shot_timeout -= seconds;
        if self.input_black.fire && self.black_jet.shot_timeout < 0.0{
            self.fire_shot_black(ctx);
            self.assets.shooting_sound.play(ctx)?;
        }
        if self.input_white.fire && self.white_jet.shot_timeout < 0.0 {
            self.fire_shot_white(ctx);
            self.assets.shooting_sound.play(ctx)?;
        }

        // update player position
        update_player_pos(&mut self.white_jet, seconds);
        update_player_pos(&mut self.black_jet, seconds);
        //make sure jets dont apper on the other side of the screen when they overflow it
        player_overflowing_screen(
            &mut self.white_jet,
            self.screen_width as f32,
            self.screen_height as f32,
        );
        player_overflowing_screen(
            &mut self.black_jet,
            self.screen_width as f32,
            self.screen_height as f32,
        );
        
        for shot in &mut self.white_shots{
            update_shot_pos(shot, seconds);
            shot_overflowing_screen(shot,self.screen_width as f32, self.screen_width as f32);
            shot.life -= seconds;
        }

        for shot in &mut self.black_shots{
            update_shot_pos(shot, seconds);
            shot_overflowing_screen(shot,self.screen_width as f32, self.screen_width as f32);
            shot.life -= seconds;
        }
        self.collision_handle(ctx);

        self.clear_shot();
    }

    Ok(())
  }


  fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(89, 150, 206));

        {
          let assets = &mut self.assets;
          let coordinates = (self.screen_width, self.screen_height);
          let black_jet = &self.black_jet;
          let white_jet = &self.white_jet;
          draw_black(assets, ctx, black_jet, coordinates)?;
          draw_white(assets, ctx, white_jet, coordinates)?;

          for shot in &self.white_shots {
              draw_shot(assets, ctx, shot, coordinates)?;
          }

          for shot in &self.black_shots {
            draw_shot(assets, ctx, shot, coordinates)?;
          }
        }

        //displaying the results
        let white_score_position = Point2{
            x: 300.0,
            y: 10.0
        };
        let white_score = format!("White: {}", self.black_jet.striked);
        let display_score_white = graphics::Text::new((white_score, self.assets.font ,32.0));

        let black_score_position = Point2{
            x: 10.0,
            y: 10.0
        };
        let black_score = format!("Black: {}", self.white_jet.striked);
        let displays_score_black = graphics::Text::new((black_score,self.assets.font, 32.0));

        graphics::draw(ctx, &display_score_white, (white_score_position, 0.0, Color::WHITE))?;
        graphics::draw(ctx, &displays_score_black, (black_score_position, 0.0, Color::BLACK))?;
      
        //checking for winner of the game
        if self.black_jet.striked >= 5 {
            graphics::clear(ctx, graphics::Color::from_rgb(255, 255, 255));
            let game_over = graphics::Text::new((format!("Game over! White Jet wins!"), self.assets.font, 32.0));

            let center = Point2{
                x: 100.0,
                y: self.screen_height/2.0,
            };
            graphics::draw(ctx, &game_over, (center, 0.0, Color::BLACK))?;
        }
        if self.white_jet.striked >= 5 {
            graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
            let game_over = graphics::Text::new((format!("Game over! Black Jet wins!"), self.assets.font, 32.0));

            let center = Point2{
                x: 100.0,
                y: self.screen_height/2.0,
            };
            graphics::draw(ctx, &game_over, (center, 0.0, Color::WHITE))?;
           
        }

      graphics::present(ctx)?;

      timer::yield_now();
      Ok(())
  }

//   fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, mods: KeyMods, _: bool){
//       if keyboard::is_key_pressed(ctx, KeyCode::Up){
//           self.input_white.y = 1.0;
//       }
//       else if keyboard::is_key_pressed(ctx, KeyCode::Left){
//           self.input_white.x = -1.0;
//       }
//       else if keyboard::is_key_pressed(ctx, KeyCode::Right){
//           self.input_white.x = 1.0;
//       }
//       else if keyboard::is_key_pressed(ctx, KeyCode::W){
//           self.input_black.y = 1.0;
//       }
//       else if keyboard::is_key_pressed(ctx, KeyCode::A){
//           self.input_black.x = -1.0;
//       }
//       else if keyboard::is_key_pressed(ctx, KeyCode::D){
//           self.input_black.x = 1.0;
//       }
//       else {
//           ()
//       }
//   }

//   fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods){
//       if !keyboard::is_key_pressed(_ctx, KeyCode::Up){
//           self.input_white.y = 0.0;
//       }
//       else if !keyboard::is_key_pressed(_ctx, KeyCode::Left) | !keyboard::is_key_pressed(_ctx, KeyCode::Right){
//           self.input_white.x = 0.0;
//       }
//       else if !keyboard::is_key_pressed(_ctx, KeyCode::W){
//           self.input_black.y = 0.0;
//       }
//       else if !keyboard::is_key_pressed(_ctx, KeyCode::A) | !keyboard::is_key_pressed(_ctx, KeyCode::D){
//           self.input_black.x = 0.0;
//       }
//       else{
//           ()
//       }
//   }

  fn key_down_event(
    &mut self,
    ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
    _repeat: bool,
) {
    match keycode {
        KeyCode::Up => {
            self.input_white.y = 1.0;
        }
        KeyCode::Left => {
            self.input_white.x = -1.0;
        }
        KeyCode::Right => {
            self.input_white.x = 1.0;
        }
        KeyCode::W => {
            self.input_black.y = 1.0;
        }
        KeyCode::A => {
            self.input_black.x = -1.0;
        }
        KeyCode::D => {
            self.input_black.x = 1.0;
        }
        KeyCode::P => {
            self.input_white.fire = true;
        }
        KeyCode::Space => {
            self.input_black.fire = true;
        }
        KeyCode::Escape => event::quit(ctx),
        _ => (), // Do nothing
    }
}

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up => {
                self.input_white.y = 0.0;
            }
            KeyCode::W => {
                self.input_black.y = 0.0;
            }
            KeyCode::A | KeyCode::D =>{
                self.input_black.x = 0.0;
            }
            KeyCode::Left | KeyCode::Right => {
                self.input_white.x = 0.0;
            }
            KeyCode::Space => {
                self.input_black.fire = false;
            }
            KeyCode::P => {
                self.input_white.fire = false;
            }
            _ => (), // Do nothing
        }
    }
}


pub fn main() -> GameResult{
    let path = path::PathBuf::from("./resources");

    let window = WindowMode::default().dimensions(640.0, 480.0);

    let window_setup = WindowSetup::default().title("Jet Fighter");

    let (mut ctx, events_loop) = ContextBuilder::new("Jet Fighter","Marinski")
    .add_resource_path(path)
    .window_setup(window_setup)
    .window_mode(window)
    .build()
    .unwrap();

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, events_loop, state)
}