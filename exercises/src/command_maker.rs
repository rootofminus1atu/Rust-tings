pub fn display() {
    let com1 = Command {
        name: "Say Hello".to_string(),
        description: "Prints 'Hello' to the console".to_string(),
        action: hi_thing
    };

    let com2 = Command {
        name: "Square a number".to_string(),
        description: "Squares a number and prints the result to the console".to_string(),
        action: || square_thing(4)
    };
}

fn hi_thing() {
    println!("Hi");
}


fn square_thing(x: i32) {
    println!("{}", x * x);
}

struct CommandBundle {
    options: Vec<Command>
}

struct Command {
    name: String,
    description: String,
    action: fn()
}

impl CommandBundle {
    fn new() -> Self {
        CommandBundle { options: Vec::new() }
    }

    fn add(&mut self, name: &str, description: &str, action: fn()) {
        self.options.push(Command { name: name.to_string(), description: description.to_string(), action });
    }

    fn display(&self) {
        for command in &self.options {
            println!("{} - {}", command.name, command.description);
        }
    }

    fn execute(&self, name: &str) {
        for command in &self.options {
            if command.name == name {
                (command.action)();
                return;
            }
        }
        println!("Invalid command");
    }
}