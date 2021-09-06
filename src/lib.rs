

pub mod dict_tree {
    #[derive(Debug, Clone)]
    pub struct MyTree {
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

        pub fn find_suffix(&mut self, input: Vec<char>) -> Vec<Vec<char>> {
            let mut res = vec![];
            if input.len() == 1 {
                match &self.suffix {
                    None => {
                        return res;
                    }
                    Some(sf) => {
                        for i in sf {
                            if i.val == input[0] {
                                return i.iter_self_values(false);
                            }
                        }
                    }
                }
            } else {
                let first = input[0];
                match &self.suffix {
                    None => {
                        return res;
                    }
                    Some(sf) => {
                        for i in sf {
                            if i.val == first {
                                //
                                let t_node = self.iter_value_node(first);
                                let mut rest = vec![' '; input.len() - 1];
                                rest[..].clone_from_slice(&input[1..]);
                                return t_node.find_suffix(rest);
                            }
                        }
                    }
                }
            }
            res
        }

        pub fn iter_self_values_dry(&self) -> Vec<Vec<char>> {
            let mut res = vec![];
            let s1 = self.iter_self_values(true);
            for i in s1 {
                let mut rest = vec![' '; i.len() - 1];
                rest[..].clone_from_slice(&i[1..]);
                if rest.len() > 0 {
                    res.push(rest);
                }
            }

            res
        }

        fn iter_self_values(&self, contains_self: bool) -> Vec<Vec<char>> {
            let mut res = vec![];
            match &self.suffix {
                None => {
                    let v1 = vec![self.val];
                    res.push(v1);
                    return res;
                }
                Some(s1) => {
                    for i in s1 {
                        let lv1 = i.iter_self_values(contains_self);
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
                    //mark...
                    let mut node1 = self.iter_value_node(input[0]);
                    node1.endpoint = true;
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


}
