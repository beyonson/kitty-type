use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut cursor_x = 1;
    let mut cursor_y = 1;
    let mut buffer: String = "".to_owned();

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

        // Increment cursor but stop on second line
        if cursor_x > 80 && cursor_y < 2 {
          cursor_x = 1;
          cursor_y = 2;
        } else if cursor_x > 80 {
          cursor_x = 1;
        } else {
          cursor_x += 1;
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
