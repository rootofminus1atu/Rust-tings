pub fn display() {
    let a = 10;
    let b = 0;
    let res = divide(a, b);

    match res {
        Ok(result) => println!("{} / {} = {}", a, b, result),
        Err(e) => println!("Error: {:?}", e),
    }

    let a = -10.0;
    let res = sqrt(a);

    match res {
        Ok(result) => println!("sqrt({}) = {}", a, result),
        Err(e) => println!("Error: {:?}", e),
    }
}

enum ArithmeticError {
    DivideByZero(f64, f64),
    NegativeSquareRoot(f64),
}

impl std::fmt::Debug for ArithmeticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = "ArithmeticError:";

        match self {
            ArithmeticError::DivideByZero(a, b) => {
                write!(f, "{} You cannot divide {} by {}, because division by 0 is not possible.", base, a, b)
            }
            ArithmeticError::NegativeSquareRoot(a) => {
                write!(f, "{} You cannot take the square root of {}, because square roots of negative numbers are not allowed in the real numbers.", base, a)
            }
        }
    }
}

fn divide(a: i32, b: i32) -> Result<i32, ArithmeticError> {
    if b == 0 {
        return Err(ArithmeticError::DivideByZero(a as f64, b as f64));
    }

    Ok(a / b)
}

fn sqrt(a: f64) -> Result<f64, ArithmeticError> {
    if a < 0.0 {
        return Err(ArithmeticError::NegativeSquareRoot(a));
    }

    Ok(a.sqrt())
}