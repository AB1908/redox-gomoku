extern crate termion;

// use std::fmt::Write;
use std::io::Read;
// use std::env;
// use std::io;
use std::io::{stdin, stdout, Write};
use std::string::String;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

const TOP_LEFT: &'static str = "┌";
const TOP: &'static str = "┬";
const TOP_RIGHT: &'static str = "┐";
const VERT: &'static str = "│";
const LEFT: &'static str = "├";
const RIGHT: &'static str = "┤";
const INSIDE: &'static str = "┼";
const BOT_LEFT: &'static str = "└";
const BOT: &'static str = "┴";
const BOT_RIGHT: &'static str = "┘";
const HORZ: &'static str = "---";
const CELL: &'static str = "   ";
const PLAYER1: &'static str = " X ";
const PLAYER2: &'static str = " O ";
//const PLAYER_MARKER: &'static str = " # ";

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum CellStatus {
    Player1Marked,
    Player2Marked,
    NotMarked,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Cell {
    player_here: bool,
    status: CellStatus,
    x: i32,
    y: i32,
}

struct Game<R, W: Write> {
    status: bool,
    x: i32,
    y: i32,
    width: i32,
    grid: Box<[Cell]>,
    stdout: W,
    stdin: R,
}

/// Initialize the game.
fn init<W: Write, R: Read>(mut stdout: W, stdin: R, w: i32, h: i32) {
    write!(stdout, "{}", clear::All).unwrap();

    // Set the initial game state.
    let mut game = Game {
        x: 0,
        y: 0,
        status: false,
        width: w,
        grid: vec![Cell {
            player_here: false,
            x: 0,
            y: 0,
            status: CellStatus::NotMarked,
        }; w as usize * h as usize].into_boxed_slice(),
        stdin: stdin.keys(),
        stdout: stdout,
    };

    // Reset that game.
    // game.reset();
    // game.draw_tile_row(2);
    game.draw_grid();
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Game<R, W> {
    
    fn pos(&self, x: i32, y: i32) -> usize {
        y as usize * self.width as usize + x as usize

    }

    fn read_cell(&mut self, c: usize) -> CellStatus {
        self.grid[c].status
    }

    fn get(&mut self, x: i32, y: i32) -> Cell {
        let pos = self.pos(x, y);

        self.read_cell(pos);
        self.grid[pos]
    }
/// Get a mutable reference to the cell at (x, y).
    fn get_mut(&mut self, x: i32, y: i32) -> &mut Cell {

        let pos = self.pos(x, y);
        self.read_cell(pos);
        &mut self.grid[pos]
    }

    fn draw_tile_row(&mut self, y: i32) {
        // let cell = self.get(x, y);
        let mut row;
        row = String::from("");
        for i in 0..15 {
        row.push_str(VERT);
        let cell = self.get(i, y);
        match cell.status {
            CellStatus::Player1Marked => row.push_str(PLAYER1),
            CellStatus::Player2Marked => row.push_str(PLAYER2),
            CellStatus::NotMarked => row.push_str(CELL),
        }
    }
    row.push_str(VERT);
    write!(self.stdout, "{}\n\r", row).unwrap();
    }

    fn draw_horizontal_wall(&mut self)

    {
        let mut row;
        row = String::from(LEFT);
        for _i in 0..14 {
            row.push_str(HORZ);
            row.push_str(INSIDE);
        }
        row.push_str(HORZ);
        row.push_str(RIGHT);
        write!(self.stdout, "{}\n\r", row).unwrap();
    }

    fn draw_top_wall(&mut self)
    {
        let mut row;
        row = String::from(TOP_LEFT);
        for _i in 0..14 {
            row.push_str(HORZ);
            row.push_str(TOP);
        }
        row.push_str(HORZ);
        row.push_str(TOP_RIGHT);
        write!(self.stdout, "{}\n\r", row).unwrap();
    }

    fn draw_bottom_wall(&mut self)
    {
        let mut row;
        row = String::from(BOT_LEFT);
        for _i in 0..14 {
            row.push_str(HORZ);
            row.push_str(BOT);
        }
        row.push_str(HORZ);
        row.push_str(BOT_RIGHT);
        write!(self.stdout, "{}", row).unwrap();
    }

    fn draw_grid(&mut self)
    {
        self.draw_top_wall();
        for i in 0..15 {
            self.draw_tile_row(i);
            if i < 14 {
                self.draw_horizontal_wall();
            }
        }
        self.draw_bottom_wall();
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    let termsize = termion::terminal_size().ok().unwrap();
    let termwidth = termsize.0;
    // let termheight = ;
    let mut input = String::new();
    // print_grid(&mut stdout);
    write!(stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
    init(stdout, stdin, 15, 15);
}
