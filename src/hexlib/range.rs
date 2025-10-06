use std::cmp;
use std::ops::Sub;

trait UnsignedInteger {}
impl UnsignedInteger for u8 {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}
impl UnsignedInteger for u128 {}
impl UnsignedInteger for usize {}

#[derive(Clone)]
pub struct Range<T: UnsignedInteger> {
    pub start: T,
    pub end: T,
}

pub type AddressRange = Range<u32>;

impl<T: UnsignedInteger + PartialOrd + Ord + Clone + Copy + Sub<Output = T>> Range<T> {
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
    pub fn merge(&mut self, range: &Range<T>) -> bool {
        // もし範囲が重なっている場合はマージする
        if self.start <= range.end && self.end >= range.start {
            self.start = cmp::min(self.start, range.start);
            self.end = cmp::max(self.end, range.end);
            return true;
        }
        return false;
    }

    /**
     * * Rangeのサイズを取得
     *
     * * @return サイズ
     */
    pub fn size(&self) -> T {
        self.end - self.start
    }
}

#[cfg(test)]
mod address_range_tests {
    use super::*;

    #[test]
    fn address_range_normal() {
        let range = AddressRange::new(30, 70);

        // 前方重なり
        let mut range2 = AddressRange::new(0, 40);
        assert_eq!(range2.merge(&range), true);
        assert_eq!(range2.start, 0);
        assert_eq!(range2.end, 70);

        // 後方重なり
        let mut range3 = Range::new(40, 100);
        assert_eq!(range3.merge(&range), true);
        assert_eq!(range3.start, 30);
        assert_eq!(range3.end, 100);

        // 内包
        let mut range4 = Range::new(0, 100);
        assert_eq!(range4.merge(&range), true);
        assert_eq!(range4.start, 0);
        assert_eq!(range4.end, 100);

        // 外包
        let mut range5 = Range::new(40, 60);
        assert_eq!(range5.merge(&range), true);
        assert_eq!(range5.start, 30);
        assert_eq!(range5.end, 70);

        // 完全に重なっていない
        let mut range6 = Range::new(80, 100);
        assert_eq!(range6.merge(&range), false);
        assert_eq!(range6.start, 80);
        assert_eq!(range6.end, 100);

        // 境界値
        let mut range7 = Range::new(0, 30);
        assert_eq!(range7.merge(&range), true);
        assert_eq!(range7.start, 0);
        assert_eq!(range7.end, 70);

        // 境界値
        let mut range8 = Range::new(70, 100);
        assert_eq!(range8.merge(&range), true);
        assert_eq!(range8.start, 30);
        assert_eq!(range8.end, 100);

        let no_range = AddressRange::new(0, 0);
        assert_eq!(no_range.size(), 0);
    }

    #[test]
    #[should_panic]
    fn address_range_abnormal() {
        let _range = AddressRange::new(70, 30);
    }
}
