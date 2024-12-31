pub struct AddressRange {
    start: u32,
    end: u32,
}

impl AddressRange {
    pub fn new(start_addr: u32, end_addr: u32) -> AddressRange {
        AddressRange {
            start: start_addr,
            end: end_addr,
        }
    }

    pub fn merge(&mut self, range: AddressRange) -> bool {
        if self.end >= range.start && range.end >= self.start {
            self.start = self.start.min(range.start);
            self.end = self.end.max(range.end);
            return true;
        }

        return false;
    }
}

#[cfg(test)]
mod address_range_tsts {
    use super::*;

    #[test]
    fn merge_normal() {
        let mut range1 = AddressRange::new(30, 100);
        let range2 = AddressRange::new(10, 40);

        assert_eq!(range1.merge(range2), true);
        assert_eq!(range1.start, 10);
        assert_eq!(range1.end, 100);

        let range3 = AddressRange::new(90, 140);

        assert_eq!(range1.merge(range3), true);
        assert_eq!(range1.start, 10);
        assert_eq!(range1.end, 140);

        let range4 = AddressRange::new(200, 300);

        assert_eq!(range1.merge(range4), false);
        assert_eq!(range1.start, 10);
        assert_eq!(range1.end, 140);
    }
}
