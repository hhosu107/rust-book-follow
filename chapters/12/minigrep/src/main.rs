use std::env;

use minigrep::Config;

struct Bar {
    x: i32,
    y: i32,
}

impl Bar {
    fn new(x: i32, y: i32) -> Bar {
        Bar {x, y}
    }
}

struct X {
    y: i32
}

struct B {
    x: X
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // For now, ignore error handling.
    // NOTE: https://github.com/rust-lang/rust/pull/71156 에 별 해괴한 destructuring method들이
    // 들어있다.
    let mut bb: B = B{x: X{y: 0}};
    let a: i32;
    let c: i32;
    (a, (bb.x.y, c)) = (0, (1, 2));
    assert_eq!((c, (a, bb.x.y)), (2, (0, 1)));
    let Bar {x: a, y: b} = Bar::new(1, 2);
    let (x, y, .., z) = (1.0, 2.0, 3.0, 4.0, 5.0);
    assert_eq!((z, y, x), (5.0, 2.0, 1.0));
    println!("{a}, {b}");
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
