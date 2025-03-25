//! Exhaustive search of the problem space (up to certain limits).

use std::collections::VecDeque;

/// Performs an exhaustive search for optimal solutions.
///
/// Due to internal optimizations, this algorithm only considers inputs with lengths in the range
/// `[3, 32]`. This uses about 4 GiB of RAM. Extending this to 64 is technically possible but not
/// recommended, as it would require more than `2^64` bytes (4 exbibytes) of RAM.
///
/// # Notional representation
///
/// `nums` is converted to a 32-bit unsigned integer such that `nums[0]` is the least-significant
/// bit and subsequent vector indices represent increasingly significant bits. Each possible
/// transformed state is ordered according to its value when represented in this same way. For
/// instance, a hypothetical state `[0, 1, 1, 0, 1]` is ordered as `0b10110` (22 in decimal).
///
/// A flip operation is represented by a `flip_index` value that corresponds to the lowest index of
/// the flipped digits in the original `nums`. For instance, a flip of digits `0..=2` is
/// represented by a `flip_index` value of `0`.
///
/// Given an arbitrary state and a length `n`, there are `n - 3` sensible flip operations: there
/// are `n - 2` legal flip operations, since the last two `flip_index` values are illegal, and
/// redoing the previous transformation is not productive, as it would merely undo the change. (For
/// the very first operation, there are `n - 2` sensible operations.)
///
/// # Algorithm
///
/// We define a graph of states represented as vertices and flip operations as undirected edges. We
/// build the graph breadth-first, tracking our count of operations as we go (and limiting our
/// depth  as specified below). When we add an edge that leads to an existing node, we do not add
/// it to the traversal queue, as there is already an equal-or-lower-cost path to that node. If we
/// reach our goal state of `2^n - 1`, we return our operation count. Otherwise, we return `-1`
/// after building the complete graph.
///
/// If `limit > 0`, we only consider solutions up to depth `limit` (inclusive). Otherwise, we
/// follow the algorithm above without arbitrary limits.
///
/// # Data structure
///
/// All we need to know about an edge is whether we have already visited the vertex on the other
/// end. This means we don't actually need to store edges. Omitting edges reduces our memory
/// footprint enormously and makes this algorithm feasible on desktop hardware. (Some napkin math
/// reveals that a graph for an input with length 32 might store `32 * 2^32` 4-byte edges, which
/// would consume 512 GiB.)
///
/// Each vertex is either visited or unvisited; we do not need to store any other information about
/// the vertex. Since we have ordered our states within the range of 32-bit integers, we simply
/// store `2^32` [bool]s in a plain [Vec]: `false` means unvisited, and `true` means
/// visited. This gives us `O(1)` access time at the cost of 4 GiB of RAM.
///
/// Additionally, we store a traversal queue. Each entry in the queue stores a vertex to visit and
/// the number of operations to reach that vertex.
///
/// # Memory usage
///
/// There are `2^n` possible states, and an input with no solutions and a negative `limit` will
/// utilize some or perhaps all of them. This makes memory the limiting factor in input length.
/// The vector of vertices will always be 4 GiB (plus a few bytes). Each entry in the traversal
/// queue is 8 bytes (two [i32] values), so the upper limit (which will never be reached) is 32
/// GiB: `2^32 states * 8 bytes/state`. In practice, the traversal queue appears to remain small
/// enough to be inconsequential.
pub fn exhaustive_search(nums: Vec<i32>, limit: i32) -> i32 {
    let len = nums.len();
    let goal = match len {
        0..3 => return -1,
        3..32 => 2_u32.pow(len as u32) - 1,
        32 => u32::MAX,
        n => panic!("nums.len() should be between 3 and 32 (inclusive) but was: {n}"),
    };

    let mut visited = vec![false; 2_usize.pow(32)];
    let mut traversal_queue = VecDeque::new();

    // Convert `nums` to `u32`.
    //
    // IMPORTANT: see doc comment on `Traversal::vertex`!
    let mut start: u32 = 0;
    for n in nums.iter().rev() {
        start <<= 1;
        match n {
            0 => (),
            1 => start += 1,
            n => panic!("nums contained a value other than 0 and 1: {n}"),
        }
    }

    traversal_queue.push_back(Traversal {
        vertex: start,
        operations: 0,
    });
    while let Some(step) = traversal_queue.pop_front() {
        if step.vertex == goal {
            return step.operations;
        } else if limit > 0 && step.operations > limit {
            return -1;
        }

        visited[step.vertex as usize] = true;

        for flip_index in 0..(len - 2) {
            // Move the binary pattern `111` over `flip_index` bits and then XOR those bits of the
            // starting state.
            let vertex = step.vertex ^ (0b111_u32 << flip_index);
            if !&visited[vertex as usize] {
                traversal_queue.push_back(Traversal {
                    vertex,
                    operations: step.operations + 1,
                });
            }
        }
    }
    -1
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Traversal {
    /// Represents a state of the transformed input `nums`. Counting from the least-significant
    /// bit, bit `n` represents `nums[n]`. For instance, if `nums = [1, 1, 0, 1]`, then `vertex =
    /// 0b1011`. This appears backwards to the human eye but makes manipulating bits intuitive.
    vertex: u32,

    // Due to alignment requirements, `operations` will be padded to 4 bytes, so we might as well
    // use them to help ensure that we don't overflow. I'm not sure of the upper bound, but I do
    // not believe it is anywhere near `u32::MAX`.
    operations: i32,
}
