use std::io;

fn main() {
    println!("\
        [INFO] Starting nth Fibonacci number \
        calculator...");

    println!("\
        [INFO] Waiting for input...");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("[ERROR] Failed to read input...");

    let n: i32 = n.trim().parse().expect("");

    let Phi = (5_f64.sqrt() + 1.0) / 2.0;
    let phi = 1.0 / Phi;

    let fibonacci_number: u32 =
        ((
            Phi.powf(n.into()) -
            (phi * -1.0).powf(n.into())
        ) / 5_f64.sqrt()) as u32;

    println!(
        "[INFO] {}th Fibonacci number is {}",
        n, fibonacci_number);
}
