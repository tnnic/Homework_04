pub struct SignedCounter(isize);

impl SignedCounter {
    // Метод для создания счетчика по умолчанию
    pub fn new() -> Self {
        SignedCounter(0)
    }

    // Метод для уменьшения значения счетчика
    pub fn prev(&self) -> SignedCounter {
        SignedCounter(self.0 - 1)
    }

    // Метод для увеличения значения счетчика
    pub fn next(&self) -> SignedCounter {
        SignedCounter(self.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_signed_counter() {
        assert_eq!(SignedCounter::new().0, 0);
    }

    #[test]
    fn test_prev_signed() {
        let counter = SignedCounter(5);
        assert_eq!(counter.prev().0, 4);
    }

    #[test]
    fn test_next_signed() {
        let counter = SignedCounter(5);
        assert_eq!(counter.next().0, 6);
    }
}
