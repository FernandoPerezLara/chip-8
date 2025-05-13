const CYCLE_SIZE: usize = 4096;
const PHI: u32 = 0x9E3779B9;

pub struct MultiplyWithCarry {
    history: [u32; CYCLE_SIZE],
    carry: u32,
    index: usize,
}

impl MultiplyWithCarry {
    pub fn new(seed: u32) -> Self {
        let mut history = [0; CYCLE_SIZE];

        history[0] = seed;
        history[1] = seed.wrapping_add(PHI);
        history[2] = seed.wrapping_add(PHI).wrapping_add(PHI);

        for i in 3..CYCLE_SIZE {
            let window = &mut history[i - 3..i + 1];
            window[3] = window[0] ^ window[1] ^ PHI ^ seed;
        }

        Self {
            history,
            carry: 362436,
            index: 4095,
        }
    }

    pub fn random(&mut self) -> u32 {
        const A: u64 = 18782;
        const R: u32 = 0xFFFFFFFE;

        self.index = (self.index + 1) & (CYCLE_SIZE - 1);
        let t = A * self.history[self.index] as u64 + self.carry as u64;

        self.carry = (t >> 32) as u32;
        let mut x = (t + self.carry as u64) as u32;
        if x < self.carry {
            x += 1;
            self.carry += 1;
        }

        self.history[self.index] = R - x;
        self.history[self.index]
    }
}
