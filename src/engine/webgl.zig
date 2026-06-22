pub extern "env" fn gl_clearColor(r: f32, g: f32, b: f32, a: f32) void;
pub extern "env" fn gl_clear(mask: u32) void;
pub extern "env" fn gl_viewport(x: i32, y: i32, width: i32, height: i32) void;

pub const COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const DEPTH_BUFFER_BIT: u32 = 0x00000100;

pub extern "env" fn js_log(ptr: [*]const u8, len: usize) void;

pub fn log(msg: []const u8) void {
    js_log(msg.ptr, msg.len);
}

pub extern "env" fn 
