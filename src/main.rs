//! rust_grammer_sample
//! oooo
mod console_virtual_terminal;
use console_virtual_terminal as cvt;
use std::io;

const  FIELD:[[i32; 10]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
];

// https://crates.io/crates/termion
// https://docs.rs/termion/2.0.1/termion/
// https://qiita.com/hatoo@github/items/905a19a98876e7446edf
fn main() {
    // テトリス 横10、縦20
    cvt::use_alternate_screen_buffer();
    cvt::set_window_title("rust_grammer_sample");
    cvt::set_cursor_position(0, 0);

    for field_line in FIELD {
        for i in field_line {
            if i == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let stdin = io::stdin();

    cvt::use_main_screen_buffer();

}