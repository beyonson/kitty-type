use std::io::{stdin, stdout, Write};
use std::time::SystemTime;
use termion::event::Key;
use termion::input::TermRead;
use termion::color;
use termion::raw::IntoRawMode;
use random_word::Lang;
use rand::Rng;

fn main() { // Stdio
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    // Iterators
    let mut cursor_x: u16 = 0;

    // Game data
    let mut buffer: String = "".to_owned();
    let prompt_string = generate_prompt();
    let mut prompt = prompt_string.chars();
    let prompt_len = prompt_string.len();
    let mut mistakes = 0;
    let mut retracking = false;

    print_prompt(&mut stdout, prompt.as_str());
    let now = SystemTime::now();
    let prompt_vec = create_char_vec(&mut prompt);

    for k in stdin.keys() {
        let current_key = prompt_vec[usize::from(cursor_x)];
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
                termion::cursor::Goto(cursor_x+1, 1),
                current_key.to_string()
            )
            .unwrap();
            mistakes += 1;
            retracking = false;
        } else {
            write!(stdout, 
                "{}{}{}", 
                termion::color::Bg(color::Reset), 
                termion::cursor::Goto(cursor_x+1, 1),
                current_key.to_string()
            )
            .unwrap();
            retracking = false;
        }

        match k.as_ref().unwrap() {
            Key::Ctrl(_c) => break,
            _ => {}
        }

        // Push input key to input buffer
        match k.unwrap() {
            Key::Char(k) => buffer.push(k),
            _ => {}
        }
        
        // End condition
        if usize::from(cursor_x) == prompt_len - 1 {
            match now.elapsed() {
                Ok(elapsed) => {
                    let elapsed_time = elapsed.as_secs();
                    complete_test(&mut stdout, elapsed_time as f32, mistakes as f32, prompt_len as f32);
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
            break;
        }

        if !retracking {
            cursor_x += 1;
        } else {
            cursor_x -= 1;
        }

        write!(stdout,
            "{}",
            termion::cursor::Goto(cursor_x+1, 1)
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
fn complete_test(stdout: &mut std::io::Stdout, elapsed_time: f32, mistakes: f32, prompt_len: f32) {
    let accuracy = 100.0*((prompt_len - mistakes)/prompt_len);
    let wpm = prompt_len/5.0*(60.0/elapsed_time)*accuracy/100.0;
    write!(
        stdout,
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        termion::cursor::Goto(1, 3),
        termion::color::Fg(color::Blue),
        termion::color::Bg(color::Reset),
        "Accuracy: ", 
        accuracy.to_string(),
        "%", 
        " WPM: ",
        wpm.to_string(),
        termion::cursor::Goto(1, 5),
        "      /^^\\",
        termion::cursor::Goto(1, 6),
        "    __\\`-`",
        termion::cursor::Goto(1, 7),
        "_--/  , )," ,
        termion::cursor::Goto(1, 8)
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


fn generate_prompt() -> String {
    let mut prompt: String = "".to_owned();
    
    for i in 0..10 {
        let rand_int = rand::thread_rng().gen_range(3..6);
        prompt.push_str(random_word::gen_len(rand_int, Lang::En).unwrap());
        if i != 9 {
        prompt.push_str(" ");
        }
    }
    
    prompt
}


fn create_char_vec(prompt: &mut std::str::Chars) -> Vec<char> {
    let mut vec = Vec::new();
    for _ in 0..prompt.as_str().len() {
        vec.push(prompt.nth(0).unwrap());
    }
    
    vec
}
