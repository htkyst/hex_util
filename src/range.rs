use std::cmp;

pub struct Range<T> {
    start: T,
    end: T,
}

impl<T: PartialOrd + Ord + Clone + Copy> Range<T> {
    /**
     * * Rangeの新規作成
     * 
     * * @param start_addr 開始アドレス
     * * @param end_addr 終了アドレス
     */
    pub fn new(start_addr: T, end_addr: T) -> Range<T> {
        if start_addr > end_addr {
            panic!("Invalid range: start address is greater than end address");
        }
        Range {
            start: start_addr,
            end: end_addr,
        }
    }

    /**
     * * Rangeのマージ
     * 
     * * @param range マージするRange
     * * @return true: マージ成功, false: マージ失敗
     */
    pub fn merge(&mut self, range: Range<T>) -> bool {
        // もし範囲が重なっている場合はマージする
        if self.start <= range.end || self.end >= range.start {
            self.start = cmp::min(self.start, range.start);
            self.end = cmp::max(self.end, range.end);
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
