use std::time::{SystemTime, UNIX_EPOCH};

/// Simple Linear Congruential Generator matching glibc's rand()
/// glibc uses: next = (a * prev + c) % m
/// where a = 1103515245, c = 12345, m = 2^31
pub struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    /// Create a new RNG seeded with current time (matching srand(time(NULL)))
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);
        Self { state: seed }
    }

    /// Generate next random number (matching glibc rand())
    pub fn next(&mut self) -> i32 {
        // glibc LCG parameters
        const A: u32 = 1103515245;
        const C: u32 = 12345;
        const M: u32 = 1 << 31;

        self.state = (A.wrapping_mul(self.state).wrapping_add(C)) % M;
        self.state as i32
    }

    /// Select a random element from a slice
    pub fn select<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        let index = (self.next() as usize) % items.len();
        &items[index]
    }
}
