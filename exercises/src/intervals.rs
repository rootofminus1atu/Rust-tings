
use std::fmt;

static DEFAULT_INTERVAL: Interval = Interval {
    lower: BoundaryValue { value: 0.0, closed: false },
    upper: BoundaryValue { value: 0.0, closed: false },
};

pub fn display() {
    let itv_res = Interval::from_str("(-1.0 ,  2.5]");

    match itv_res {
        Ok(itv) => println!("itv: {:?}", itv),
        Err(e) => println!("Error: {:?}", e),
    }

}


#[allow(dead_code)]
struct BoundaryValue {
    value: f64,
    closed: bool,
}



#[allow(dead_code)]
struct Interval {
    lower: BoundaryValue,
    upper: BoundaryValue,
}

impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lower_bracket = if self.lower.closed { '[' } else { '(' };
        let upper_bracket = if self.upper.closed { ']' } else { ')' };

        write!(f, "{}{}, {}{}", lower_bracket, self.lower.value, self.upper.value, upper_bracket)
    }
}

#[allow(dead_code)]
pub struct InvalidInterval {
    interval: String,
    message: String,
}

impl InvalidInterval {
    pub fn new(interval: &str, message: &str) -> Self {
        InvalidInterval {
            interval: interval.to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Debug for InvalidInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InvalidInterval Error: `{}` is not a valid interval. {}", self.interval, self.message)
    }
}
/* 
impl fmt::Debug for InvalidInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut interval_capture: &str;
        let mut msg_capture: &str;

        match self {
            InvalidInterval::InvalidFormat(interval, msg) => {
                interval_capture = interval;
                msg_capture = msg;
            },
            InvalidInterval::InvalidNumber(interval, msg) => {
                interval_capture = interval;
                msg_capture = msg;
            }
        }

        write!(f, "InvalidInterval Error: `{}` is not a valid interval. {}", interval_capture, msg_capture)
    }
}
*/
#[allow(dead_code)]
impl Interval {
    pub fn from_str(interval_str: &str) -> Result<Interval, InvalidInterval> {
        // 0. prepare data
        let interval_str = interval_str.trim();
        if interval_str.is_empty() {
            return Err(InvalidInterval::new(interval_str, "You must provide a non-empty string."))
        }

        // 1. Get the 1st and last chars, match for brackets
        let first_char = interval_str.chars().next().unwrap_or(' ');
        let last_char = interval_str.chars().last().unwrap_or(' ');

        const VALID_LEFT: [char; 2] = ['(', '['];
        const VALID_RIGHT: [char; 2] = [')', ']'];

        if !VALID_LEFT.contains(&first_char) || !VALID_RIGHT.contains(&last_char) {
            return Err(InvalidInterval::new(interval_str, "No valid matching brackets found."))
        }

        // 2. split remainings on comma
        let parts: Vec<&str> = interval_str[1..interval_str.len() - 1].split(',').collect();

        // 3. get 2 parts
        let (lower, upper) = match parts.len() {
            2 => (parts[0].trim(), parts[1].trim()),
            _ => return Err(InvalidInterval::new(interval_str, "No comma found."))
        };

        // 4. try extract the numebrs from what's left
        let lower_num = lower.parse::<f64>().map_err(|_| InvalidInterval::new(interval_str, &format!("The 1st number `{}` could not be parsed.", lower)))?;
        let upper_num = upper.parse::<f64>().map_err(|_| InvalidInterval::new(interval_str, &format!("The 2nd number `{}` could not be parsed.", upper)))?;


        // 5. final checks
        if lower_num > upper_num {
            return Err(InvalidInterval::new(interval_str, "The 1st number must be smaller than the 2nd number."))
        }


        // 6. create the interval and return it
        let itv = Interval {
            lower: BoundaryValue {
                value: lower_num,
                closed: first_char == '[',
            },
            upper: BoundaryValue {
                value: upper_num,
                closed: last_char == ']',
            },
        };

        Ok(itv)
    }


    fn contains(&self, x: f64) -> bool {
        let lower_ok = if self.lower.closed {
            x >= self.lower.value
        } else {
            x > self.lower.value
        };

        let upper_ok = if self.upper.closed {
            x <= self.upper.value
        } else {
            x < self.upper.value
        };

        lower_ok && upper_ok
    }
    
}