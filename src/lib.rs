//! A crate implementing dices
 
/// Tables can be queried to determine chances
pub mod table;

use table::Table;

trait Die {
    fn roll(&self) -> u32;

    fn distribution(&self) -> Distribution;
}

#[derive(Debug)]
pub struct Distribution {
    values: Vec<u32>,
    at_least: Vec<u32>,
    at_most: Vec<u32>,
    /// The number of possible outcomes in the distribution.
    total_permutations: u32,
    /// The minimum value in the distribution.
    min: u32,
}

impl Distribution {
    pub fn new(outcomes: &[u32]) -> Distribution {
        let min = outcomes.iter().min().unwrap();
        let max = outcomes.iter().max().unwrap();
        let len = (max - min) as usize + 1;

        let mut values = Vec::with_capacity(len);
        let mut at_least = Vec::with_capacity(len);
        let mut at_most = Vec::with_capacity(len);
        for _ in 0..len {
            values.push(0u32);
        }

        for &o in outcomes {
            let o = (o - min) as usize;
            values[o] += 1;
        }

        let mut at_least_c = outcomes.len() as u32;
        let mut at_most_c = 0;

        for v in &values {
            at_least.push(at_least_c);
            at_least_c -= *v;
            at_most_c += *v;
            at_most.push(at_most_c);
        }

        Distribution {
            values: values,
            at_least: at_least,
            at_most: at_most,
            total_permutations: outcomes.len() as u32,
            min: *min }
    }

    /// Returns the lowest possible result.
    pub fn min(&self) -> u32 {
        self.min
    }

    /// Returns the highest possible result.
    pub fn max(&self) -> u32 {
        self.min() + (self.values.len() as u32) - 1
    }

    /// Returns a `Table` with the exact odds for each result.
    pub fn exactly(&self) -> Table {
        Table::new(self.min() as usize, self.total_permutations(), self.values.clone())
    }

    /// Returns a `Table` with the odds to get at least the result given.
    /// The lowest possible result will always have a 100% chance, and the
    /// highest possible result will always have the exact chance.
    pub fn at_least(&self) -> Table {
        Table::new(self.min() as usize, self.total_permutations(), self.at_least.clone())
    }

    /// Returns a `Table` with the odds to get at most the result given.
    /// The highest possible result will always have a 100% chance, and the
    /// lowest possible result will always have the exact chance.
    pub fn at_most(&self) -> Table {
        Table::new(self.min() as usize, self.total_permutations(), self.at_most.clone())
    }

    /// Returns the total number of possible permutations of the die.
    pub fn possible_permutations(&self) -> u32 {
        self.total_permutations
    }

    pub fn total_permutations(&self) -> u32 {
        self.total_permutations
    }

    pub fn total_combinations(&self) -> u32 {
        self.max() - self.min()
    }

    pub fn mean(&self) -> f32 {
        0.0
    }

    pub fn std_dev(&self) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let dist = super::Distribution::new(&[1, 2, 3, 4, 5, 6]);

        assert!(dist.min() == 1);
        assert!(dist.max() == 6);

        println!("Exact: {:?}", dist.exactly());
        println!("At least: {:?}", dist.at_least());
        println!("At most: {:?}", dist.at_most());

        assert!(true);
    }

    #[test]
    fn com() {
        let a = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        let mut v = Vec::with_capacity(a.len() * a.len() * a.len());

        for i in &a {
            for j in &a {
                for k in &a {
                    v.push(i + j + k);
                }
            }
        }

        let dist = super::Distribution::new(&v);
 
        println!("Exact: {:?}", dist.exactly());
        println!("At least: {:?}", dist.at_least());
        println!("At most: {:?}", dist.at_most().chance_of(2));

        assert!(false);
   }
}
