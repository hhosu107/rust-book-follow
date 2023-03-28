use std::io;

fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut k = String::new();
    io::stdin()
        .read_line(&mut k)
        .expect("Failed to read line");
    let x: u32 = k.trim().parse().expect("Please type a number!");
    for _ in 0..x {
        let c = a + b;
        a = b;
        b = c;
    }
    println!("{a}");
}
