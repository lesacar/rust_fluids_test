use raylib::{ffi::{IsMouseButtonDown, IsKeyPressed, ToggleFullscreen, GetScreenWidth, GetScreenHeight, KeyboardKey::*, SetTargetFPS, GetFrameTime, SetWindowPosition}, prelude::*};
use raylib::consts::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom; // Add this import
use std::io::*;

// Constants
const SCREEN_W: i32 = 1440;
const SCREEN_H: i32 = 720;
const RECT_SIZE: i32 = 4;
const GRID_CELL_SIZE: i32 = 10;


fn main() {
    let mut rng = rand::thread_rng();
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

    let mut grid: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let mut last_fall_check_time = 0.0;

    while !rl.window_should_close() {
        if cur_dt > 0.0 {
            perf_limiter += cur_dt;
        }
        cur_dt = unsafe { GetFrameTime() };

        if cur_dt - last_fall_check_time > 0.05 {
            last_fall_check_time = cur_dt;
            update_sand_fall(&mut grid);
        }
        
        // DRAWING
        let mut d = rl.begin_drawing(&thread);
        unsafe {if IsKeyPressed(KEY_R as i32) {
            grid.clear();
        }}
        d.clear_background(Color::BLACK);
        d.draw_text(&(1.0/cur_dt).to_string(), 10, 10, 20, Color::WHITE);

        // Redraw all previously drawn rectangles
        for (_, rectangles) in &grid {
            for &(mx, my) in rectangles {
                d.draw_rectangle(mx, my, RECT_SIZE, RECT_SIZE, Color::GREEN);
            }
        }

        // Make sand rectangles fall down
        let mut updated_grid: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
        for (_, rectangles) in &grid {
            for &(mx, my) in rectangles {
                let below_y = my + RECT_SIZE;
                let can_move_down = below_y < SCREEN_H && !is_collision(&grid, mx, below_y);
                if can_move_down {
                    let new_position = (mx, below_y);
                    insert_into_grid(&mut updated_grid, new_position);
                } else {
                    let mut moves = Vec::new();
                    if !is_collision(&grid, mx - RECT_SIZE, below_y) {
                        moves.push((mx - RECT_SIZE, my));
                    }
                    if !is_collision(&grid, mx + RECT_SIZE, below_y) {
                        moves.push((mx + RECT_SIZE, my));
                    }
                    moves.shuffle(&mut rng);
                    for &(x, y) in moves.iter() {
                        insert_into_grid(&mut updated_grid, (x, y));
                    }
                    if moves.is_empty() {
                        insert_into_grid(&mut updated_grid, (mx, my));
                    }
                }
            }
        }

        // Update grid with the new positions
        grid = updated_grid;

        // Draw new sand rectangle if left mouse button is down
        if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let mx = d.get_mouse_x();
            let my = d.get_mouse_y();
            let grid_x = (mx / RECT_SIZE) * RECT_SIZE;
            let grid_y = (my / RECT_SIZE) * RECT_SIZE;
            let position = (grid_x, grid_y);
            insert_into_grid(&mut grid, position);
        }

        if perf_limiter > 1.0 {
            perf_limiter = 0.0;
        }
    }
}

fn is_collision(grid: &HashMap<(i32, i32), HashSet<(i32, i32)>>, x: i32, y: i32) -> bool {
    let cell_x = x / GRID_CELL_SIZE;
    let cell_y = y / GRID_CELL_SIZE;
    if let Some(rectangles) = grid.get(&(cell_x, cell_y)) {
        rectangles.contains(&(x, y))
    } else {
        false
    }
}

fn insert_into_grid(grid: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>, position: (i32, i32)) {
    let cell_x = position.0 / GRID_CELL_SIZE;
    let cell_y = position.1 / GRID_CELL_SIZE;
    grid.entry((cell_x, cell_y)).or_default().insert(position);
}

fn update_sand_fall(grid: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>) {
    let mut updated_grid: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    for ((_, _), rectangles) in grid.iter() {
        for &(mx, my) in rectangles {
            let below_y = my + RECT_SIZE;

            if below_y < SCREEN_H && !is_collision(grid, mx, below_y) {
                insert_into_grid(&mut updated_grid, (mx, below_y));
            } else {
                let mut moves = Vec::new();
                if !is_collision(grid, mx - RECT_SIZE, below_y) {
                    moves.push((mx - RECT_SIZE, my));
                }
                if !is_collision(grid, mx + RECT_SIZE, below_y) {
                    moves.push((mx + RECT_SIZE, my));
                }
                moves.shuffle(&mut rand::thread_rng());
                for &(x, y) in moves.iter() {
                    insert_into_grid(&mut updated_grid, (x, y));
                }
                if moves.is_empty() {
                    insert_into_grid(&mut updated_grid, (mx, my));
                }
            }
        }
    }

    *grid = updated_grid;
}