use verify_leetcode_3191::exhaustive_search::exhaustive_search;
use verify_leetcode_3191::nums_iter;
use verify_leetcode_3191::solution::sliding_window;

fn main() {
    for n in 1..=32 {
        print!("Checking n = {n}... ");
        let mut error_free = true;
        for nums in nums_iter(5) {
            let sw = sliding_window(nums.clone());
            let es = exhaustive_search(nums.clone(), sw);
            if sw != es {
                if error_free {
                    error_free = false;
                    println!("Errors:");
                }
                println!("{nums:?}: {sw:4}{es:4}");
            }
        }

        if error_free {
            println!("All checks passed!");
        }
    }
}
