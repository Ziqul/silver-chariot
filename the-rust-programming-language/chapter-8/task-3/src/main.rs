mod structs;
mod help;

use std::io;
use std::io::prelude::*;
use structs::SystemState;
use help::{
  animate_slow_output,
  imitate_process,
};

fn main() {
  let mut state = initialize();
  main_loop(&mut state);
  terminate();
}

fn initialize() -> SystemState {
  animate_slow_output("Starting... ", 40);
  let state = SystemState::build();
  imitate_process(500);
  animate_slow_output("COMPLETED\n", 20);
  return state;
}

fn terminate() {
  animate_slow_output("Shutting down... ", 40);
  imitate_process(500);
  animate_slow_output("COMPLETED\n", 20);
}

fn main_loop(state: &mut SystemState) {
  let stdin = io::stdin();
  let mut stdin_iterator = stdin.lock().lines();

  animate_slow_output(
    "Waiting for input...\n", 40);

  loop {
    animate_slow_output("> ", 40);

    process_input(
      &(stdin_iterator
        .next()
        .unwrap()
        .unwrap()
      ),
      state,
    );

    if state.exit {
      break;
    }
  }

}

fn process_input(
  input: &str,
  state: &mut SystemState,
) {
  let input: Vec<&str> =
    input.trim().split(' ').collect();

  match input[0] {
    "exit" => {
      imitate_process(500);
      state.exit = true;
    },
    _ => {
      imitate_process(500);
      animate_slow_output(
        "Unknown command... ", 40);
      imitate_process(500);
      animate_slow_output("IGNORING\n", 40);
    },
  }
}
