use ncurses::*;

use std::{env, process::exit, io::BufRead};
use std::{thread, time::Duration};

use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;

use termion::{color, style, terminal_size};


fn main() {
    let args: Vec<_> = env::args().collect();
    let (_, h) = termion::terminal_size().unwrap();

    if args.len()-1 < 2 {
        println!("{}{}err{}: Supplied {:?}/2 arguments!", color::Fg(color::Red), style::Bold, color::Fg(color::White), args.len()-1);
        println!("{}{} | {}  'timer [message] [hh:mm:ss]'", color::Fg(color::Red), style::Bold, color::Fg(color::White));
        exit(1);
    }

    let re = Regex::new(r"^\d{2}:\d{2}:\d{2}$").unwrap();
    if !re.is_match(&args[2]) {
        println!("{}{}err{}: Incorrect time formatting '{}'", color::Fg(color::Red), style::Bold, color::Fg(color::White), args[2]);
        println!("{}{} | {}  'timer [message] [hh:mm:ss]'", color::Fg(color::Red), style::Bold, color::Fg(color::White));
        exit(1);
    }

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    init_pair(2, COLOR_WHITE, COLOR_BLACK);
    init_pair(3, COLOR_RED, COLOR_BLACK);

    attron(A_BOLD());

    attron(COLOR_PAIR(2));
    print_centered((h-(h/8)) as i32, "press 'q' to exit".to_string());

    attron(COLOR_PAIR(1));
    print_centered(((h/2)-4) as i32, args[1].clone());

    start_countdown();

    // Wait for Q key
    let mut quit = false;
    while !quit {
        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
             _  => {}
        }
    }

    endwin();
}

fn start_countdown() {
    // Countdown thread

    thread::spawn(|| {
        let args: Vec<_> = env::args().collect();
        let (_, h) = termion::terminal_size().unwrap();

        let raw: Vec<_> = args[2].split(':').collect();
        let mut sec: i32 = raw[2].parse().unwrap();
        let mut min: i32 = raw[1].parse().unwrap();
        let mut hrs: i32 = raw[0].parse().unwrap();

        print_centered(((h/2)-2) as i32, hrs.to_string()+"h "+&min.to_string()+"m "+ &sec.to_string()+"s");

        // Countdown loop
        let mut broke = false;
        while !broke {
            thread::sleep(Duration::from_secs(1));
            print_centered(((h/2)-2) as i32, hrs.to_string()+"h "+&min.to_string()+"m "+ &sec.to_string()+"s");

            // Check if time has end or if someone is dumb and typed 00:00:00
            if sec == 0 && min == 0 && hrs == 0 {
                broke = true;
            }

            // Countdown logic
            match sec as i32 {
                0 => { 
                    if  min != 0 {
                        min -= 1; 
                        sec += 60;
                    }
                },
                _ => sec -= 1
            }
            print_centered(((h/2)-2) as i32, hrs.to_string()+"h "+&min.to_string()+"m "+ &sec.to_string()+"s");

            match min as i32 {
                0 => {
                    if sec == 0 && min == 0 {
                        if hrs == 0 {
                            attron(COLOR_PAIR(3));
                            print_centered(((h/2)-2) as i32, "0h 0m 0s".to_string());
                            broke = true;
                        }

                        hrs -= 1;
                        min += 59;
                        sec += 60;
                    }
                },
                _ => {}
            }
        }  

        while broke {
            thread::sleep(Duration::from_secs(1));
            attron(COLOR_PAIR(2));
            print_centered(((h/2)-2) as i32, "0h 0m 0s".to_string());

            thread::sleep(Duration::from_secs(1));
            attron(COLOR_PAIR(3));
            print_centered(((h/2)-2) as i32, "0h 0m 0s".to_string());
        }
    });
}

fn print_centered(start_row: i32, text: String) {
    // Calculate position
    let (w, _) = terminal_size().unwrap();
    let half_len = UnicodeSegmentation::graphemes(&text as &str, true).count() / 2;
    let adjusted_col = w/2 - half_len as u16;

    // Clear line that will be printed a text
    mv(start_row, 0);
    clrtoeol();

    // Print text and refresh
    mvprintw(start_row, adjusted_col as i32, &text as &str);
    refresh();
}