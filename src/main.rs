extern crate termion;

use std::io;
use termion::*;
use std::env;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

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

fn print_grid() {
    for i in 0..31 {
        create_row(i);
    }
}

fn create_row(row_num: i32) {
	let mut row;
	if row_num == 0 {
		row = String::from(top_left);
		row.push_str(horz);
	}
	else if row_num == 30 {
		row = String::from(bot_left);
		row.push_str(horz);
	}
	else if row_num % 2 == 0 {
		row = String::from(left);
		row.push_str(horz);
	}
	else {
		row = String::from(vert);
	}

        for i in 0..14 {
                if row_num == 0 {
                    row.push_str(top);
					row.push_str(horz);
                }
                else if row_num == 30 {
                    row.push_str(bot);
					row.push_str(horz);
                }
				else if row_num % 2 == 1 {
					row.push_str(cell);
					row.push_str(vert);
				}
                else if row_num % 2 == 0 {
                    row.push_str(inside);
					row.push_str(horz);
                }
        }
		// row.push_str(horz);
        if row_num == 0 {
                row.push_str(top_right);
        }
        else if row_num == 30 {
                row.push_str(bot_right);
        }
        else if row_num % 2 == 1 {
				row.push_str(cell);
                row.push_str(vert);
        }
		else {
			row.push_str(right);
		}
	println!("{}", row);
}

fn main() {
	// println!("{}", clear::All);
	println!("{}{}",termion::clear::All, termion::cursor::Goto(1,1));
	let termsize = termion::terminal_size().ok().unwrap();
    	let termwidth = termsize.0;
	// let termheight = termsize.1;
	let termheight = 15;
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("error: unable to read user input");	
	print_grid();
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
