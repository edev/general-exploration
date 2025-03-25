//! A Rust port of my submitted C++ solution. This is the algorithm that I seek to verify.

/// Rust port of my submitted solution.
pub fn sliding_window(mut nums: Vec<i32>) -> i32 {
    let mut ops: i32 = 0;
    for i in 0..nums.len() {
        if nums[i] == 0 {
            if !flip(&mut nums, i) {
                return -1;
            }
            ops += 1;
        }
    }
    ops
}

/// Returns whether the requested flip was legal. If `true`, the flip was performed. If `false`,
/// `nums` was not modified.
fn flip(nums: &mut [i32], index: usize) -> bool {
    if index + 2 >= nums.len() {
        return false;
    }

    for offset in 0..3 {
        nums[index + offset] ^= 1;
    }
    true
}
