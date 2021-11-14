// 随机数
pub struct PrngState {
    u: [u64; 4],
}

// 随机数方法
pub trait Prng {
    fn seed_secure(&mut self);
    fn condition(&mut self);
    fn gen_u64(&mut self) -> u64;
    fn gen_u64d(&mut self) -> u64;
}

fn tw223_gen(prng: &mut PrngState, idx: usize, r: &mut u64, p: [i32; 3]) {
    if let Some(z) = prng.get_mut(idx) {
        *z = (((*z << p[1]) ^ *z) >> (p[0] - p[2]))
            ^ ((*z & (0xffffffffffffffff << (64 - p[0]))) << p[2]);
        *r = *r ^ *z;
    }
}

fn tw223_step(prng: &mut PrngState, r: &mut u64) {
    tw223_gen(prng, 0, r, [63, 31, 18]);
    tw223_gen(prng, 1, r, [58, 19, 28]);
    tw223_gen(prng, 2, r, [55, 24, 7]);
    tw223_gen(prng, 3, r, [47, 21, 8]);
}

impl PrngState {
    pub fn new() -> PrngState {
        PrngState { u: [0; 4] }
    }

    fn get(&self, i: usize) -> Option<u64> {
        if i < 4 {
            Some(self.u[i])
        } else {
            None
        }
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut u64> {
        if i < 4 {
            Some(&mut self.u[i])
        } else {
            None
        }
    }
}

impl Prng for PrngState {
    fn seed_secure(&mut self) {
        self.u[0] = 0xa0d277570a345b8c;
        self.u[1] = 0x764a296c5d4aa64f;
        self.u[2] = 0x51220704070adeaa;
        self.u[3] = 0x2a2717b5a7b7b927;

        self.condition();
        self.gen_u64();
    }

    fn gen_u64d(&mut self) -> u64 {
        let mut r: u64 = 0;
        tw223_step(self, &mut r);

        (r & 0x000fffffffffffff) | 0x3ff0000000000000
    }

    fn gen_u64(&mut self) -> u64 {
        let mut r: u64 = 0;
        tw223_step(self, &mut r);

        r
    }

    fn condition(&mut self) {
        if self.get(0).lt(&Some(1u64 << 1)) {
            *self.get_mut(0).unwrap() += 1u64 << 1;
        }
        if self.get(1).lt(&Some(1u64 << 6)) {
            *self.get_mut(1).unwrap() += 1u64 << 6;
        }
        if self.get(2).lt(&Some(1u64 << 9)) {
            *self.get_mut(2).unwrap() += 1u64 << 6;
        }
        if self.get(3).lt(&Some(1u64 << 17)) {
            *self.get_mut(3).unwrap() += 1u64 << 17;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Prng, PrngState};

    #[test]
    fn init_prng() {
        let prng: PrngState = PrngState::new();

        for i in 0..4 {
            assert!(prng.get(i).is_some());
        }

        assert!(prng.get(4).is_none());
    }

    #[test]
    fn seed_secure() {
        let mut prng: PrngState = PrngState::new();

        for i in 0..4 {
            assert!(prng.get(i).eq(&Some(0)))
        }

        prng.seed_secure();

        for i in 0..4 {
            assert!(prng.get(i).ne(&Some(0)));
        }
    }

    #[test]
    fn gen_u64() {
        let mut prng: PrngState = PrngState::new();
        prng.seed_secure();

        let mut g1 = prng.gen_u64();

        for _ in 1..100 {
            prng.condition();
            let g2 = prng.gen_u64();

            assert_ne!(g1, g2);

            g1 = g2;
        }
    }
}
