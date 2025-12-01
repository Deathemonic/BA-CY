use rand_mt::Mt;

pub struct MersenneTwister {
    rng: Mt,
}

impl MersenneTwister {
    #[inline]
    pub fn new(seed: u32) -> Self {
        Self {
            rng: Mt::new(seed),
        }
    }

    #[inline]
    pub fn next(&mut self) -> u32 {
        self.rng.next_u32() >> 1
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let high = self.rng.next_u32() as u64;
        let low = self.rng.next_u32() as u64;
        (high << 32) | low
    }

    pub fn next_bytes(&mut self, buf: &mut [u8]) {
        let (chunks, remainder) = buf.as_chunks_mut::<4>();
        for chunk in chunks {
            let num = self.next();
            *chunk = num.to_le_bytes();
        }
        if !remainder.is_empty() {
            let num = self.next();
            let bytes = num.to_le_bytes();
            remainder.copy_from_slice(&bytes[..remainder.len()]);
        }
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        self.rng.next_u32() as f32 * (1.0 / 4294967296.0)
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        let a = (self.rng.next_u32() >> 5) as f64;
        let b = (self.rng.next_u32() >> 6) as f64;
        (a * 67108864.0 + b) * (1.0 / 9007199254740992.0)
    }

    #[inline]
    pub fn next_range(&mut self, min: i32, max: i32) -> i32 {
        let (min, max) = if min > max { (max, min) } else { (min, max) };
        ((max - min) as f64 * self.next_f64() + min as f64).floor() as i32
    }
}
