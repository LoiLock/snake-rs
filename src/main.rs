use std::collections::LinkedList;

use macroquad::prelude::*;

const SQUARE_SIZE: f32 = 10.0;

type Coordinate = (i16, i16);

struct Snake {
    head: Coordinate,
    dir: Coordinate,
    body: LinkedList<Coordinate>,
}

fn get_display_text(position: Coordinate, speed: f64, score: i32) -> String {
    format!(
        "pos: {:?},\nspeed: {}p/s\nscore: {}",
        position,
        1.0 / speed,
        score
    )
}

#[macroquad::main("Snake")]
async fn main() {
    let up = (0, -1);
    let down = (0, 1);
    let left = (-1, 0);
    let right = (1, 0);
    let mut last_update = get_time();

    let squares_x = screen_width() / SQUARE_SIZE;
    let squares_y = screen_height() / SQUARE_SIZE;

    let mut speed = 0.15;
    let startingpoint: Coordinate = (0, 0);

    let mut display_text = get_display_text(startingpoint.clone(), speed.clone(), 1);
    let font_size = 30.0;

    let mut snake = Snake {
        head: (0, 0),
        dir: right,
        body: LinkedList::new(),
    };
    let mut speed_multiplier = 1.0;

    rand::srand(macroquad::miniquad::date::now() as _);

    let mut fruit = (
        rand::gen_range(0, squares_x as i16),
        rand::gen_range(0, squares_y as i16),
    );

    let mut gameover = false;

    loop {
        if !gameover {
            clear_background(LIGHTGRAY);

            if is_key_down(KeyCode::Space) {
                speed_multiplier = 3.0;
            } else {
                speed_multiplier = 1.0;
            }

            // If 0.3 seconds has elapsed, update the playing field
            if get_time() - last_update > speed / speed_multiplier {
                last_update = get_time();

                // Add push head at beginning of body
                snake.body.push_front(snake.head);

                // Place head at current head position, and add the direction onto it
                snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);

                // If the snake's hits the fruit, don't remove the last list item, therefore increasing the length of the snake's body
                if snake.head == fruit {
                    fruit = (
                        rand::gen_range(0, squares_x as i16),
                        rand::gen_range(0, squares_y as i16),
                    );

                    speed *= 0.85;
                } else {
                    // Remove last body part (is increased every loop, this gets rid of it if we haven't eaten a fruit)
                    snake.body.pop_back();
                }

                // If any the snake's head hits any body part
                for (x, y) in &snake.body {
                    if *x == snake.head.0 && *y == snake.head.1 {
                        gameover = true;
                    }
                }
            }

            // Exit game loop whenever it's game over
            if gameover {
                continue;
            }

            display_text = get_display_text(
                snake.head,
                speed / speed_multiplier,
                snake.body.len() as i32 + 1,
            );

            // Don't allow movement that would result in colliding with yourself
            if is_key_down(KeyCode::Up) && snake.dir != down {
                snake.dir = up;
            }
            if is_key_down(KeyCode::Down) && snake.dir != up {
                snake.dir = down;
            }
            if is_key_down(KeyCode::Left) && snake.dir != right {
                snake.dir = left;
            }
            if is_key_down(KeyCode::Right) && snake.dir != left {
                snake.dir = right;
            }

            // Draw snake's head
            draw_rectangle(
                snake.head.0 as f32 * SQUARE_SIZE as f32,
                snake.head.1 as f32 * SQUARE_SIZE as f32,
                SQUARE_SIZE,
                SQUARE_SIZE,
                DARKGREEN,
            );

            // Draw snake's body
            for (x, y) in &snake.body {
                draw_rectangle(
                    *x as f32 * SQUARE_SIZE as f32,
                    *y as f32 * SQUARE_SIZE as f32,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                    DARKGREEN,
                )
            }

            // Draw fruit
            draw_rectangle(
                fruit.0 as f32 * SQUARE_SIZE as f32,
                fruit.1 as f32 * SQUARE_SIZE as f32,
                SQUARE_SIZE,
                SQUARE_SIZE,
                RED,
            );
        } else {
            display_text = "You lost".to_string();
            clear_background(WHITE);

            if is_key_down(KeyCode::Enter) {
                snake = Snake {
                    head: (0, 0),
                    dir: (1, 0),
                    body: LinkedList::new(),
                };
                rand::srand(macroquad::miniquad::date::now() as _);

                fruit = (
                    rand::gen_range(0, squares_x as i16),
                    rand::gen_range(0, squares_y as i16),
                );
                speed = 0.15;
                last_update = get_time();
                gameover = false;
            }
        }

        let text_size = measure_text(&display_text, None, font_size as _, 1.0);

        draw_text(
            &display_text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. - text_size.height / 2.,
            font_size,
            DARKGRAY,
        );

        next_frame().await
    }
}
