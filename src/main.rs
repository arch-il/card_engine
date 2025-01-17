#[allow(unused)]
mod card;

use macroquad::{color, window};

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Card Engine".to_owned(),
        window_resizable: false,
        window_width: 806,
        window_height: 500,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        window::clear_background(color::BLACK);

        window::next_frame().await
    }
}
