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
    "add" => add(&input, state),
    "exit" => exit(state),
    "print" => ppp(state),
    _ => default(state),
  }
}

fn exit(
  state: &mut SystemState,
) {
  imitate_process(500);
  state.exit = true;
}

fn default(
  state: &mut SystemState,
) {
  imitate_process(500);
  animate_slow_output(
    "Unknown command...\n", 40);
  imitate_process(500);
  animate_slow_output("Possible commands:\n", 40);
  animate_slow_output("  add\n", 40);
  animate_slow_output("  print\n", 40);
  animate_slow_output("  exit\n", 40);
}

fn add(
  input: &Vec<&str>,
  state: &mut SystemState,
) {
  if input.len() >= 4 {
    let mut position = "start".to_string();
    let mut person_name = "".to_string();
    let mut department_name = "".to_string();
    for token in input {
      match (
        token.to_owned().trim(),
        position.as_str(),
      ) {
        ("add", "start") => {
          imitate_process(500);
          animate_slow_output(
            "Readint person name... ", 40);
          position = "person_name".to_string();
        },
        ("to", "person_name") => {
          imitate_process(500);
          animate_slow_output("COMPLETED\n", 40);
          imitate_process(500);
          animate_slow_output(
            "Readint department name... ", 40);
          position =
            "department_name".to_string();
        },
        (_, "person_name") => {
          person_name += &(
            token.to_owned().trim().to_owned() +
            " "
          );
        },
        (_, "department_name") => {
          department_name += &(
            token.to_owned().trim().to_owned() +
            " "
          );
        },
        _ => {
          imitate_process(500);
          animate_slow_output(
            "Unexpected input... ", 40);
          imitate_process(500);
          animate_slow_output("IGNORING\n", 40);
        },
      }
    }

    person_name =
      person_name.trim().to_string();
    department_name =
      department_name.trim().to_string();

    if (
      person_name.len() == 0 ||
      department_name.len() == 0
    ) {
      imitate_process(500);
      animate_slow_output(
        "\nIncorrect usage...\n", 40);
      imitate_process(500);
      animate_slow_output(
        &(
          "Correct usage example:\n".to_string() +
          "  add PERSON_NAME to DEPARTMENT_NAME\n"
        ),
        40);

      return;
    }

    imitate_process(500);
    animate_slow_output("COMPLETED\n", 40);

    imitate_process(500);
    animate_slow_output(
      &("Adding ".to_owned() +
        &person_name +
        " to " +
        &department_name +
        "... "
      ),
      40);

    let current_department =
      state
        .departments
        .entry(department_name)
        .or_insert(Vec::new());
    current_department.push(person_name);

    imitate_process(500);
    animate_slow_output("COMPLETED\n", 40);
  } else {
    imitate_process(500);
    animate_slow_output(
      "Incorrect usage...\n", 40);
    imitate_process(500);
    animate_slow_output(
      &(
        "Correct usage example:\n".to_string() +
        "  add PERSON_NAME to DEPARTMENT_NAME\n"
      ),
      40);
  }
}

fn ppp(
  state: &mut SystemState,
) {
  imitate_process(500);
  animate_slow_output(
    "Printing added data... \n\n", 40);

  for (
    department_name,
    people_names,
  ) in &state.departments {
    animate_slow_output(&department_name, 30);
    animate_slow_output(":\n", 30);
    for person_name in people_names {
      animate_slow_output("  ", 30);
      animate_slow_output(&person_name, 30);
      animate_slow_output("\n", 30);
    }
  }

  imitate_process(500);
  animate_slow_output("\nCOMPLETED\n", 40);
}
