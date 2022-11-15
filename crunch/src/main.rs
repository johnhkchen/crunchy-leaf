fn foo() -> usize{
    return 42;
}

fn main() {
    println!("Hello, world!");
    println!("{}", foo());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_knows() {
        use crate::foo;
        let result = foo();
        assert_eq!(result, 42);
    }
}