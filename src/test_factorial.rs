

#[cfg(test)]
mod tests {
    #[test]
    fn test_factorial() {
        let mut calculator = crate::factorial::FactorialCalculator::new();
        assert_eq!(calculator.factorial(1), 1);
        assert_eq!(calculator.factorial(6), 720);
        assert_eq!(calculator.factorial(4), 24);
        assert_eq!(calculator.factorial(6), 720);
    }
}