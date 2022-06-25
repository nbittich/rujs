#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io::Write;

use rujs::{
    bindings::{js_Alloc, js_dostring, js_freestate, js_newstate, JS_STRICT},
    console::js_console_init,
    PKG_NAME, VERSION,
};

fn main() {
    clear_terminal();
    println!("{} v.{}\n", PKG_NAME, VERSION);
    arrow_print();

    unsafe {
        let J = js_newstate(js_Alloc::None, rujs::NULL, JS_STRICT as i32);
        js_console_init(J);

        let mut line = String::new();
        while std::io::stdin().read_line(&mut line).is_ok() {
            let _ = js_dostring(J, line.as_ptr() as *const i8);
            line.clear();
            arrow_print();
        }
        js_freestate(J);
    }
}

fn arrow_print() {
    print!(">> ");
    std::io::stdout().flush().unwrap();
}
fn clear_terminal() {
    if cfg!(unix) {
        let _ = std::process::Command::new("clear").status();
    } else if cfg!(windows) {
        let _ = std::process::Command::new("cls").status();
    } else {
        eprintln!("cannot clear the terminal for the target os");
    };
}
