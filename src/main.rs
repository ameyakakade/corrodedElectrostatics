pub mod raylib;
use raylib::*;

fn main() {
    println!("Hello, world!");
    init_window(400, 600, "Hello");
    while !window_should_close() {
        begin_drawing();
        clear_background(0x44334433);
        end_drawing();
    }
}
