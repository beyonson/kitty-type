use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::color;
use termion::raw::IntoRawMode;


fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    let start_row = 3;
    let mut cursor_x = 1;
    let mut cursor_y = start_row;
    let mut buffer: String = "".to_owned();
    let mut prompt = "kitty type".chars();
    let prompt_len = prompt.as_str().len();
    let mut misses = 0;

    print_prompt(&mut stdout, prompt.as_str());

    for k in stdin.keys() {
        write!(stdout, "{}", termion::cursor::Goto(cursor_x, cursor_y)).unwrap();

        // Count mistypes
        if *k.as_ref().unwrap() != Key::Char(prompt.nth(0).unwrap()) {
            write!(stdout, "{}", termion::color::Fg(color::Red)).unwrap();
            misses += 1;
        } else {
            write!(stdout, "{}", termion::color::Fg(color::Black)).unwrap();
        }

        match k.as_ref().unwrap() {
            Key::Char('q') => break,
            Key::Char('-') => println!("{buffer}"),
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Backspace => println!("×"),
            _ => {}
        }

        // Push input key to input buffer
        match k.unwrap() {
            Key::Char(k) => buffer.push(k),
            _ => {}
        }
        
        // End condition
        if buffer.len() == prompt_len {
            write!(
                stdout,
                "{}{}{}{}",
                termion::cursor::Goto(1, cursor_y+1),
                "You done, mistakes: ",
                misses.to_string(),
                termion::cursor::Goto(1, cursor_y+2)
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

    write!(
        stdout,
        "{}{}",
        termion::color::Fg(color::Reset),
        termion::cursor::Show
    )
    .unwrap();
}


fn print_prompt(stdout: &mut std::io::Stdout, prompt: &str) {
    write!(
        stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        prompt,
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}
