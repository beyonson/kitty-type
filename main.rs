use std::io;

fn main() -> io::Result<()>
{
  let test_text = "fish should fine by duck mode\n";

  println!("Start typing when you're ready");
  println!("Press enter to finish");

  println!("\n{}", test_text);

  let mut test_input = String::new(); 
  io::stdin().read_line(&mut test_input)?;

  println!("Test complete!");
  println!("WPM: N/A");

  Ok(())
}
