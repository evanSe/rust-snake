extern crate pancurses;
extern crate rand;

use std::thread;
use std::time::Duration;
use pancurses::{initscr, endwin, Window, newwin, Input, noecho, curs_set};
use rand::Rng;
const FIELD_CHARACTER: char = 'a';
const FOOD_CHARACTER: char = 'b';

#[derive(Debug, PartialEq)]
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

impl Direction {
    fn get_opposite (direction: &Direction) -> Direction {
        match *direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }

    fn is_opposite (&self, direction: &Direction) -> bool {
        Direction::get_opposite(self) == *direction
    }
}


impl Snake {
    fn move_body (&mut self, is_growing: &bool, main_window_x: &i32, main_window_y: &i32) {
        self.body.insert(0, self.head);
        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Right => self.head.x += 1,
            Direction::Left => self.head.x -= 1
        };

        if self.head.y < 0 {
            self.head.y = *main_window_y - 1;
        };

        if self.head.x < 0 {
            self.head.x = *main_window_x - 1;
        };

        if self.head.y >= *main_window_y {
            self.head.y = 0;
        };

        if self.head.x >= *main_window_x {
            self.head.x = 0;
        };

        if !is_growing {
            self.body.pop();
        };
    }

    fn change_direction (&mut self, new_direction: Direction) {
        if !self.direction.is_opposite(&new_direction) {
            self.direction = new_direction;
        }
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
            Input::KeyUp | Input::Character('k') => Some(Direction::Up),
            Input::KeyDown | Input::Character('j') => Some(Direction::Down),
            Input::KeyRight | Input::Character('l') => Some(Direction::Right),
            Input::KeyLeft | Input::Character('h') => Some(Direction::Left),
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
    let (main_window_y, main_window_x) = main_window.get_max_yx();
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
    let mut speed = 100;
    loop {
        let food_eaten = player.eat(&food);
        draw_field(&main_window, &player, &food);
        player.move_body(&food_eaten, &main_window_x, &main_window_y);
        if food_eaten {
            if speed > 10 {
                speed -= 5;
            }
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

        let sleep_time = Duration::from_millis(speed);
        thread::sleep(sleep_time)
    }

    main_window.nodelay(false);

    let message = "winner winner chicken dinner!".to_string();
    main_window.mv(10, 10);
    for ref item in message.chars() {
        main_window.addch(*item);
    };
    main_window.refresh();
    let keypress = main_window.getch();
}
