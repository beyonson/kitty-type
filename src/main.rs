use slint::Timer;
use std::io;

fn my_callback(prompt: &str, input: &mut String) {
  if prompt != input {
    println!("You failed");
    println!("You typed {}", input);
  } 
}

fn main()
{
  let test_text = "fish should fine by duck mode\n";

  println!("Start typing when you're ready");

  println!("\n{}", test_text);

  let mut test_input = String::new(); 

  Timer::single_shot(std::time::Duration::from_secs(5), move || {
    println!("Test complete.");
    slint::quit_event_loop();
  });

  io::stdin().read_line(&mut test_input);

  //println!("WPM: N/A");
  slint::run_event_loop_until_quit();
  if &mut test_input != test_text {
    println!("You failed the test.");
  }
}
