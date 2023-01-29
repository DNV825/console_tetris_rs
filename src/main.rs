mod console_virtual_terminal;
mod field;
mod win_key_input;

use console_virtual_terminal as cvt;
use std::{thread, time};
use win_key_input as keyboard;
use win_key_input::{KeyKind, KeyState};

// https://crates.io/crates/termion
// https://docs.rs/termion/2.0.1/termion/
// https://qiita.com/hatoo@github/items/905a19a98876e7446edf
fn main() {
    let player1 = "@";
    let mut player1_pos_x = 1;
    let mut player1_pos_y = 1;

    cvt::use_alternate_screen_buffer(); // 代替画面にする。
    cvt::set_window_title("console tetris"); // 代替画面にタイトルを表示する。
    cvt::set_cursor_invisible(); // カーソル表示を消す。
    cvt::set_cursor_position(0, 0); // カーソル位置を0, 0にする。

    // テトリスのフィールドを描画する。
    let field = crate::field::Field::new();
    for field_line in field.game_area {
        for i in field_line {
            if i == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    let mut esc = keyboard::KeyInput::new(KeyKind::ESC);
    let mut space = keyboard::KeyInput::new(KeyKind::SPACE);
    let mut p = keyboard::KeyInput::new(KeyKind::P);
    let mut left_arrow = keyboard::KeyInput::new(KeyKind::LEFT_ARROW);
    let mut right_arrow = keyboard::KeyInput::new(KeyKind::RIGHT_ARROW);
    let mut down_arrow = keyboard::KeyInput::new(KeyKind::DOWN_ARROW);

    loop {
        if let KeyState::Pressed = left_arrow.get_key_state() {
            if player1_pos_x == 1 {
                player1_pos_x = 1;
            } else {
                player1_pos_x -= 1;
            }
        }

        if let KeyState::Pressed = right_arrow.get_key_state() {
            if player1_pos_x == 80 {
                player1_pos_x = 80;
            } else {
                player1_pos_x += 1;
            }
        }

        if let KeyState::Pressed = down_arrow.get_key_state() {
            if player1_pos_y == 30 {
                player1_pos_y = 30;
            } else {
                player1_pos_y += 1;
            }
        }

        // カーソル位置を(x,y) = (1,60) にする。原点は(1,1)。y = 30がWindows 11のターミナルのデフォルト最終行っぽい。
        let x = 60;
        let mut y = 1;
        let debug_print = | key:&str, x, y:&mut u32, state:&str | {
            cvt::set_cursor_position(x, *y);
            print!("{:6}: {:9}", key, state);
            *y += 1;
        };

        debug_print("esc", x, &mut y, esc.get_key_state_str());
        debug_print("[sp]", x, &mut y, space.get_key_state_str());
        debug_print("p", x, &mut y, p.get_key_state_str());
        debug_print("←", x, &mut y, left_arrow.get_key_state_str());
        debug_print("→", x, &mut y, right_arrow.get_key_state_str());
        debug_print("↓", x, &mut y, down_arrow.get_key_state_str());

        cvt::set_cursor_position(x, y);
        print!("(x, y):({:2}, {:2})", player1_pos_x, player1_pos_y);
        y += 1;

        cvt::set_cursor_position(player1_pos_x, player1_pos_y);
        print!("{}", player1);

        if let KeyState::Pressed = esc.get_key_state() {
            cvt::set_cursor_visible(); // カーソルを出す。
            cvt::use_main_screen_buffer(); // メイン画面
            break;
        }

        // 程よくplayerを動かすためにスリープを挟む。
        thread::sleep(time::Duration::from_millis(10));
    }
}
