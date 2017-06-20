extern crate pancurses;
extern crate rand;

use std::thread;
use std::time::Duration;
use pancurses::{initscr, endwin, Window, newwin, Input, noecho, curs_set};
use rand::Rng;
const FIELD_CHARACTER: char = 'a';
const FOOD_CHARACTER: char = 'b';

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}

#[derive(Debug)]
struct Snake {
    head: Point,
    direction: Direction,
    body: Vec<Point>
}

#[derive(Debug)]
struct Food {
    position: Point
}

impl Snake {
    fn move_body (&mut self, is_growing: &bool) {
        self.body.insert(0, self.head);
        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Right => self.head.x += 1,
            Direction::Left => self.head.x -= 1
        }
        if !is_growing {
            self.body.pop();
        }
    }

    fn change_direction (&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    fn detect_body_collision (&self) -> bool {
        self.body.iter()
            .any(|&body_part| body_part == self.head)
    }

    fn eat (&self, food: &Food) -> bool {
        food.position == self.head
    }
}

fn draw_field(main_window: &Window, snake: &Snake, food: &Food) {
    main_window.clear();

    main_window.mv(food.position.y, food.position.x);
    main_window.addch(FOOD_CHARACTER);

    main_window.mv(snake.head.y, snake.head.x);
    main_window.addch(FIELD_CHARACTER);
    for ref item in &snake.body {
        main_window.mv(item.y, item.x);
        main_window.addch(FIELD_CHARACTER);
    };

    main_window.refresh();
}

fn get_direction_from_keypress (keypress: Option<Input>) -> Option<Direction> {
    match keypress {
        Some(keypress) => match keypress {
            Input::KeyUp => Some(Direction::Up),
            Input::KeyDown => Some(Direction::Down),
            Input::KeyRight => Some(Direction::Right),
            Input::KeyLeft => Some(Direction::Left),
            _ => None
        },
        None => None
    }
}

fn main() {
    let border_window = initscr();
    border_window.border('|', '|', '-', '-', '+', '+', '+', '+');
    border_window.refresh();
    let (y, x) = border_window.get_beg_yx();
    let (lines, columns) = border_window.get_max_yx();
    let main_window = newwin(lines - 2, columns - 2, y + 1, x + 1);
    let (main_window_y, main_window_x) = border_window.get_max_yx();
    main_window.nodelay(true);
    main_window.keypad(true);
    noecho();
    curs_set(0);

    let mut player = Snake {
        direction: Direction::Down,
        body: vec!(),
        head: Point {
            x: 5,
            y: 5
        }
    };

    let mut rng = rand::thread_rng();
    let mut food = Food {
        position: Point {
            x: (rng.gen::<i32>() % main_window_x).abs(),
            y: (rng.gen::<i32>() % main_window_y).abs()
        }
    };

    loop {
        let food_eaten = player.eat(&food);
        draw_field(&main_window, &player, &food);
        player.move_body(&food_eaten);
        if food_eaten {
            food = Food {
                position: Point {
                    x: (rng.gen::<i32>() % main_window_x).abs(),
                    y: (rng.gen::<i32>() % main_window_y).abs()
                }
            };
        };

        match player.detect_body_collision() {
            true => break,
            false => ()
        };

        let keypress = main_window.getch();
        get_direction_from_keypress(keypress)
            .map(|direction| player.change_direction(direction));

        let sleep_time = Duration::from_millis(100);
        thread::sleep(sleep_time)
    }

    // endwin();
}
