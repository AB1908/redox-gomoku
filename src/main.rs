extern crate termion;

use std::io::Read;
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
const WELCOME_TEXT: &'static str = " 
-----------------------Welcome to Gomoku for Redox OS!-------------------------\n\r
The rules are simple:\n\r
1. Chain five or more pieces together to win!\n\r
2. Use WASD, vim bindings or the arrow keys to move around and Enter to select.\n\r
3. Press q to quit\n\r

                          Press Enter to start!\n\r
-------------------------------------------------------------------------------\n\r";

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
    status_player1: bool,
    status_player2: bool,
    x: u16,
    y: u16,
    grid: Box<[Cell]>,
    stdout: W,
    stdin: R,
}

/// Initialize the game.
fn init<W: Write, R: Read>(mut stdout: W, stdin: R, w: u16, h: u16) {
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    // write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", WELCOME_TEXT);

    // Set the initial game state.
    let mut game = Game {
        player1_turn: true,
        x: 1,
        y: 0,
        status_player1: false,
        status_player2: false,
        // width: w,
        grid: vec![
            Cell {
                // player_here: false,
                x: 0,
                y: 0,
                status: CellStatus::NotMarked,
            };
            w as usize * h as usize
        ].into_boxed_slice(),
        stdin: stdin.keys(),
        stdout: stdout,
    };

    game.start();
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> Game<R, W> {
    fn pos(&self, x: u16, y: u16) -> usize {
        y as usize * 15 + x as usize
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

    // TODO Since grid is initially empty, remove case matching and directly string together an
    // empty grid.
    fn draw_tile_row(&mut self, y: u16) {
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

    fn draw_horizontal_wall(&mut self) {
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

    fn draw_top_wall(&mut self) {
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

    fn draw_bottom_wall(&mut self) {
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

    fn draw_grid(&mut self) {
        write!(self.stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();
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
        use termion::event::Key::*;
        let mut ready = false;
        while !ready {
            let b = self.stdin.next().unwrap().unwrap();
            match b {
                Char('\n') => ready = true,
                Char('q') => {
                    write!(
                        self.stdout,
                        "{}{}{}",
                        clear::All,
                        style::Reset,
                        cursor::Goto(1, 1)
                    ).unwrap();
                    return;
                },
                _ => {},
            };
        }
        // write!(self.stdout, "{}", cursor::Goto(4,4)).flush().unwrap();
        self.draw_grid();
        while self.status_player1 == false && self.status_player2 == false {
            if self.player1_turn {
                write!(
                    self.stdout,
                    "{}{}",
                    cursor::Goto(1, 32),
                    "It's Player 1's turn."
                );
            } else {
                write!(
                    self.stdout,
                    "{}{}",
                    cursor::Goto(1, 32),
                    "It's Player 2's turn."
                );
            }
            // self.draw_grid();
            // Read a single byte from stdin.
            let b = self.stdin.next().unwrap().unwrap();
            match b {
                Char('h') | Char('a') | Left => self.x = self.left(self.x),
                Char('j') | Char('s') | Down => self.y = self.down(self.y),
                Char('k') | Char('w') | Up => self.y = self.up(self.y),
                Char('l') | Char('d') | Right => self.x = self.right(self.x),
                Char('\n') => self.enter(),
                Char('q') => {
                    write!(
                        self.stdout,
                        "{}{}{}",
                        clear::All,
                        style::Reset,
                        cursor::Goto(1, 1)
                    ).unwrap();
                    return;
                }
                _ => {}
            }

            write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2)).unwrap();
            // Make sure the cursor is placed on the current position.
            self.stdout.flush().unwrap();
        }

        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        if self.status_player1 {
            write!(self.stdout, "{}", "Player 1 has won the game!");
        } else {
            write!(self.stdout, "{}", "Player 2 has won the game!");
        }
    }

    fn up(&self, y: u16) -> u16 {
        if y <= 1 {
            // Upper bound reached. Wrap around.
            29 - 1
        } else {
            y - 2
        }
    }

    fn down(&self, y: u16) -> u16 {
        if y + 2 >= 29 {
            // Lower bound reached. Wrap around.
            0
        } else {
            y + 2
        }
    }

    fn left(&self, x: u16) -> u16 {
        if x <= 3 {
            // Lower bound reached. Wrap around.
            60 - 3
        } else {
            x - 4
        }
    }

    fn right(&self, x: u16) -> u16 {
        if x + 4 >= 60 {
            // Upper bound reached. Wrap around.
            1
        } else {
            x + 4
        }
    }

    fn translate_x_address(&self, x: u16) -> u16 {
        (x + 1) / 4
    }

    fn translate_y_address(&self, y: u16) -> u16 {
        (y + 1) / 2
    }

    fn mark_cell(&mut self, player1: bool, x: u16, y: u16) {
        let marker;
        let cellstat;
        if player1 {
            marker = PLAYER1;
            cellstat = CellStatus::Player1Marked;
        } else {
            marker = PLAYER2;
            cellstat = CellStatus::Player2Marked;
            // self.row_win(player1);
        }
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(self.x + 1, self.y + 2),
            marker
        ).unwrap();
        self.get_mut(x, y).status = cellstat;
        self.check_win(player1);
        self.player1_turn = !self.player1_turn;
    }

    fn enter(&mut self) {
        let x = self.translate_x_address(self.x);
        let y = self.translate_y_address(self.y);
        if self.player1_turn && self.get(x, y).status == CellStatus::NotMarked {
            self.mark_cell(true, x, y);
        } else if self.get(x, y).status == CellStatus::NotMarked {
            self.mark_cell(false, x, y);
        }
    }

    //traverse in opposite directions and allow win if summation is exactly five

    fn row_win(&mut self, player1: bool) {
        //argument for which player win
        let mut counter = 0;
        'rows: for _i in 0..15 {
            let mut _j = 0;
            'columns: while _j <= 10 {
                for _k in 0..5 {
                    if player1 && self.get(_j + _k, _i).status == CellStatus::Player1Marked {
                        counter += 1;
                    } else if !player1 && self.get(_j + _k, _i).status == CellStatus::Player2Marked
                    {
                        counter += 1;
                    } else {
                        _j = _j + _k;
                        break;
                    }
                }
                _j += 1;
                if counter == 5 {
                    if player1 {
                        self.status_player1 = true;
                    } else {
                        self.status_player2 = true;
                    }
                    break 'rows;
                }
                counter = 0;
            }
        }
    }

    fn column_win(&mut self, player1: bool) {
        let mut counter = 0;
        'columns: for _i in 0..15 {
            let mut _j = 0;
            'rows: while _j <= 10 {
                for _k in 0..5 {
                    if player1 && self.get(_i, _j + _k).status == CellStatus::Player1Marked {
                        counter += 1;
                    } else if !player1 && self.get(_i, _j + _k).status == CellStatus::Player2Marked
                    {
                        counter += 1;
                    } else {
                        _j = _j + _k;
                        break;
                    }
                }
                _j += 1;
                if counter == 5 {
                    if player1 {
                        self.status_player1 = true;
                    } else {
                        self.status_player2 = true;
                    }
                    break 'columns;
                }
                counter = 0;
            }
        }
    }

    fn leftdiag_win(&mut self, player1: bool) {
        let mut counter1 = 0;
        let mut counter2 = 0;
        let n = 15;
        for _slice in 0..(2 * n - 1) {
            let z = if _slice < n { 0 } else { _slice - n + 1 };
            for _j in z..(_slice - z + 1) {
                if self.get(_j, _slice - _j).status == CellStatus::Player1Marked && player1 == true
                {
                    counter1 += 1;
                } else if player1 == false
                    && self.get(_j, _slice - _j).status == CellStatus::Player2Marked
                {
                    counter2 += 1;
                }
            }
            if counter1 >= 5 && player1 {
                self.status_player1 = true;
                break;
            } else if counter2 >= 5 && player1 == false {
                self.status_player2 = true;
                break;
            }
            counter1 = 0;
            counter2 = 0;
        }
    }

    fn rightdiag_win(&mut self, player1: bool) {
        let mut counter1 = 0;
        let mut counter2 = 0;
        let n = 15;
        for _slice in 0..(2 * n - 1) {
            let z = if _slice < n { 0 } else { _slice - n + 1 };
            for _j in z..(_slice - z + 1) {
                if self.get(_j, n - 1 + _j - _slice).status == CellStatus::Player1Marked
                    && player1 == true
                {
                    counter1 += 1;
                } else if player1 == false
                    && self.get(_j, n - 1 + _j - _slice).status == CellStatus::Player2Marked
                {
                    counter2 += 1;
                }
            }
            if counter1 >= 5 && player1 {
                self.status_player1 = true;
                break;
            } else if counter2 >= 5 && player1 == false {
                self.status_player2 = true;
                break;
            }
            counter1 = 0;
            counter2 = 0;
        }
    }

    fn check_win(&mut self, player1: bool) {
        self.column_win(player1);
        self.row_win(player1);
        self.leftdiag_win(player1);
        self.rightdiag_win(player1);
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}",
        clear::All,
        style::Reset,
        cursor::Goto(1, 1)
    ).unwrap();
    init(stdout, stdin, 15, 15);
}
