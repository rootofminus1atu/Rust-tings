use core::panic;

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

    // let res = exprs.iter().map(|s| Token::tokenize(s)).collect::<Vec<_>>();

    for thing in exprs {
        println!("{} -> {:?}", thing, Token::tokenize(thing));
    }


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
        println!("{} -> {:?}", s, Expression::from_tokens(&Token::tokenize(s)));
    }
}


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
    Int(i32),
    Var(String),
    Operation(Box<Operation>)
}

/// An intermediate representation containig both tokens and expressions that have already been parsed.
///
/// Because the order of operations and other priorities exist. At least I think this will make managing that easier.
#[derive(Debug)] 
enum Tokexpr {
    Token(Token),
    Expression(Expression)
}

impl Tokexpr {
    pub fn from_tokens(tokens: &[Token]) -> Vec<Tokexpr> {
        tokens.into_iter().map(|t| Tokexpr::Token(t.clone())).collect::<Vec<_>>()
    } 
}



#[derive(Clone)]
struct Inner;

enum Wrapper {
    Left(Inner),
    Right
}

impl Wrapper {
    pub fn wraps_collection(inners: &mut [Inner]) -> &mut [Wrapper] {
        &mut inners.iter().map(|i| Wrapper::Left(i.clone())).collect::<Vec<_>>()
    }

    // is there a way to avoid having to clone and return an owned type? is it possible to have it be more referential
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