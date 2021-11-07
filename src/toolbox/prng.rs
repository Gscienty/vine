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
    let z = prng.get(idx);
    *z = (((*z << p[1]) ^ *z) >> (p[0] - p[2]))
        ^ ((*z & (0xffffffffffffffff << (64 - p[0]))) << p[2]);
    *r = *r ^ *z;
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

    fn get(&mut self, i: usize) -> &mut u64 {
        &mut self.u[i]
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

        (*self.get(3) & 0x000fffffffffffff) | 0x3ff0000000000000
    }

    fn gen_u64(&mut self) -> u64 {
        let mut r: u64 = 0;
        tw223_step(self, &mut r);

        *self.get(3)
    }

    fn condition(&mut self) {
        if *self.get(0) < (1u64 << 1) {
            *self.get(0) += 1u64 << 1;
        }
        if *self.get(1) < (1u64 << 6) {
            *self.get(1) += 1u64 << 1;
        }
        if *self.get(2) < (1u64 << 9) {
            *self.get(2) += 1u64 << 9;
        }
        if *self.get(3) < (1u64 << 17) {
            *self.get(3) += 1u64 << 17;
        }
    }
}
