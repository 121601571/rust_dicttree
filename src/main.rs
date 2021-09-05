use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
struct MyTree {
    val: char,
    suffix: Option<Vec<MyTree>>,
}
//preceded with space as sp..
impl MyTree {
    pub fn contains_value(&self, input: char) -> bool {
        if let Some(val) = &self.suffix {
            for i in val {
                if i.val == input {
                    return true;
                }
            }
        }
        false
    }

    pub fn show_all(&self)->Vec<Vec<char>>{
        let mut res = vec![];


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

    pub fn new(node_val: char) -> Self {
        MyTree {
            val: node_val,
            suffix: None,
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

        false
    }

    pub fn add_chars(&mut self, input: Vec<char>) {
        if input.len() == 1 {
            if self.contains_value(input[0]) {
                return;
            }
            let new_node = MyTree::new(input[0]);
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
                let mut node1 = self.iter_value_node(first);
                node1.add_chars(rest);
            } else {
                //new and add..
                let mut new_node = MyTree::new(first);
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

fn main() {
    println!("Hello, world!");
    let mut a = MyTree::new(' ');
    a.add_chars(vec!['a', 'b']);
    a.add_chars(vec!['a', 'c']);
    a.add_chars(vec!['b', 'c']);
    a.add_chars(vec!['a', 'c', 'd']);

    println!("{:?}", a);
    println!("{}", a.contains_whole(vec!['c', 'd', 'e']));
}
