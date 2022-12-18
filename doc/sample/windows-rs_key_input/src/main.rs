use  windows::Win32::UI::Input::KeyboardAndMouse;

fn main() {
    let key_esc = 0x1b; // esc
    let key_q = 0x51;   // q

    unsafe {
        loop {
            let is_key_esc_down = KeyboardAndMouse::GetAsyncKeyState(key_esc);
            let is_key_q_down = KeyboardAndMouse::GetAsyncKeyState(key_q);
            
            println!("esc_keydown: {}          ", is_key_esc_down);
            println!("  q_keydown: {}          ", is_key_q_down);
            print!("\x1bM");
            print!("\x1bM");
            if is_key_esc_down != 0 {
                break;
            }
        }
    }
}

