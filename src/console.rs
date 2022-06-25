use std::ffi::CStr;

use crate::{
    bindings::{
        js_State, js_construct, js_currentfunction, js_defglobal, js_defproperty, js_getglobal,
        js_getproperty, js_newcconstructor, js_newcfunction, js_pushundefined, js_tostring,
        JS_DONTENUM,
    },
    OBJECT, PROTOTYPE,
};

pub const LOG_JS_FN: &CStr = crate::unsafe_cstr!("log");
pub const CONSOLE_OBJ_JS: &CStr = crate::unsafe_cstr!("Console");
pub const CONSOLE_INST_JS: &CStr = crate::unsafe_cstr!("console");

unsafe extern "C" fn log(J: *mut js_State) {
    let c = js_tostring(J, 1);
    let out = CStr::from_ptr(c);
    println!("{}", out.to_str().unwrap());
    js_pushundefined(J)
}

unsafe extern "C" fn js_console_new(J: *mut js_State) {
    js_currentfunction(J);
    js_getproperty(J, -1, PROTOTYPE.as_ptr());
}

/// # Safety
/// Trust me
pub unsafe extern "C" fn js_console_init(J: *mut js_State) {
    js_getglobal(J, OBJECT.as_ptr());
    js_getproperty(J, -1, PROTOTYPE.as_ptr());

    js_newcfunction(J, Some(log), LOG_JS_FN.as_ptr(), 1);
    js_defproperty(J, -2, LOG_JS_FN.as_ptr(), JS_DONTENUM as i32);

    js_newcconstructor(
        J,
        Some(js_console_new),
        Some(js_console_new),
        CONSOLE_OBJ_JS.as_ptr(),
        1,
    );
    js_defglobal(J, CONSOLE_OBJ_JS.as_ptr(), JS_DONTENUM as i32);
    js_getglobal(J, CONSOLE_OBJ_JS.as_ptr());

    js_construct(J, 0);
    js_defglobal(J, CONSOLE_INST_JS.as_ptr(), JS_DONTENUM as i32);
}
