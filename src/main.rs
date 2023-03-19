mod physx;

use physx::PhysxObject;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640,480)
        .title("test")
        .build();

    rl.set_target_fps(30);

    let ball_position = Vector2 {
        x: 50.0,
        y: 200.0,
    };

    let dampening_factor:f32 = 2.0;

    let mut ball: PhysxObject = PhysxObject::new(ball_position, Vector2 { x: 0.0, y: 0.0 }, Vector2 { x: 0.0, y: 0.0 });
    ball.set_accel(Vector2 { x: 0.0, y: 1.0 });
    ball.set_vel(Vector2 { x: 3.0, y: 0.0 });


    while !rl.window_should_close() {
        
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(&format!("{} : {}", ball.pos.x, ball.pos.y), 10, 10, 20, Color::BLACK);
        d.draw_text(&format!("{} : {}", ball.vel.x, ball.vel.y), 10, 25, 20, Color::BLACK);
        d.draw_circle_v(ball.pos, 50.0, Color::MAROON);
        ball.update_vel(1.0);
        ball.update_pos(1.0);
        if (ball.pos.y + 50.0 > 480.0) {
            ball.set_vel(Vector2 { x: ball.vel.x, y: (ball.vel.y * -1.0)+dampening_factor });
            ball.set_pos(Vector2 { x: ball.pos.x, y: 429.9 });
        }
    }
}