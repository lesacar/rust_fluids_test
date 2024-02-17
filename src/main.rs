use raylib::{ffi::{IsMouseButtonDown, IsKeyPressed, ToggleFullscreen, GetScreenWidth, GetScreenHeight, KeyboardKey::*, SetTargetFPS, GetFrameTime, SetWindowPosition}, prelude::*};
use raylib::consts::*;
use rand::Rng;
use std::collections::HashSet;
use rand::seq::SliceRandom; // Add this import

const SCREEN_W: i32 = 1440;
const SCREEN_H: i32 = 720;
const RECT_SIZE: i32 = 4;

fn main() {
    let mut rng = rand::thread_rng();
    set_trace_log(TraceLogLevel::LOG_ERROR);
    unsafe {
        SetTargetFPS(75);
        SetWindowPosition(1000, 1000);
    }

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W as i32, SCREEN_H as i32)
        .title("Rust raylib window")
        .msaa_4x()
        .build();
    let mut cur_dt: f32 = 0.0;
    let mut perf_limiter = 0.0;
    let mut drawn_rectangles = HashSet::<(i32, i32)>::new(); // Define HashSet with (i32, i32) elements

    while !rl.window_should_close() {
        if cur_dt > 0.0 {
            perf_limiter += cur_dt;
        }
        cur_dt = unsafe { GetFrameTime() };

        // DRAWING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Redraw all previously drawn rectangles
        for &(mx, my) in &drawn_rectangles {
            let grid_x = (mx / RECT_SIZE) * RECT_SIZE;
            let grid_y = (my / RECT_SIZE) * RECT_SIZE;
            d.draw_rectangle(grid_x, grid_y, RECT_SIZE, RECT_SIZE, Color::GREEN);
        }

        // Make sand rectangles fall down
        let mut updated_rectangles = HashSet::new();
        for &(mx, my) in &drawn_rectangles {
            let below_y = my + RECT_SIZE;

            let can_move_down = below_y < SCREEN_H &&
                (!drawn_rectangles.contains(&(mx, below_y)) ||
                (!drawn_rectangles.contains(&(mx - RECT_SIZE, below_y)) &&
                !drawn_rectangles.contains(&(mx + RECT_SIZE, below_y))));

            // Check if the sand column is exactly 2 blocks high
            let is_two_blocks_high = drawn_rectangles.contains(&(mx, my - RECT_SIZE)) && !drawn_rectangles.contains(&(mx, my - 2 * RECT_SIZE));

            // If the sand column is exactly 2 blocks high and there's space below, stack rectangles vertically
            if is_two_blocks_high && can_move_down {
                updated_rectangles.insert((mx, below_y));
            } else {
                // Otherwise, apply regular sand falling logic
                let mut moves = Vec::new();
                if can_move_down {
                    updated_rectangles.insert((mx, below_y));
                } else {
                    if !drawn_rectangles.contains(&(mx - RECT_SIZE, below_y)) {
                        moves.push((mx - RECT_SIZE, my));
                    }
                    if !drawn_rectangles.contains(&(mx + RECT_SIZE, below_y)) {
                        moves.push((mx + RECT_SIZE, my));
                    }
                    moves.shuffle(&mut rng);
                    for &(x, y) in moves.iter() {
                        updated_rectangles.insert((x, y));
                    }
                    if moves.is_empty() {
                        updated_rectangles.insert((mx, my));
                    }
                }
            }
        }

        // Update drawn rectangles with the new positions
        drawn_rectangles = updated_rectangles;

        // Draw new sand rectangle if left mouse button is down
        if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let mx = d.get_mouse_x();
            let my = d.get_mouse_y();
            let grid_x = (mx / RECT_SIZE) * RECT_SIZE;
            let grid_y = (my / RECT_SIZE) * RECT_SIZE;
            let position = (grid_x, grid_y);
            if !drawn_rectangles.contains(&position) {
                drawn_rectangles.insert(position);
            }
        }
        if perf_limiter > 1.0 {
            perf_limiter = 0.0;
        }
    }
}
