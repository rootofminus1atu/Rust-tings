fn main() {
    let expr = Expression::Operation(Box::new(
        Operation::Mult {
            left: Expression::Int(2),
            right: Expression::Operation(Box::new(
                Operation::Pow { 
                    base: Expression::Var("x".into()), 
                    power: Expression::Int(5) 
                }
            ))
        }
    ));

    let exprs = vec![
        // parentheses
        "(((x)^(2)) - (2)) / ((5) * (((10) - (x)) * (x)^(3)))",
        "(x^2 - 2) / (5 * (10 - x) * x^3)",

        // more complicated parentheses
        "((x - 1) * (x + 1)) / (5 * (10 - (x + 3)(x - 2)) * (2 + y))",
        "1 / (5 * (10 - (x + 3)(x - 2)) * (2 + y))",
        "((x - 1) * (x + 1)) / 1",

        // limited parantheses
        "2 / x",
        "x / 2",
        "(2 + y) / x^2",
        "y^2 / (x - 4)",
        "x / 2 / 3",
        "1 / (2 / 3)",
        "(1 / 2) / 3",
        "(1 / ( 2 / ( 3 / 4 )))",

        // empty
        "",

        // order of operations
        "1 + 2 * 3",
        "1 * 2 + 3",
        "1 + 2 + 3",
        "1 + 2 - 3",
        "1 - 2 + 3",
        "1 - 2 - 3"
    ];

    let res = exprs.iter().map(|s| Expression::from_str(s)).collect::<Vec<_>>();

    dbg!(res);
}


#[derive(Debug, Clone, PartialEq)]
enum Token {
    Int(i32),
    Var(String),
    Add,
    Subt,
    Mult,
    Div,
    Pow,
    LPar,
    RPar,
}

impl Token {
    pub fn tokenize(expr: &str) -> Vec<Self> {
        let mut tokens = Vec::new();
        let mut chars = expr.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    let mut num_candidate = String::new();

                    while let Some(&d) = chars.peek() {
                        if d.is_digit(10) {
                            num_candidate.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::Int(num_candidate.parse().unwrap()));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut var = String::new();

                    while let Some(&d) = chars.peek() {
                        // hopefully that covers variable names and hopefully +-*/ arent allowed
                        // should test that
                        if d.is_ascii_alphanumeric() || d == '_' {
                            var.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::Var(var));
                }
                '+' => {
                    tokens.push(Token::Add);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Subt);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Mult);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Div);
                    chars.next();
                }
                '^' => {
                    tokens.push(Token::Pow);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LPar);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RPar);
                    chars.next();
                }
                _ => {
                    chars.next();
                }
            }
        }

        tokens
    }
}


#[derive(Debug)]
enum Expression {
    Int(i32),
    Var(String),
    Operation(Box<Operation>)
}

impl Expression {
    pub fn from_str(expr: &str) -> Self {

        // do a parentheses scan, try to find `() op ()`
        // make sure that it's the outhermost ()op()
        // once found check the binary op and go from there


        Self::Int(0)
    }
}


// whats a better way? enum of trait? im representing simple binary operatiors that just have expressions on the left/right, should i use enum or trait or both?

#[derive(Debug)]
enum Operation {
    Add { left: Expression, right: Expression },
    Subt { left: Expression, right: Expression },
    Mult { left: Expression, right: Expression },
    Div { left: Expression, right: Expression },
    Pow { base: Expression, power: Expression }
}


struct Pow {
    base: Expression,
    power: Expression
}

struct Div {
    left: Expression,
    right: Expression
}

struct Mult {
    left: Expression,
    right: Expression
}

struct Add {
    left: Expression,
    right: Expression
}

struct Subt {
    left: Expression,
    right: Expression
}