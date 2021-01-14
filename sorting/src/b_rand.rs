// generate random numbers by storing a large number in curr,
// multiplying by another lager number
// incrementing that new number
// and then taking the modulo, so it remains with a given range
// mul inc should be less than the modulo, to save computation
// they should be prime, to slow frequency of repeated numbers
use lazy_static;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}
pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_val(max)
}
pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 4259461,
            inc: 7060771,
            modulo: 81935240129,
        }
    }

    pub fn next_val(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}
