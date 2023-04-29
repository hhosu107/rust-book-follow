use std::io;
use std::collections::HashSet;

fn main() {
    // List of string(words).
    // For each word, change word[1..]-ay if 자음, word-hay if 모음
    let mut vowels = HashSet::new();
    for c in vec!['a', 'e', 'i', 'o', 'u'] {
        vowels.insert(c);
    }
    let mut vec: Vec<String> = Vec::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.split_whitespace();
    for i in input {
        vec.push(i.to_string());
    }
    for word in &mut vec {
        let first_char: char = word.chars().next().unwrap();
        if !vowels.contains(&first_char) {
            *word = word[1..].to_string();
            word.push(first_char);
            word.push_str("-ay");
        } else {
            word.push_str("-hay");
        }
    }
    for word in vec {
        print!("{word} ");
    }
    println!("");
}
