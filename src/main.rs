#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::thread::yield_now;

use raylib::{ffi::{IsMouseButtonDown, IsKeyPressed, ToggleFullscreen, GetScreenWidth, GetScreenHeight, KeyboardKey::*, SetTargetFPS, GetFrameTime, SetWindowPosition}, prelude::*};
use raylib::consts::*;
use rand::Rng;

pub struct Sand {
    x: u8,
    y: u8,
}

fn main() {
    let mut screen_W = 640;
    let mut screen_H = 480;
    let mut rng = rand::thread_rng();
    set_trace_log(TraceLogLevel::LOG_ERROR);
    unsafe {
        SetTargetFPS(75);
        SetWindowPosition(1000,1000);
    }
    
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Rust raylib window")
        .msaa_4x()
        .resizable()
        .build();
    let mut cur_dt: f32 = 0.0;
    let mut perf_limiter = 0.0;
    let mut is_mouse_down = false;
    while !rl.window_should_close() {
        if cur_dt > 0.0 {
            perf_limiter += cur_dt;
        }
        let mut last_dt = cur_dt;
        cur_dt = unsafe {GetFrameTime()};
        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            is_mouse_down = true;
        } else {
            is_mouse_down = false;
        }
        // DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
        unsafe {
            if IsKeyPressed(KEY_F as i32) {
                ToggleFullscreen();
            }
            screen_W = GetScreenWidth();
            screen_H = GetScreenHeight();
        }
        
        let rand_x: u32 = rng.gen_range(0..screen_W) as u32;
        let rand_y: u32 = rng.gen_range(0..screen_H) as u32;
        if is_mouse_down { d.draw_pixel(d.get_mouse_x(), d.get_mouse_y(), Color::GREEN);}
        if perf_limiter > 1.0 {
            perf_limiter = 0.0;
        }
    }
}