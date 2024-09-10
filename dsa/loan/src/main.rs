use lazy_static::lazy_static;
use std::sync::Mutex;


struct Loan {
    principal: f64,
}

impl Loan {
    const ANNUAL_INTEREST_RATE: i32 = 10;  // wont really work as a const

    fn new(principal: f64) -> Self {
        Self { principal }
    }

    fn calculate_monthly_payment(&self, number_of_payments: i32) -> f64 {
        let percentage = Self::ANNUAL_INTEREST_RATE as f64 / 100.;
        let monthly_interest = percentage / 12.;

        self.principal * (monthly_interest / (1. - (1. + monthly_interest).powf(-number_of_payments as f64)))
    }

    fn set_annual_interest_rate(new_rate: i32) {
        // nah
        // bank class to hold the annual interest rate an an arr of loans
    }
}

fn main() {
    let l1 = Loan::new(5000.);
    let l2 = Loan::new(31000.);
}
