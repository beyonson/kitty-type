use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::color;
use termion::raw::IntoRawMode;


fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    let mut cursor_x = 1;
    let mut buffer: String = "".to_owned();
    let mut prompt = "kitty type".chars();
    let prompt_len = prompt.as_str().len();
    let mut mistakes = 0;

    print_prompt(&mut stdout, prompt.as_str());

    for k in stdin.keys() {
        let current_key = prompt.nth(0).unwrap();
        write!(stdout,
            "{}{}",
            termion::color::Bg(color::LightWhite),
            termion::cursor::Goto(cursor_x, 1)
        )
        .unwrap();

        // Count mistakes
        if *k.as_ref().unwrap() != Key::Char(current_key) {
            write!(stdout, 
                "{}{}{}", 
                termion::color::Bg(color::LightRed), 
                termion::cursor::Goto(cursor_x, 1),
                current_key.to_string()
            )
            .unwrap();
            mistakes += 1;
        } else {
            write!(stdout, 
                "{}{}", 
                termion::cursor::Goto(cursor_x, 1),
                current_key.to_string()
            )
            .unwrap();
        }

        match k.as_ref().unwrap() {
            Key::Char('q') => break,
            Key::Char('-') => println!("{buffer}"),
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
            complete_test(&mut stdout, mistakes as f32, prompt_len as f32);
            break;
        }

        cursor_x += 1;
        stdout.flush().unwrap();
    }

    write!(
        stdout,
        "{}{}{}",
        termion::color::Fg(color::Reset),
        termion::color::Bg(color::Reset),
        termion::cursor::Show
    )
    .unwrap();
}


// Print completion and compute accuracy
fn complete_test(stdout: &mut std::io::Stdout, mistakes: f32, prompt_len: f32) {
    let accuracy = 100.0*((prompt_len - mistakes)/prompt_len);
    write!(
        stdout,
        "{}{}{}{}{}{}{}",
        termion::cursor::Goto(1, 3),
        termion::color::Fg(color::Blue),
        termion::color::Bg(color::Reset),
        "You done, accuracy: ", 
        accuracy.to_string(),
        "%", 
        termion::cursor::Goto(1, 4)
    )
    .unwrap();
    stdout.flush().unwrap();
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

