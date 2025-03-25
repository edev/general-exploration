pub mod exhaustive_search;
pub mod solution;

/// Iterator that yields every possible `nums` of a given length.
pub struct NumsIter {
    len: usize,
    max: i32,
    next: i32,
}

impl Iterator for NumsIter {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.max {
            return None;
        }
        self.next += 1;

        // Build a `nums` value out of the bits of `self.next`. For sanity, assign the
        // least-significant bit the lowest index in `nums`.
        let mut nums = Vec::with_capacity(self.len);
        for digit in (0..self.len).rev() {
            nums.push((self.next >> digit) % 2);
        }
        Some(nums)
    }
}

/// Returns an iterator over all possible `nums` of length `len`.
pub fn nums_iter(len: u8) -> NumsIter {
    // Edge cases:
    // 0 => max = 0, so iterator never yields values
    // [1, 2] => iterator yields values, but algorithms should always return -1
    // [33, usize::MAX] => implementations may panic
    NumsIter {
        len: len as usize,
        max: 2_i32.pow(len as u32) - 1,
        next: 0,
    }
}
