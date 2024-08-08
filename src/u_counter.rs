pub struct UnsignedCounter(usize);

impl UnsignedCounter {
    // Метод для создания счетчика по умолчанию
    pub fn new() -> Self {
        UnsignedCounter(0)
    }

    // Метод для увеличения значения счетчика
    pub fn next(&self) -> UnsignedCounter {
        UnsignedCounter(self.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(UnsignedCounter::new().0, 0);
    }
    
    #[test]
    fn test_next_unsigned() {
        let counter = UnsignedCounter(5);
        assert_eq!(counter.next().0, 6);
    }
}
