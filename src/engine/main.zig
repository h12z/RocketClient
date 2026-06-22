const std = @import("std");
const gl = @import("webgl.zig");

const allocator = std.heap.smp_allocator;

export fn init(width: i32, height: i32) void {
    gl.log("RocketClient Engine Initialized!");
    gl.gl_viewport(0, 0, width, height);
}

export fn update(time: f32) void {
    // Let's pulse the background color slightly to show it's working
    const r = (std.math.sin(time) + 1.0) / 2.0 * 0.1;
    const g = (std.math.cos(time) + 1.0) / 2.0 * 0.1;

    // Clear to a very dark "Amber" tinted black
    gl.gl_clearColor(r + 0.05, g + 0.05, 0.0, 1.0);
    gl.gl_clear(gl.COLOR_BUFFER_BIT);
}

export fn resize(width: i32, height: i32) void {
    gl.gl_viewport(0, 0, width, height);
}

export fn alloc(len: usize) [*]u8 {
    const slice = allocator.alloc(u8, len) catch @panic("Out of memory");
    return slice.ptr;
}
