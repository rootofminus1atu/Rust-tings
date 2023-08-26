use std::collections::HashMap;
use std::io;
use std::io::BufRead;

pub fn display() {
    println!("Departments App");
    let mut db = MockDb::new();

    loop {
        show_menu();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        let command = match Command::from_str(&input)  {
            Some(c) => c,
            None => {
                println!("Invalid command");
                continue;
            }
        };

        command.execute(&mut db);
    }
}

fn show_menu() {
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");
}

enum Command {
    Add { name: String, dept: String},
    List(String),
    All, 
    Quit
}

impl Command {
    fn from_str(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        
        match words.as_slice() {
            ["Add", name, "to", dept] => Some(Command::Add { name: name.to_string(), dept: dept.to_string() }),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            _ => None
        }
    }

    fn execute(&self, db: &mut MockDb) {
        match self {
            Command::Add { name, dept } => {
                db.add_employee(dept, name);
                println!("Added {} to {}", name, dept);
            },
            Command::List(dept) => {
                match db.list_employees_for_dept(dept) {
                    Some(employees) => {
                        println!("Employees in {}:", dept);
                        for employee in employees {
                            println!("\t{}", employee);
                        }
                    },
                    None => println!("No dept named in {}", dept)
                }
            },
            Command::All => {
                for (dept, employees) in db.list_all_depts_and_employees() {
                    println!("{}:", dept);
                    for employee in employees {
                        println!("\t{}", employee);
                    }
                }
            },
            Command::Quit => {
                println!("Quitting");
                std::process::exit(0);
            }
        }
    }
}


struct MockDb {
    data: HashMap<String, Vec<String>>,
}

impl MockDb {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add_employee(&mut self, dept: &str, name: &str) {
        self.data.entry(dept.to_string()).or_insert(vec![]).push(name.to_string());
    }

    fn list_employees_for_dept(&self, dept: &str) -> Option<&Vec<String>> {
        self.data.get(dept)
    }

    fn list_all_depts_and_employees(&self) -> &HashMap<String, Vec<String>> {
        &self.data
    }
}
