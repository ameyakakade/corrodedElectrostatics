pub mod raylib;
use raylib::*;

fn main() {
    init_window(400, 600, "Hello from rust!");
    let img = load_texture("wow.png");
    while !window_should_close() {
        begin_drawing();
        clear_background(0x44DD4433);
        draw_texture(img, 0, 0, get_color(0x000044FF));
        end_drawing();
    }
}
