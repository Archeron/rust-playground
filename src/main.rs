mod physx;

use physx::PhysxObject;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640,480)
        .title("test")
        .build();

    rl.set_target_fps(30);

    let mut balls:  Vec<PhysxObject> = Vec::new();

    let mut ball: PhysxObject = PhysxObject::new(
        Vector2 { x: 50.0, y: 200.0 }, 
        Vector2 { x: 3.0, y: 0.0 }, 
        Vector2 { x: 0.0, y: 3.0 }, 
        50.0,
        2.0 );

    ball.set_accel(Vector2 { x: 0.0, y: 1.0 });
    ball.set_vel(Vector2 { x: 3.0, y: 0.0 });
    
    balls.push(ball);

    while !rl.window_should_close() {
        
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        for ball in balls.iter_mut() {
            d.draw_circle_v(ball.pos, ball.radius, Color::MAROON);
            ball.update_vel(1.0);
            ball.update_pos(1.0);
            if ball.pos.y + 50.0 > 480.0 {
                ball.set_vel(Vector2 { x: ball.vel.x, y: (ball.vel.y * -1.0)+ball.dampening_factor });
                ball.set_pos(Vector2 { x: ball.pos.x, y: 429.9 });
            }
            if (ball.pos.x + 50.0 > 640.0) || (ball.pos.x < 50.0) {
                ball.vel.x *= -1.0;
            }
        }
        balls.retain(|x| (x.vel.y.abs() > 0.1) || (x.pos.y < 429.0));

        if (d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)) {
                let mut mousex = d.get_mouse_x() as f32;
                let mut mousey = d.get_mouse_y() as f32;

                match mousex < 50.0 {
                    true => mousex = 50.0,
                    false => (),
                }
                match mousex > 590.0 {
                    true => mousex = 590.0,
                    false => (),
                }
                match mousey < 50.0 {
                    true => mousey = 50.0,
                    false => (),
                }
                match mousey > 430.0 {
                    true => mousey = 430.0,
                    false => (),
                }

            let mut new_ball: PhysxObject = PhysxObject::new(
                Vector2 { x: mousex, y: mousey }, 
                Vector2 { x: 3.0, y: 0.0 }, 
                Vector2 { x: 0.0, y: 1.0 }, 
                50.0,
                2.0 );
            balls.push(new_ball);
        }
    }
}