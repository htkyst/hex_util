pub struct Range<T: Default = u32> {
    start: T,
    end: T,
}

impl<T> Range<T> {
    pub fn new<T>(start_addr: T, end_addr: T) -> Range<T> {
        Range {
            start: start_addr,
            end: end_addr,
        }
    }

    pub fn merge<T>(&mut self, range: Range<T>) -> bool {
        if self.end >= range.start && range.end >= self.start {
            self.start = self.start.min(range.start);
            self.end = self.end.max(range.end);
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod address_range_tests {
    use super::*;

    #[test]
    fn merge_normal() {
        let mut range1 = Range::new(30, 100);
        let range2 = Range::new(10, 40);

        assert_eq!(range1.merge(range2), true);
        assert_eq!(range1.start, 10);
        assert_eq!(range1.end, 100);

        let range3 = Range::new(90, 140);

        assert_eq!(range1.merge(range3), true);
        assert_eq!(range1.start, 10);
        assert_eq!(range1.end, 140);

        let range4 = Range::new(200, 300);

        assert_eq!(range1.merge(range4), false);
        assert_eq!(range1.start, 10);
        assert_eq!(range1.end, 140);
    }
}
