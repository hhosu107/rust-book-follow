use std::io; //표준입출력

fn main() {

    let mut postfix = String::new();
    io::stdin()
        .read_line(&mut postfix)
        .expect("Failed to read line");

    let result: i32 = cal(&postfix.trim()); // TODO

    println!("input: {}", &postfix.trim());
    println!("output: {result}");
}

fn cal(s: &str) -> i32 {
    let mut nums: [i32; 20] = [0; 20];
    let mut top = 0;

    let bytes = s.as_bytes();

    let mut current_value: i32 = 0;
    let mut is_digit = false;
    for (i, &item) in bytes.iter().enumerate() {
        println!("{i}");
        if item == b' ' {
            if is_digit {
                nums[top] = current_value;
                top += 1;
                println!("current_value: {current_value}, top: {top}");
                current_value = 0;
                is_digit = false;
            }
        } else if item == b'+' {
            let c = add(nums[top - 1], nums[top - 2]);
            top -= 2;
            nums[top] = c;
            top += 1;
            println!("top: {top}, top_val: {}", nums[top - 1]);
        } else if item == b'-' {
            let c = sub(nums[top - 2], nums[top - 1]);
            top -= 2;
            nums[top] = c;
            top += 1;
            println!("top: {top}, top_val: {}", nums[top - 1]);
        } else if item == b'*' {
            let c = mul(nums[top - 1], nums[top - 2]);
            top -= 2;
            nums[top] = c;
            top += 1;
            println!("top: {top}, top_val: {}", nums[top - 1]);
        } else if item == b'/' {
            let c = div(nums[top - 2], nums[top - 1]);
            top -= 2;
            nums[top] = c;
            top += 1;
            println!("top: {top}, top_val: {}", nums[top - 1]);
        } else { // digits
            is_digit = true;
            current_value = 10 * current_value + (item - b'0') as i32;
        }
    }
    return nums[0];
}

fn add(x1: i32, x2: i32) -> i32 {
    x1 + x2
}

fn sub(x1: i32, x2: i32) -> i32 {
    x1 - x2
}

fn mul(x1: i32, x2: i32) -> i32 {
    x1 * x2
}

fn div(x1: i32, x2: i32) -> i32 {
    x1 / x2
}
