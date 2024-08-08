#[derive(PartialEq)]
#[derive(Debug)]
pub struct Pair(i32, i32);

impl Pair {
    // Метод для создания пары по умолчанию
    pub fn new() -> Self {
        Pair(0, 0)
    }

    // Метод для суммирования двух пар (поэлементно)
    pub fn vector_sum(&self, other: &Pair) -> Pair {
        Pair(self.0 + other.0, self.1 + other.1)
    }

    // Метод для скалярной суммы элементов пары
    pub fn scalar_sum(&self, other: &Pair) -> i32 {
        self.0 + self.1 + other.0 + other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pair() {
        assert_eq!(Pair::new(), Pair(0, 0));
    }

    #[test]
    fn test_pair_vector_sum() {
        let pair1 = Pair(1, 2);
        let pair2 = Pair(3, 4);
        assert_eq!(pair1.vector_sum(&pair2), Pair(4, 6));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let pair1 = Pair(1, 2);
        let pair2 = Pair(3, 4);
        assert_eq!(pair1.scalar_sum(&pair2), 10);
    }
}
