pub mod test_func {
    use rand::{distributions::Alphanumeric, Rng};

    pub fn random_gen() -> Vec<char> {
        let bit_length = rand::thread_rng().gen_range(5..10);

        let s: Vec<char> = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(bit_length)
            .map(char::from)
            .collect();

        s
    }

    pub fn mass_gen(sum1: u32) -> Vec<Vec<char>> {
        let mut res = vec![];
        for _ in 0..sum1 as usize {
            res.push(random_gen());
        }

        res
    }
}
