/// A `Table` enumerates possible results.
#[derive(Debug)]
pub struct Table {
    values: Vec<u32>,
    min: usize,
    norm: u32,
}

impl Table {
    pub fn new(min: usize, norm: u32, values: Vec<u32>) -> Table {
        Table {
            values: values,
            min: min,
            norm: norm,
        }
    }

    /// Returns the chance of a certain value.
    pub fn chance_of(&self, x: usize) -> f32 {
        let n = self.norm as f32;

        self.permutations_of(x) as f32 / n
    }

    /// Returns the number of possible permutations that gives this result.
    pub fn permutations_of(&self, x: usize) -> u32 {
        self.values[x - self.min]
    }

    pub fn to_permutations(&self) -> Vec<(usize, u32)> {
        self.values
            .iter()
            .enumerate()
            .map(|(i, &v)| (i + self.min, v))
            .collect()
    }

    pub fn to_chances(&self) -> Vec<(usize, f32)> {
        let n = self.norm as f32;

        self.values
            .iter()
            .enumerate()
            .map(|(i, &v)| (i + self.min, v as f32 / n))
            .collect()
    }
}
