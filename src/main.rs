use std::ffi::CString;

unsafe extern "C" {
    fn InitWindow(width: i32, height: i32, string: *mut i8);
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: u64);
}

fn main() {
    println!("Hello, world!");
    let winn = CString::new("Hi").unwrap().into_raw();
    unsafe {
        InitWindow(400, 600, winn);
        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(0x44334433);
            EndDrawing();
        }
    }
}
