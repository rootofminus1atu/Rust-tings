use core::panic;

// todos:
// testing done well not this bs
// use BetterOperation
// remove Box from Expression


fn main() {
    // what's supported currently:
    // - numbers
    // - variables
    // - addition and subtraction
    // 
    // no other operations and no parentheses (will add them on later)
    let tests1 = vec![
        "1 + 2",
        "2 + x",
        "y + x",
        "a + 5",
        "1 + 2 - 3",
        "1 - 2 + hello + 4",
        "1 - 2 + 3"
    ];

    for s in tests1 {
        println!("{} -> {:?}", s, Token::tokenize(s));
    }
}

// should I use something denormalized like this (no Val, Int and Var directly here)
#[derive(Debug, Clone, PartialEq)]
enum Token2 {
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

// or something more normalized like this (Val that can be Int or Var)
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Val(Val),
    Add,
    Subt,
    Mult,
    Div,
    Pow,
    LPar,
    RPar,
}

#[derive(Debug, Clone, PartialEq)]
enum Val {
    Int(i32),
    Var(String),
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

                    tokens.push(Token::Val(Val::Int(num_candidate.parse().unwrap())));
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

                    tokens.push(Token::Val(Val::Var(var)));
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
    Val(Val),
    Int(i32),  // BETTER NAMES AND REARRANGE THIS
    Var(String),
    BinaryOp {
        kind: OperationKind,
        left: Box<Expression>,
        right: Box<Expression>
    }
}

impl Expression {
    pub fn from_str(expr: &str) -> Self {
        let tokens = Token::tokenize(expr);

        // first pars
        // take out the LPar, Inside, RPar (once the correct RPar is found, with the counting method)
        // parse the inside instead, which eventually won't have parentheses anymore, can proceed to the next step

        // then ^ with the special right-to-left rule for chains of ^

        // then * /

        // then + -
        // doing this first actually in `from_tokens` down below
        

        Self::Int(0)
    }

    fn from_val(val: &Val) -> Self {
        match val {
            Val::Int(n) => Expression::Int(*n),
            Val::Var(x) => Expression::Var(x.clone())
        }
    }

    fn from_tokens(tokens: &[Token]) -> Self {
        let res = match tokens {
            [_val @ Token::Val(val)] => {
                match val {
                    Val::Int(n) => Expression::Int(*n),
                    Val::Var(x) => Expression::Var(x.clone())  // is this bad
                }
            },
            [rest @ .., Token::Add, _last @ Token::Val(x)] => {
                Expression::Operation(Box::new(
                    Operation::Add {
                        left: Self::from_tokens(rest),
                        right: Self::from_val(x),
                    }
                ))
            },
            [rest @ .., Token::Subt, _last @ Token::Val(x)] => {
                Expression::Operation(Box::new(
                    Operation::Subt {
                        left: Self::from_tokens(rest), 
                        right: Self::from_val(x),
                    }
                ))
            },                
            _ => { panic!("noooooooo") } 
        };
        
        res
    }
}

#[derive(Debug)]
enum OperationKind {
    Add,
    Subt,
    Mult,
    Div,
    Pow
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