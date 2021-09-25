use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn animate_slow_output(
  output: &str,
  delay: u64,
) {
  for c in output.chars() {
    printflush(&c.to_string());
    sleep(Duration::from_millis(delay));
  }
}

pub fn imitate_process(duration_in_millis: u64) {
  sleep(
    Duration::from_millis(
      duration_in_millis
    )
  );
}

fn printflush(output: &str) {
  print!("{}", output);
  io::stdout().flush().unwrap();
}
