use std::ffi::*;

unsafe extern "C" {
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: u64);
    fn InitWindow(width: i32, height: i32, string: *const i8);
    fn ImageDrawRectangle(
        dst: *const Image,
        posX: i32,
        posY: i32,
        width: i32,
        height: i32,
        color: Color,
    );
    fn UnloadImage(image: *mut Image);
    fn LoadImage(fileName: *const i8) -> Image;
    fn LoadTexture(fileName: *const i8) -> Texture;
    fn DrawTexture(texture: Texture, posX: i32, posY: i32, tint: Color);
}

pub fn end_drawing() {
    unsafe {
        EndDrawing();
    }
}
pub fn clear_background(color: u64) {
    unsafe {
        ClearBackground(color);
    }
}
pub fn begin_drawing() {
    unsafe {
        BeginDrawing();
    }
}
pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

pub fn init_window(width: i32, height: i32, string: &str) {
    let ptr = CString::new(string).unwrap();
    unsafe {
        InitWindow(width, height, ptr.as_ptr());
    }
}

pub fn load_image(file_name: &str) -> Image {
    let ptr = CString::new(file_name).unwrap();
    unsafe { LoadImage(ptr.as_ptr()) }
}

pub fn image_draw_rectangle(
    dst: &Image,
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: Color,
) {
    unsafe {
        ImageDrawRectangle(dst, pos_x, pos_y, width, height, color);
    }
}

pub fn get_color(hex_value: u32) -> Color {
    let r = (hex_value & 0xFF000000) as u8;
    let g = (hex_value & 0x00FF0000) as u8;
    let b = (hex_value & 0x0000FF00) as u8;
    let a = (hex_value & 0x000000FF) as u8;
    Color { r, g, b, a }
}

pub fn load_texture(file_name: &str) -> Texture {
    let ptr = CString::new(file_name).unwrap();
    unsafe { LoadTexture(ptr.as_ptr()) }
}

pub fn draw_texture(texture: Texture, pos_x: i32, pos_y: i32, color: Color) {
    unsafe { DrawTexture(texture, pos_x, pos_y, color) }
}

#[repr(C)]
pub struct Vector2 {
    x: f32,
    y: f32,
}
#[repr(C)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
pub struct Image {
    data: *mut c_void,
    width: i32,
    height: i32,
    mipmaps: i32,
    format: i32
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
    mipmaps: i32,
    format: i32
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            UnloadImage(self);
        }
    }
}
