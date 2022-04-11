/*
    COMMANDS:
        - list [employees, departments]
        - list employees [in/from] [department]
        - add [name] to [department]
        - remove [name] from [department]
*/

use colored::Colorize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    employees.insert(
        "Sales".to_string(),
        vec!["Jim".to_string(), "Andy".to_string()],
    );
    employees.insert(
        "IT".to_string(),
        vec!["Sharon".to_string(), "Steve".to_string()],
    );

    loop {
        println!("{}", format!("Command:").underline());

        // read in a command from the user
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect(&format!("Failed to read input").italic().yellow());

        // split on spaces and save each word as an entry in a vector
        let command_vec: Vec<String> = command.trim().split(" ").map(str::to_string).collect();

        // run the appropriate function based on each value of the vector
        if command_vec.len() == 4 {
            match command_vec[0].as_str() {
                "add" => match command_vec[2].as_str() {
                    "to" => add_employee(
                        &mut employees,
                        command_vec[1].as_str(),
                        command_vec[3].as_str(),
                    ),
                    _ => invalid_command(&command_vec[2]),
                },
                "remove" => match command_vec[2].as_str() {
                    "from" => remove_employee(
                        &mut employees,
                        command_vec[1].as_str(),
                        command_vec[3].as_str(),
                    ),
                    _ => invalid_command(&command_vec[2]),
                },
                "list" => match command_vec[1].as_str() {
                    "employees" => match command_vec[2].as_str() {
                        "from" | "in" => list_employees(&employees, Some(command_vec[3].as_str())),
                        _ => invalid_command(&command_vec[2]),
                    },
                    _ => invalid_command(&command_vec[1]),
                },
                _ => invalid_command(&command_vec[0]),
            }
        } else if command_vec.len() == 2 {
            match command_vec[0].as_str() {
                "list" => match command_vec[1].as_str() {
                    "employees" | "all" => list_employees(&employees, None),
                    "departments" => list_departments(&employees),
                    _ => invalid_command(&command_vec[1]),
                },
                _ => invalid_command(&command_vec[0]),
            }
        } else if command_vec[0] == "exit" {
            std::process::exit(0);
        } else {
            println!(
                "{}",
                format!("Invalid number of commands").italic().yellow()
            );
        }

        println!("");
    }
}

fn add_employee(employees: &mut HashMap<String, Vec<String>>, name: &str, department: &str) {
    // check if the department already has employees, if so then push to the vector
    // else create a new vector in that key

    match employees.entry(department.to_string()) {
        Entry::Vacant(e) => {
            e.insert(vec![name.to_string()]);
        }
        Entry::Occupied(mut e) => {
            e.get_mut().push(name.to_string());
        }
    }

    println!(
        "{}",
        format!("Added {} to {}", name, department)
            .italic()
            .yellow()
    );
}

fn remove_employee(employees: &mut HashMap<String, Vec<String>>, name: &str, department: &str) {
    // check if department is a valid key
    if !employees.contains_key(department) {
        println!(
            "{}",
            format!("Department {} not found", department)
                .italic()
                .yellow()
        );
        return;
    }

    // get index of employee to remove
    let index;
    match employees[department].iter().position(|v| v == name) {
        Some(i) => index = i,
        None => {
            println!(
                "{}",
                format!("Could not find {} in {}", name, department)
                    .italic()
                    .yellow()
            );
            return;
        }
    };

    // remove employee
    match employees.entry(department.to_string()) {
        Entry::Vacant(_) => {
            println!(
                "{}",
                format!("Department {} is empty", department)
                    .italic()
                    .yellow()
            );
            return;
        }
        Entry::Occupied(mut e) => {
            e.get_mut().remove(index);
        }
    }

    println!(
        "{}",
        format!("Removed {} from {}", name, department)
            .italic()
            .yellow()
    );
}

fn list_employees(employees: &HashMap<String, Vec<String>>, department: Option<&str>) {
    // list employees of a department if one is specified, else list all

    match department {
        Some(dept) => {
            if !employees.contains_key(dept) {
                return;
            }
            println!("{}", format!("{:?}", employees[dept]).bold().green());
        }
        None => println!(
            "{}",
            format!("{:?}", employees.values().flatten().collect::<Vec<_>>())
                .bold()
                .green()
        ),
    }
}

fn list_departments(employees: &HashMap<String, Vec<String>>) {
    // list all departments

    if employees.keys().len() > 0 {
        println!("{}", format!("{:?}", employees.keys()).bold().green());
    } else {
        println!("{}", format!("No departments found").italic().yellow());
    }
}

fn invalid_command(command: &str) {
    println!(
        "{}",
        format!("Invalid command: {}", command).italic().yellow()
    );
}
