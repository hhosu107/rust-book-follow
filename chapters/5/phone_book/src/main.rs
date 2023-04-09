use std::collections::HashMap;
use std::io;

struct MemberInfo {
    name: String,
    student_number: String,
    phone: String,
}

impl MemberInfo {
    fn new(name: String, student_number: String, phone: String) -> MemberInfo {
        MemberInfo {
            name,
            student_number,
            phone,
        }
    }
}

type MemberMap = HashMap<String, MemberInfo>;

fn main() {
    // Inputs are consisted of 3 types of inputs.
    // 1. `1 name student_number phone`: saves the member information.
    // If there is a name conflict, the new information overwrites the old one.
    // 2. `2 name`: prints the member information.
    // If there is no name, print "No member" instead.
    // 3. `0`: terminates the program.
    // 4. Others at the first index: print "Invalid input" and terminate the program.
    let mut member_map = MemberMap::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        match input.next() {
            Some("1") => {
                let name = input.next().unwrap().to_string();
                let student_number = input.next().unwrap().to_string();
                let phone = input.next().unwrap().to_string();
                member_map.insert(name.clone(), MemberInfo::new(name, student_number, phone));
            }
            Some("2") => {
                let name = input.next().unwrap();
                match member_map.get(name) {
                    Some(member) => {
                        println!("{} {} {}", member.name, member.student_number, member.phone);
                    }
                    None => {
                        println!("No member");
                    }
                }
            }
            Some("0") => {
                println!("Program terminated");
                break;
            }
            _ => {
                println!("Invalid input");
                break;
            }
        }
    }
}
