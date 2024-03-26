use raylib::prelude::*;
use raylib::consts::MouseButton::*;
pub use raylib::core::*;
use std::thread::sleep;
use std::time::Duration;

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 800.0;
const BALL_COLOR: Color = Color::WHITE;
const BALL_RADIUS: f32 = 6.0;

#[derive(Clone, Copy, Debug)]
struct Ball {
    position: Vector2,
    radius: f32,
    color: Color,
    visible: bool,
}

impl Ball {
    pub fn new(pos: Vector2, visible: bool) -> Self {
        Self {
            position: Vector2::new(pos.x, pos.y),
            radius: BALL_RADIUS,
            color:  BALL_COLOR,
            visible
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Chaikin")
        .vsync()
        .build();
    
    let mut balls = Vec::new();
    let mut balls_before_s: Vec<Ball> = Vec::new();
    let mut lines: Vec<(Vector2, Vector2)> = Vec::new();
    let mut can_draw = false;
    let mut animation_running = false;
    let duration = Duration::from_millis(600);
    let mut counter = 0;

    //gameloop
    while !rl.window_should_close() {

        if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) && animation_running != true {
            let pos = rl.get_mouse_position();
            balls.push(Ball::new(pos, true));
        }

        //toggles lines visibility
        if rl.is_key_released(KeyboardKey::KEY_ENTER) {
            if can_draw == false {
              can_draw = true;  
            } else {
                can_draw = false
            }
        }

        //clear all balls and lines
        if rl.is_key_released(KeyboardKey::KEY_C) {
            balls.clear();
            lines.clear();
            animation_running = false;
        }

        //starts or continues Chaikin Magic
        if rl.is_key_released(KeyboardKey::KEY_S) && balls.len() > 2  || animation_running == true {

            //on first KEY_S press, save current balls positions and start animation
            if animation_running == false {
                balls_before_s = balls.clone();   
                animation_running = true; 
            }
            //new vector to temporarily hold additional balls
            let mut smoother: Vec<Ball> = Vec::new();

            //Chaiking formula and new balls creation
            do_the_chaikin(balls, &mut smoother);
            
            //replace old balls with 2x young balls
            balls = smoother.clone(); // no pun intended!

            //as animation counter turns over, return balls to original set
            if counter == 6 {
                balls = balls_before_s.clone();
            }
            //remove old lines
            lines.clear()
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // draw clicked points (if animation is running, only first and last point will be drawn)
        for i in 0..balls.len() {
            if balls[i].visible == true {
                d.draw_circle_v(balls[i].position, BALL_RADIUS, BALL_COLOR);
            }
            // get lines starting and ending points.
            if i < balls.len()-1 {
            let line_temp = (balls[i].position, balls[i+1].position);
            lines.push(line_temp);
            }
        }
        //draw lines if allowed
        if can_draw == true && lines.len() > 0 {
            for j in 0..lines.len() {
            d.draw_line_v(lines[j].0, lines[j].1, Color::WHITE)
            } 
        }
        //slows things down for older folk
        if animation_running == true {
            sleep(duration);
            counter += 1;

            if counter == 7 {
                counter = 0;
            }
        }
    }
}

fn do_the_chaikin(b: Vec<Ball>, smoother: &mut Vec<Ball>) {

    for i in 0..b.len() {
        //exclude first and last ball 
        if i == 0 ||  i == b.len()-1 {
            smoother.push(b[i])
        } else {
            //get previous ball position
            let prev_ball_x = b[i-1].position.x;
            let prev_ball_y = b[i-1].position.y;
            //get current ball position
            let curr_ball_x = b[i].position.x;
            let curr_ball_y = b[i].position.y;
            //get next ball position
            let next_ball_x = b[i+1].position.x;
            let next_ball_y = b[i+1].position.y;
            //calculate new positions using Chaikins formula
            let prev_new_ball_position_x = 0.25 * prev_ball_x + 0.75 * curr_ball_x;
            let prev_new_ball_position_y = 0.25 * prev_ball_y + 0.75 * curr_ball_y;
            let next_new_ball_position_x = 0.75 * curr_ball_x + 0.25 * next_ball_x;
            let next_new_ball_position_y = 0.75 * curr_ball_y + 0.25 * next_ball_y;
            //create new balls
            let new_prev_ball = Ball::new(Vector2::new(prev_new_ball_position_x, prev_new_ball_position_y), false);
            let new_next_ball = Ball::new(Vector2::new(next_new_ball_position_x, next_new_ball_position_y), false);

            smoother.push(new_prev_ball);
            smoother.push(new_next_ball);
        }
    }  
}
