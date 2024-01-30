use rand::Rng;

pub fn gen_ran() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
