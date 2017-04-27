extern crate pancurses;

use pancurses::{initscr, endwin};
const FIELD_CHARACTER: &'static str = "■";

const FIELD_HEIGHT: u8 = 10;
const FIELD_WIDTH: u8 = 20;

fn main() {
  let window = initscr();
  window.printw("Hello Rust");
  window.border('║', '║', '═', '═', '╔', '╗', '╚', '╝');
  window.refresh();
  window.getch();
  endwin();
}
