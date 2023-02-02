struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Counter;

    #[test]
    fn limit_counter() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);

        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(sum, 18);

        let v = vec![1u64, 2, 3, 4, 5, 6];
        let val = v
            .iter()
            .enumerate()
            // 每两个元素剔除一个
            // [1, 3, 5]
            .filter(|&(idx, _)| idx % 2 == 0)
            .map(|(idx, val)| val)
            // 累加 1+3+5 = 9
            .fold(0u64, |sum, acm| sum + acm);

        println!("{}", val);
    }
}
