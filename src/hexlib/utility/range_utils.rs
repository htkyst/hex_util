use crate::hexlib::memory::range::AddressRange;

/**
 * Rebuild address ranges by merging overlapping or adjacent ranges
 *
 * @param ranges Existing address ranges
 * @param new_range New address range to add
 */
pub fn rebuild_ranges(ranges: &mut Vec<AddressRange>, new_range: AddressRange) {
    let mut rebuild_ranges: Vec<AddressRange> = Vec::new();
    let mut merged = false;

    for range in ranges.iter_mut() {
        if range.merge(&new_range) {
            rebuild_ranges.push(range.clone());
            merged = true;
        } else {
            rebuild_ranges.push(range.clone());
        }
    }

    if !merged {
        rebuild_ranges.push(new_range);
    }

    // Sort ranges by start address
    rebuild_ranges.sort_by_key(|r| r.start);
    *ranges = rebuild_ranges.into_iter().map(|r| r.clone()).collect();
}