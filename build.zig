const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.resolveTargetQuery(.{
        .cpu_arch = .wasm32,
        .os_tag = .freestanding,
    });

    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "game",
        .root_module = b.addModule("game", .{
            .root_source_file = b.path("src/engine/main.zig"),
            .target = target,
            .optimize = optimize,
            
        }),
    });

    // We don't want a main function, we'll export our own hooks
    exe.entry = .disabled;

    // Make sure exported functions are visible in the WASM module
    exe.rdynamic = true;

    b.installArtifact(exe);
    
}
