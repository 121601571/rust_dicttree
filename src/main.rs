mod lib;

use rand::{distributions::Alphanumeric, Rng};

fn random_gen() -> Vec<char> {
    let bit_length = rand::thread_rng().gen_range(5..10);

    let s: Vec<char> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(bit_length)
        .map(char::from)
        .collect();

    s
}

fn mass_gen(sum1: u32) -> Vec<Vec<char>> {
    let mut res = vec![];
    for _ in 0..sum1 as usize {
        res.push(random_gen());
    }

    res
}

fn main() {
    let mut a = lib::dict_tree::MyTree::new(' ', true);
    a.add_chars(vec!['a', 'a', 'c']);
    a.add_chars(vec!['a', 'b', 'd']);
    a.add_chars(vec!['a', 'b', 'e']);
    a.add_chars(vec!['a', 'b']);

    let ll = a.find_suffix(vec!['a', 'b']);
    println!("{:?}", ll);
    let l2 = a.iter_self_values_dry();
    println!("{:?}", l2);

    // let todo = mass_gen(1000000);
    // for i in todo {
    //     a.add_chars(i);
    // }
    //
    // let target: Vec<char> = "abc".chars().collect();
    // let ok1 = a.contains_whole(target);
    // println!("{}", ok1);
}
