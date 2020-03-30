extern crate game_of_life;

use bbggez::ggez::conf::{FullscreenType, WindowMode};
use bbggez::ggez::event;
use bbggez::ggez::ContextBuilder;
use game_of_life::GameOfLife;

fn main() {
    let window_mode = WindowMode::default().dimensions(800.0, 800.0);
    let (mut context, mut event_loop) = ContextBuilder::new("Conway's Game of Life", "Brookzerker")
        .window_mode(window_mode)
        .build()
        .unwrap();

    let mut game_of_life = GameOfLife::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game_of_life) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
