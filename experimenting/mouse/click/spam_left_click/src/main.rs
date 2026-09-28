use std::{thread, time::Duration};

use enigo::{Button, Direction, Enigo, Mouse, Settings};

fn main() {
    let mut mouse = Enigo::new(&Settings::default()).expect("failed to initialize mouse input");

    loop {
        mouse
            .button(Button::Left, Direction::Click)
            .expect("failed to click the left mouse button");
        thread::sleep(Duration::from_secs(1));
    }
}
