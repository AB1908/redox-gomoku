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
    // player_here: bool,
    status: CellStatus,
    x: u16,
    y: u16,
}

struct Game<R, W: Write> {
    player1_turn: bool,
    status: bool,
    x: u16,
    y: u16,
    width: u16,
    grid: Box<[Cell]>,
    stdout: W,
    stdin: R,
}

/// Initialize the game.
fn init<W: Write, R: Read>(mut stdout: W, stdin: R, w: u16, h: u16) {
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // Set the initial game state.
    let mut game = Game {
        player1_turn: true,
        x: 1,
        y: 0,
        status: false,
        width: w,
        grid: vec![Cell {
            // player_here: false,
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
    game.start();
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Game<R, W> {
    
    fn pos(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize

    }

    fn read_cell(&mut self, c: usize) -> CellStatus {
        self.grid[c].status
    }

    fn get(&mut self, x: u16, y: u16) -> Cell {
        let pos = self.pos(x, y);

        self.read_cell(pos);
        self.grid[pos]
    }

    /// Get a mutable reference to the cell at (x, y).
    fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {

        let pos = self.pos(x, y);
        self.read_cell(pos);
        &mut self.grid[pos]
    }

    fn draw_tile_row(&mut self, y: u16) {
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
        self.stdout.flush().unwrap();
    }

    fn start(&mut self) {
        // let mut first_click = true;
        self.draw_grid();
        // write!(self.stdout, "{}", cursor::Goto(4,4)).flush().unwrap();
        loop {
            // Read a single byte from stdin.
            let b = self.stdin.next().unwrap().unwrap();
            use termion::event::Key::*;
            match b {
                Char('h') | Char('a') | Left  => self.x = self.left(self.x),
                Char('j') | Char('s') | Down  => self.y = self.down(self.y),
                Char('k') | Char('w') | Up    => self.y = self.up(self.y),
                Char('l') | Char('d') | Right => self.x = self.right(self.x),
                Char('\n') | Char('o') => self.enter(),
                    // Check if it was a mine.
                    // let (x, y) = (self.x, self.y);

            //        if first_click {
            //            // This is the player's first turn; clear all cells of
            //            // mines around the cursor.
            //            for &(x, y) in self.adjacent(x, y).iter() {
            //                self.get_mut(x, y).mine = false;
            //            }
            //            self.get_mut(x, y).mine = false;
            //            first_click = false;
                //    }

            //        if self.get(x, y).mine {
            //            self.reveal_all();
            //            // Make the background colour of the mine we just
            //            // landed on red, and the foreground black.
            //            write!(self.stdout, "{}{}{}{}{}",
            //                   cursor::Goto(x + 2, y + 2),
            //                   color::Bg(color::Red), color::Fg(color::Black),
            //                   MINE,
            //                   style::Reset).unwrap();
            //            self.game_over();
            //            return;
            //        }

            //        if !self.get(x, y).revealed {
            //            self.points += 1;
            //        }

            //        // Reveal the cell.
            //        self.reveal(x, y);

            //        self.print_points();
            //    },
            //    Char('f') => {
            //        let (x, y) = (self.x, self.y);
            //        self.toggle_flag(x, y);
            //    }
            //    Char('r') => {
            //        self.restart();
            //        return;
            //    }
               Char('q') => {
                //    write!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();
                   write!(self.stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
                   return
                   },
               _ => {},
            }

            // Make sure the cursor is placed on the current position.
            write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2)).unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn height(&self) -> u16 {
        (self.grid.len() / self.width as usize) as u16
    }

    fn up(&self, y: u16) -> u16 {
        if y <= 1 {
            // Upper bound reached. Wrap around.
            29 - 1
        } else {
            y - 2
        }
    }
    /// Calculate the y coordinate of the cell "below" a given y coordinate.
    ///
    /// This wraps when _y = h - 1_.
    fn down(&self, y: u16) -> u16 {
        if y + 2 >= 29 {
            // Lower bound reached. Wrap around.
            0
        } else {
            y + 2
        }
    }
    /// Calculate the x coordinate of the cell "left to" a given x coordinate.
    ///
    /// This wraps when _x = 0_.
    fn left(&self, x: u16) -> u16 {
        if x <= 3 {
            // Lower bound reached. Wrap around.
            60 - 3
        } else {
            x - 4
        }
    }
    /// Calculate the x coordinate of the cell "left to" a given x coordinate.
    ///
    /// This wraps when _x = w - 1_.
    fn right(&self, x: u16) -> u16 {
        if x + 4 >= 60 {
            // Upper bound reached. Wrap around.
            1
        } else {
            x + 4
        }
    }

    fn translate_x_address(&self, x: u16) -> u16 {
        (x + 1 + 2) / 4
    }

    fn translate_y_address(&self, y: u16) -> u16 {
        (y + 2 + 1) / 2
    }

    fn enter(&mut self) {
        // move cursor back and overwrite cell contents
        if self.player1_turn {
            write!(self.stdout, "{}{}", cursor::Goto(self.x + 1, self.y + 2), PLAYER1).unwrap();
            
        }
        else {
            write!(self.stdout, "{}{}", cursor::Goto(self.x + 1, self.y + 2), PLAYER2).unwrap();
        }
        self.player1_turn = !self.player1_turn;
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    let termsize = termion::terminal_size().ok().unwrap();
    // let termheight = ;
    // print_grid(&mut stdout);
    write!(stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
    init(stdout, stdin, 15, 15);
}
