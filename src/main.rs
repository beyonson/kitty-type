use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


fn main() {
    // Declare stdio
    let mut stdout = stdout().into_raw_mode().unwrap();

    let stdin = stdin();
    let mut start_row = 3;
    let mut cursor_x = 1;
    let mut cursor_y = start_row;
    let mut buffer: String = "".to_owned();
    let test_text = "kitty type";

    write!(
        stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        test_text,
        termion::cursor::Hide
        //termion::clear::CurrentLine
    )
    .unwrap();
    stdout.flush().unwrap();

    for k in stdin.keys() {
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(cursor_x, cursor_y)
            //termion::clear::CurrentLine
        )
        .unwrap();

        match k.as_ref().unwrap() {
            Key::Char('q') => break,
            Key::Char('-') => println!("{buffer}"),
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Backspace => println!("×"),
            _ => {
                println!("{:?}", k)
            }
        }

        match k.unwrap() {
            Key::Char(k) => buffer.push(k),
            _ => {}
        }

        if buffer == test_text {
            write!(
                stdout,
                "{}{}",
                "You done",
                termion::cursor::Goto(1, cursor_y+1)
            )
            .unwrap();
            stdout.flush().unwrap();
            break;
        }

        // Increment cursor but stop on second line
        if cursor_x > 80 && cursor_y < start_row + 1 {
            cursor_x = 1;
            cursor_y += 1;
        } else if cursor_x > 80 {
            cursor_x = 1;
        } else {
            cursor_x += 1;
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
