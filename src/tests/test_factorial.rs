

#[cfg(test)]
mod test_factorial {
    
    use malachite::Natural;
    use malachite::num::arithmetic::traits::Factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(Natural::factorial(6), 720);
        
    }
}