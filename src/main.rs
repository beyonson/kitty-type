use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::color;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    // cursor x array: current position, max position
    let mut cursor_x = [1, 1];
    let mut prompt_pos = 0;
    let mut buffer: String = "".to_owned();
    let mut prompt = "kitty type".chars();
    let prompt_len = prompt.as_str().len();
    let mut mistakes = 0;
    let mut retracking = false;

    print_prompt(&mut stdout, prompt.as_str());
    let prompt_vec = create_char_vec(&mut prompt);

    for k in stdin.keys() {
        let current_key = prompt_vec[prompt_pos];
        if *k.as_ref().unwrap() == Key::Backspace {
            write!(stdout, 
                "{}", 
                termion::color::Bg(color::Reset), 
            )
            .unwrap();
            retracking = true;
        } else if *k.as_ref().unwrap() != Key::Char(current_key) {
            write!(stdout, 
                "{}{}{}", 
                termion::color::Bg(color::LightRed), 
                termion::cursor::Goto(cursor_x[0], 1),
                current_key.to_string()
            )
            .unwrap();
            mistakes += 1;
            retracking = false;
        } else {
            write!(stdout, 
                "{}{}", 
                termion::cursor::Goto(cursor_x[0], 1),
                current_key.to_string()
            )
            .unwrap();
            retracking = false;
        }

        match k.as_ref().unwrap() {
            Key::Esc => break,
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

        if !retracking {
            cursor_x[0] += 1;
            prompt_pos += 1;
        } else {
            cursor_x[0] -= 1;
            prompt_pos -= 1;
        }

        write!(stdout,
            "{}",
            termion::cursor::Goto(cursor_x[0], 1)
        )
        .unwrap();

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
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();
}


fn create_char_vec(prompt: &mut std::str::Chars) -> Vec<char> {
    let mut vec = Vec::new();
    for _ in 0..prompt.as_str().len() {
        vec.push(prompt.nth(0).unwrap());
    }
    
    vec
}
