use std::collections::HashMap;
use std::collections::BTreeMap;
use std::io;

type CompanyMap = HashMap<String, BTreeMap<String, i32>>;

fn print_usage() {
    println!("
    Inputs are three types.
    1. `Add <name> to <Department>`: saves a new member names as name to Department.
    This allows a duplication on the name, but the same name on department is considered as the
    same department.
    2. `List <department_name>`: this one prints all members in the department ordered.
    3. `List all`: this one prints all members across all department. Thus you have to judge
       whether or not department name is not 'all' when you get the first type of command.
    4. `Exit`: quit this program.");
}

fn main() {
    // Inputs are three types.
    // 1. `Add <name> to <Department>`: saves a new member names as name to Department.
    // This allows a duplication on the name, but the same name on department is considered as the
    // same department.
    // 2. `list <department_name>`: this one prints all members in the department ordered.
    // 3. `list all`: this one prints all members across all department. Thus you have to judge
    //    whether or not department name is not 'all' when you get the first type of command.
    // 4. `exit`: quit this program.
    let mut company_map : CompanyMap = CompanyMap::new();
    let mut company_workers : BTreeMap<String, i32> = BTreeMap::new();

    print_usage();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        match input.next() {
            Some("Add") => {
                let name = input.next().unwrap().to_string();
                let _ = input.next().unwrap().to_string();
                let dept = input.next().unwrap().to_string();
                if dept == "all" {
                    println!("You entered invalid department name. Use other.");
                    continue;
                }
                let _ = company_workers.entry(name.clone()).and_modify(|e| { *e += 1 }).or_insert(1);
                let dept_workers = company_map.entry(dept.clone()).or_insert(BTreeMap::new());
                let _ = dept_workers.entry(name.clone()).and_modify(|e| { *e += 1 }).or_insert(1);
            }
            Some("List") => {
                let name = input.next().unwrap().to_string();
                if name == "all" {
                    print!("Company: ");
                    for (name, count) in &company_workers {
                        for _ in 0..*count {
                            print!("{name} ");
                        }
                    }
                    print!("\n");
                } else {
                    let dept_workers = company_map.entry(name.clone()).or_insert(BTreeMap::new());
                    print!("Dept {name}: ");
                    for (names, count) in dept_workers {
                        for _ in 0..*count {
                            print!("{names} ");
                        }
                    }
                    print!("\n");
                }
            }
            Some("Exit") => {
                println!("End");
                break;
            }
            _ => {
                print_usage();
            }
        }
    }
}
