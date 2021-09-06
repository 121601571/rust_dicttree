mod lib;
mod mass_test;

fn main() {
    let mut a = lib::dict_tree::MyTree::new(' ', true);
    a.add_chars(vec!['a', 'a', 'c']);
    a.add_chars(vec!['a', 'b', 'd']);
    a.add_chars(vec!['a', 'b', 'e']);
    a.add_chars(vec!['a', 'b']);
    let res = mass_test::test_func::mass_gen(1000);
    for i in res{
        a.add_chars(i);
    }
    let target: Vec<char> = "ab".chars().collect();
    let ok1 = a.find_suffix(target);
    println!("{:?}", ok1);
    //
    // let ll = a.find_suffix(vec!['a', 'b']);
    // println!("{:?}", ll);
    // let l2 = a.iter_self_values_dry();
    // println!("{:?}", l2);

    // let todo = mass_gen(1000000);
    // for i in todo {
    //     a.add_chars(i);
    // }
    //
    // let target: Vec<char> = "abc".chars().collect();
    // let ok1 = a.contains_whole(target);
    // println!("{}", ok1);
}
