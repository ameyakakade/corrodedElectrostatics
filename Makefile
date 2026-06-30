main:
	rustc src/main.rs -C link-args="./thirdparty/raylib-6.0_macos/lib/libraylib.a" -l framework=IOKit -l framework=Cocoa -l framework=OpenGL
