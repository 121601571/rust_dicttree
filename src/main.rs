use rand::{distributions::Alphanumeric, Rng}; // 0.8

#[derive(Debug, Clone)]
struct MyTree {
    val: char,
    suffix: Option<Vec<MyTree>>,
    endpoint: bool,
}
//preceded with space as sp..
impl MyTree {
    fn contains_value(&self, input: char) -> bool {
        if let Some(val) = &self.suffix {
            for i in val {
                if i.val == input {
                    return true;
                }
            }
        }
        false
    }

    fn iter_self_values(&self) -> Vec<Vec<char>> {
        let mut res = vec![];
        match &self.suffix {
            None => {
                let v1 = vec![self.val];
                res.push(v1);
                return res;
            }
            Some(s1) => {
                for i in s1 {
                    let lv1 = i.iter_self_values();
                    for j in lv1 {
                        let mut v1 = vec![self.val];
                        v1.extend(j);
                        res.push(v1);
                    }
                }
                if self.endpoint == true {
                    res.push(vec![self.val]);
                }
            }
        }

        res
    }

    fn iter_value_node(&mut self, input: char) -> &mut MyTree {
        if let Some(val) = &mut self.suffix {
            for i in val {
                if i.val == input {
                    return i;
                }
            }
        }
        panic!()
    }

    pub fn new(node_val: char, ep: bool) -> Self {
        MyTree {
            val: node_val,
            suffix: None,
            endpoint: ep,
        }
    }

    pub fn contains_whole(&mut self, input: Vec<char>) -> bool {
        if input.len() == 0 {
            return true;
        }
        let first = input[0];
        if self.contains_value(first) {
            let node1 = self.iter_value_node(first);
            let mut rest = vec![' '; input.len() - 1];
            rest[..].clone_from_slice(&input[1..]);
            return node1.contains_whole(rest);
        } else {
            return false;
        }
    }

    pub fn add_chars(&mut self, input: Vec<char>) {
        if input.len() == 1 {
            if self.contains_value(input[0]) {
                return;
            }
            let new_node = MyTree::new(input[0], true);
            match &mut self.suffix {
                None => {
                    let mut v1 = vec![];
                    v1.push(new_node);
                    self.suffix = Some(v1);
                }
                Some(vlist) => {
                    vlist.push(new_node);
                }
            }
        } else {
            let first = input[0];
            let mut rest = vec![' '; input.len() - 1];
            rest[..].clone_from_slice(&input[1..]);
            if self.contains_value(first) {
                let node1 = self.iter_value_node(first);
                node1.add_chars(rest);
            } else {
                //new and add..
                let mut new_node = MyTree::new(first, false);
                new_node.add_chars(rest);

                match &mut self.suffix {
                    None => {
                        let mut v1 = vec![];
                        v1.push(new_node);
                        self.suffix = Some(v1);
                    }
                    Some(vlist) => {
                        vlist.push(new_node);
                    }
                }
            }
        }
    }
}

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
    let mut a = MyTree::new(' ', true);
    let todo = mass_gen(1000000);
    for i in todo {
        a.add_chars(i);
    }

    let target: Vec<char> = "abc".chars().collect();
    let ok1 = a.contains_whole(target);
    println!("{}", ok1);
}

