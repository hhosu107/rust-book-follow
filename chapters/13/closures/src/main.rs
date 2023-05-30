use std::thread;

struct T {
    val: i32
}

impl T{
    pub fn applier<F>(self, f: F) -> T
    where
        F: Fn(T) -> T
    {
        f(self)
    }
}

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let a = 4;
    let x = T{val: 5};
    let add_a = |x: T| T{val: x.val + a};
    println!("{}", x.applier(add_a).val)
}
