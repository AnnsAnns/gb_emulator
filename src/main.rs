use macroquad::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    start: bool,
    select: bool,
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut keys_down: HashSet<KeyCode>;
    let mut key_inputs = Input {
        start: false,
        select: false,
        up: false,
        right: false,
        down: false,
        left: false,
    };

    let start_key = KeyCode::Enter;
    let select_key = KeyCode::Tab;
    let up_key = KeyCode::Up;
    let right_key = KeyCode::Right;
    let down_key = KeyCode::Down;
    let left_key = KeyCode::Left;

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await;

        keys_down = get_keys_down();

        key_inputs.start = keys_down.contains(&start_key);
        key_inputs.select = keys_down.contains(&select_key);
        key_inputs.up = keys_down.contains(&up_key);
        key_inputs.right = keys_down.contains(&right_key);
        key_inputs.down = keys_down.contains(&down_key);
        key_inputs.left = keys_down.contains(&left_key);

        println!("{:?}", key_inputs);
    }
}
