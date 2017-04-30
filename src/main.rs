extern crate pancurses;

use std::thread;
use std::time::Duration;
use pancurses::{initscr, endwin, Window, newwin, Input, noecho, curs_set};
const FIELD_CHARACTER: char = 'a';

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Snake {
    head: Point,
    direction: Direction
}

impl Snake {
    fn move_body (&mut self) {
        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Right => self.head.x += 1,
            Direction::Left => self.head.x -= 1
        }
    }

    fn change_direction (&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }
}

fn draw_field(main_window: &Window, snake: &Snake) {
    main_window.clear();
    main_window.mv(snake.head.y, snake.head.x);
    main_window.addch(FIELD_CHARACTER);
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
    main_window.nodelay(true);
    main_window.keypad(true);
    noecho();
    curs_set(0);

    let mut player = Snake {
        direction: Direction::Down,
        head: Point {
            x: 5,
            y: 5
        }
    };

    loop {
        draw_field(&main_window, &player);
        player.move_body();

        let keypress = main_window.getch();

        get_direction_from_keypress(keypress)
            .map(|direction| player.change_direction(direction));

        let sleep_time = Duration::from_millis(500);
        thread::sleep(sleep_time)
    }

    // endwin();
}
