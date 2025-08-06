use regex::Regex;
use std::collections::HashMap;

struct EmployeeSystem {
    employess_by_department: HashMap<String, Vec<String>>
}

impl EmployeeSystem {
    fn show_all(&self) {
        for (department, employees_list) in &self.employess_by_department {
            println!("Here are the employees of department {department}:");
            for employee in employees_list {
                println!("-{employee}");
            }
        }
    }

    fn show_department(&self, department_name: String) {
        match self.employess_by_department.get(&department_name) {
            Some(employees_list) => {
                let mut sorted_employees = employees_list.clone();
                sorted_employees.sort();

                println!("Here are the employees of department {department_name}:");
                for employee in sorted_employees {
                    println!("-{employee}");
                }
            },
            None => { println!("Have no employees on {department_name}.") }
        }
    }

    fn add_employee(&mut self, input: String) {
        if let Some(args) = parse_add_employee_input(&input) {
            
            let employees_list = self.employess_by_department
                .entry(args.department_name.clone())
                .or_insert(Vec::new());
            
            if employees_list.contains(&args.employee_name) {
                print!("{} is already on department!!", args.employee_name);
                return;
            }

            employees_list.push(args.employee_name);
            
        } else {
            println!("Invalid format, please try to use: Add <employee_name> to <department_name> !!");
        }
    }
}

struct AddEmployeeArgs {
    employee_name: String,
    department_name: String,
}

enum Command {
    ShowAll,
    ShowDepartment,
    AddEmployeeCommand,
    Break,
}

pub fn cli_interface() {
    
    let mut system = EmployeeSystem {
        employess_by_department: HashMap::new()
    };

    println!("Welcome to employees management system.");
    println!("Use expression 'Add <employee> to <departament>' to add a employee to a department.");
    println!("Use /show_all to show all people in the company by department.");
    println!("Use /<department> to show all people in a department.");

    loop {

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match parse_command(&input) {
            Some(Command::ShowAll) => {
                system.show_all();
            },
            Some(Command::ShowDepartment) => {
                let department_name: String = input.trim().chars().skip(1).collect();
                system.show_department(department_name);
            },
            Some(Command::AddEmployeeCommand) => {
                system.add_employee(input);
            },
            Some(Command::Break) => {
                break;
            }

            None => {
                println!("Invalid Command");
            }
        }

    }
}

fn parse_command(input: &str) -> Option<Command> {
    let input = input.trim();

    if input == "/show_all" {
        return Some(Command::ShowAll);
    }
    //
    if input == "exit" {
        return Some(Command::Break);
    }

    if input.starts_with("/") && input.len() > 1 {
        return Some(Command::ShowDepartment);
    }

    let add_employee_regex = Regex::new(r"(?i)add\s+(.+?)\s+to\s+(.+)").unwrap();
    if add_employee_regex.is_match(input) {
        return Some(Command::AddEmployeeCommand);
    }

    None
}

fn parse_add_employee_input(input: &str) -> Option<AddEmployeeArgs> {
    let add_employee_regex = Regex::new(r"(?i)add\s+(.+?)\s+to\s+(.+)").unwrap();
    
    if let Some(captures) = add_employee_regex.captures(input) {
        let employee_name = captures.get(1)?.as_str().trim().to_string();
        let department_name = captures.get(2)?.as_str().trim().to_string();
        
        if !employee_name.is_empty() && !department_name.is_empty() {
            Some(AddEmployeeArgs {
                employee_name: employee_name.trim().to_string(),
                department_name: department_name.trim().to_string()
            })
        } else {
            None
        }
    } else {
        None
    }
}