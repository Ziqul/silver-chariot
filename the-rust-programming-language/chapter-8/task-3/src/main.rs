use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut state = initialize();
    main_loop(&mut state);
    terminate();
}

fn animate_slow_output(
    output: &str,
    delay: u64,
) {
    for c in output.chars() {
        printflush(&c.to_string());
        sleep(Duration::from_millis(delay));
    }
}

fn printflush(output: &str) {
    print!("{}", output);
    io::stdout().flush().unwrap();
}

fn initialize() -> SystemState {
    animate_slow_output("Starting... ", 40);
    let mut state = SystemState {
        exit: false,
    };
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
                .unwrap()),
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

fn imitate_process(duration_in_millis: u64) {
    sleep(
        Duration::from_millis(
            duration_in_millis
        )
    );
}

struct SystemState {
    exit: bool,
}
