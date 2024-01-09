pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2))
    }
}
