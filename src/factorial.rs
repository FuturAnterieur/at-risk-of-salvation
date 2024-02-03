pub struct FactorialCalculator {
    values : Vec::<usize>,
}

impl FactorialCalculator {

    pub fn new() -> Self {
        let mut ret = Self {values: Vec::<usize>::new()};
        ret.values.push(1);
        ret
    }

    pub fn factorial(&mut self, n : usize) -> usize {
        let s = self.values.len();

        if n >= s {
            for i in s..n+1 {
                self.values.push(i * self.values[i - 1]);
            }
        }
        self.values[n]
    }
}