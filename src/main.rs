extern crate termion;

// use std::fmt::Write;
use std::env;
use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};
use termion::*;
use std::string::String;

const top_left: &'static str = "┌";
const top: &'static str = "┬";
const top_right: &'static str = "┐";
const vert: &'static str = "│";
const left: &'static str = "├";
const right: &'static str = "┤";
const inside: &'static str = "┼";
const bot_left: &'static str = "└";
const bot: &'static str = "┴";
const bot_right: &'static str = "┘";
const horz: &'static str = "---";
const cell: &'static str = "   ";
const player1: &'static str = " X ";
const player2: &'static str = " O ";
const player_marker: &'static str = " # ";

fn print_grid(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    for i in 0..31 {
        create_row(stdout, i);
    }
}

fn create_row(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, row_num: i32) {
    let mut row;
    if row_num == 0 {
        row = String::from(top_left);
        row.push_str(horz);
    } else if row_num == 30 {
        row = String::from(bot_left);
        row.push_str(horz);
    } else if row_num % 2 == 0 {
        row = String::from(left);
        row.push_str(horz);
    } else {
        row = String::from(vert);
    }

    for i in 0..14 {
        if row_num == 0 {
            row.push_str(top);
            row.push_str(horz);
        } else if row_num == 30 {
            row.push_str(bot);
            row.push_str(horz);
        } else if row_num % 2 == 1 {
            row.push_str(cell);
            row.push_str(vert);
        } else if row_num % 2 == 0 {
            row.push_str(inside);
            row.push_str(horz);
        }
    }
    // row.push_str(horz);
    if row_num == 0 {
        row.push_str(top_right);
    } else if row_num == 30 {
        row.push_str(bot_right);
    } else if row_num % 2 == 1 {
        row.push_str(cell);
        row.push_str(vert);
    } else {
        row.push_str(right);
    }
    stdout.write(row.as_bytes()).unwrap();
    write!(stdout, "\n\r");
    // writeln!(stdout, "{}", row);
    // println!("{}", row);
    // write!(stdout, "{}", row).unwrap();
}

fn main() {
    // println!("{}", clear::All);
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    // write!(stdout, "Gomoku for Redox");
    //  write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}",
    //       // Clear the screen.
    //       termion::clear::All,
    //       // Goto (1,1).
    //       termion::cursor::Goto(1, 1),
    //       // Hide the cursor.
    //       termion::cursor::Hide).unwrap();
    //// Flush stdout (i.e. make the output appear).
    //stdout.flush().unwrap();
    //for c in stdin.keys() {
    //    // Clear the current line.
    //    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();
//
    //    // Print the key we type...
    //    match c.unwrap() {
    //        // Exit.
    //        Key::Char('q') => break,
    //        Key::Char(c)   => println!("{}", c),
    //        Key::Alt(c)    => println!("Alt-{}", c),
    //        Key::Ctrl(c)   => println!("Ctrl-{}", c),
    //        Key::Left      => println!("<left>"),
    //        Key::Right     => println!("<right>"),
    //        Key::Up        => println!("<up>"),
    //        Key::Down      => println!("<down>"),
    //        _              => println!("Other"),
    //    }
//
    //    // Flush again.
    //    stdout.flush().unwrap();
    //}
    //write!(stdout, "{}", termion::cursor::Show).unwrap();
    
    println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    // println!("Hello there!");
    let termsize = termion::terminal_size().ok().unwrap();
    let termwidth = termsize.0;
    let mut game_grid: [[bool; 15]; 15];
    // let termheight = termsize.1;
    let termheight = 15;
    let mut input = String::new();
    print_grid(&mut stdout);
    // while input != "q" {
    // println!("{}{}",termion::clear::All, termion::cursor::Goto(1,1));
    // for i in 0..termheight {
    // if i == 0 {
    // println!("{}{}{}{}", top_left, cell, top, top_right);
    // }
    // println!("├---┼---┼---┼---┼---┼---┼---┼---┼---┼---┼---┼---┼---┼---┼---┤");
    // println!("│   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │");
    // if i == termheight - 1 {
    // println!("{}{}{}{}", bot_left, cell, bot, bot_right);
    // }
    // }
    // io::stdin().read_line(&mut input).expect("error: unable to read user input");
    // let b = io::stdin().unwrap().unwrap();
    // if input == "d" {
    // termion::raw::IntoRawMode;
    // }
    // }
}