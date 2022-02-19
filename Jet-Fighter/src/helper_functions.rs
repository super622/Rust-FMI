use crate::entities::*;
use crate::assets::*;
use ggez::mint::{Point2};
use ggez::{Context, GameResult};
use ggez::graphics::{self};

pub fn welcome_mess(){
    println!();
    println!("Welcome to Jet Fighter!");
    println!("Black Jet plays with A, W and D keyboard keys to move his jet and Space to shoot");
    println!("White Jet plays with left, up and right keyboard arrows to move his jet and P key shoot");
    println!();
}

pub fn input_handler(player: &mut Jet, input: &InputState, secs: f32){
    player.facing += secs * PLAYER_TURN * input.x;

    if input.y > 0.0 {
        let direction_vector = Vec2::new(player.facing.sin(), player.facing.cos());
        //gradually accelarating
        player.speed += direction_vector * (PLAYER_ACCELERATION) * (secs);
    }
}

pub fn update_player_pos(player: &mut Jet, dt: f32) {
    let norm_sq = player.speed.length_squared();
    if norm_sq > MAX_PHYSICS_VEL.powi(2) {
        player.speed = player.speed / norm_sq.sqrt() * MAX_PHYSICS_VEL;
    }
    //calculatin distance using the S= V*t physics formula
    let s = player.speed * dt;
    player.position += s;
    player.facing += player.ang_vel;
}

pub fn update_shot_pos(shot: &mut Shot, dt: f32) {
    let s = shot.speed * dt;
    shot.position += s;
    shot.facing += shot.ang_vel;
}

//to screen cordinates
pub fn cordinates_converter(screen_width: f32, screen_height: f32, point: Vec2) -> Point2<f32>{
    let point = Point2{
        x: point.x + screen_width / 2.0,
        y: screen_height - (point.y + screen_height / 2.0)
    };

    point
}

pub fn player_overflowing_screen(player: &mut Jet, sx: f32, sy: f32) {
    let screen_x_bounds = sx/2.0;
    let screen_y_bounds = sy/2.0;
    if player.position.x > screen_x_bounds {
        player.position -= Vec2::new(sx, 0.0);
    } else if player.position.x < -screen_x_bounds {
        player.position += Vec2::new(sx, 0.0);
    };
    if player.position.y > screen_y_bounds {
        player.position -= Vec2::new(0.0, sy);
    } else if player.position.y < -screen_y_bounds {
        player.position += Vec2::new(0.0, sy);
    }
}


pub fn shot_overflowing_screen(shot: &mut Shot, x: f32, y: f32){
    let sx = x/2.0;
    let sy = y/2.0;

    if shot.position.x > sx {
        shot.position -= Vec2::new(x,0.0);
    }
    else if shot.position.x < -sx {
        shot.position += Vec2::new(x, 0.0);
    };

    if shot.position.y > sy{
        shot.position -= Vec2::new(0.0,y);
    }
    else if shot.position.y < sy{
        shot.position += Vec2::new(0.0, y);
    }
}


pub fn draw_white(
    assets: &mut Assets, 
    ctx: &mut Context, 
    white_jet: &Jet,
    coordinate: (f32, f32)
    ) -> GameResult {
        let (sc_width, sc_height) = coordinate;
        let pos_white_jet = cordinates_converter(sc_width,sc_height, white_jet.position);
        
        let white_jet_img = assets.white_jet_image();

        let drawparams = graphics::DrawParam::new()
            .dest(pos_white_jet)
            .rotation(white_jet.facing as f32)
            .offset(Point2{x: 0.5,y:0.5});

        graphics::draw(ctx, white_jet_img, drawparams)
    }

pub fn draw_black(
    assets: &mut Assets, 
    ctx: &mut Context, 
    black_jet: &Jet,
    coordinate: (f32, f32)
    ) -> GameResult {
        let (sc_width, sc_height) = coordinate;
        let pos_black_jet:Point2<f32> = cordinates_converter(sc_width,sc_height, black_jet.position);
        
        let black_jet_img = assets.black_jet_image();

        let drawparams = graphics::DrawParam::new()
            .dest(pos_black_jet)
            .rotation(black_jet.facing as f32)
            .offset(Point2{x: 0.5,y:0.5});

        graphics::draw(ctx, black_jet_img, drawparams)
    }

pub fn draw_shot(
    assets: &mut Assets,
    ctx: &mut Context,
    shot: &Shot,
    coordinate: (f32, f32)
) -> GameResult {
    let (sc_width, sc_height) = coordinate;
    let shot_pos = cordinates_converter(sc_width, sc_height, shot.position);

    let shot_img = assets.shot_image();

     let drawparams = graphics::DrawParam::new()
            .dest(shot_pos)
            .rotation(shot.facing as f32)
            .offset(Point2{x: 0.5,y:0.5});

    graphics::draw(ctx, shot_img, drawparams)
}