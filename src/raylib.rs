use std::ffi::CString;

unsafe extern "C" {
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: u64);
    fn InitWindow(width: i32, height: i32, string: *const i8);
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
    let w = CString::new(string).unwrap().into_raw();
    unsafe {
        InitWindow(width, height, w);
    }
}
