/// O(1) implementation of dictionary with [`usize`] key.
///
/// Operations(all O(1)) time complexity):
/// - [`insert`]: insert an instance of T and returns auto-allocated key.
/// - [`get`]: get the instance of T.
/// - [`remove`]: remove the instance of T.
///
/// Space complexity: O(maximum size of dictionary at any time in the history)
///
/// Invariant:
/// - `v.len() >= 2`.
/// - `v[0]` and `v[v.len() - 1]` are always vacant nodes.
/// - all vacant nodes are bidirectionally rinked from v[0] to v[v.len() - 1].
pub struct FastIntDictionary<T> {
    v: Vec<Node<T>>,
    size: usize,
}

enum Node<T> {
    InUse(T),
    Vacant((usize, usize)),
}

impl<T> FastIntDictionary<T> {
    pub fn new() -> Self {
        Self {
            v: vec![Node::Vacant((usize::MAX, 1)), Node::Vacant((0, usize::MAX))],
            size: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> usize {
        let second_first_index = if let Node::Vacant((_, next)) = self.v[0] {
            next
        } else {
            panic!("Invariant violated: v[0] is not a vacant node");
        };
        if second_first_index == self.v.len() - 1 {
            self.push_vacant_back();
        };
        if let Node::Vacant((prev, next)) = self.v[second_first_index] {
            self.v[second_first_index] = Node::InUse(value);
            if let Node::Vacant((prev_prev, _)) = self.v[prev] {
                self.v[prev] = Node::Vacant((prev_prev, next));
            }
            if let Node::Vacant((_, next_next)) = self.v[next] {
                self.v[next] = Node::Vacant((prev, next_next));
            }
        } else {
            panic!("Invariant violated: second_first_index is not a vacant node");
        }
        self.size += 1;
        second_first_index
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        if let Node::InUse(value) = &self.v[key] {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        if let Node::InUse(value) = &mut self.v[key] {
            Some(value)
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        if let Node::Vacant(_) = self.v[key] {
            return None;
        }
        let second_first_index = if let Node::Vacant((_, next)) = self.v[0] {
            next
        } else {
            panic!("Invariant violated: v[0] is not a vacant node");
        };
        let third_first_index = if let Node::Vacant((_, next)) = self.v[second_first_index] {
            next
        } else {
            panic!("Invariant violated: second_first_index is not a vacant node");
        };
        self.v[0] = Node::Vacant((usize::MAX, key));
        let v_key_replacement = Node::Vacant((0, second_first_index));
        let value = std::mem::replace(&mut self.v[key], v_key_replacement);
        self.v[second_first_index] = Node::Vacant((key, third_first_index));
        self.size -= 1;
        if let Node::InUse(value) = value {
            Some(value)
        } else {
            None
        }
    }

    fn push_vacant_back(&mut self) {
        let formar_length = self.v.len();
        if let Node::Vacant((prev, _)) = self.v[formar_length - 1] {
            self.v[formar_length - 1] = Node::Vacant((prev, formar_length));
            self.v.push(Node::Vacant((formar_length - 1, usize::MAX)));
        } else {
            panic!("Invariant violated: last node is not a vacant node");
        }
    }

    fn pop_vacant_back(&mut self) {
        let former_length = self.v.len();
        self.v.pop();
        if let Node::Vacant((prev, _)) = self.v[former_length - 2] {
            self.v[former_length - 2] = Node::Vacant((prev, usize::MAX));
        } else {
            panic!("Invariant violated: second-to-last node is not a vacant node");
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod test {
    pub struct XorShift {
        pub state: u64,
    }

    impl XorShift {
        pub fn next(&mut self) -> u64 {
            let mut x = self.state;

            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;

            self.state = x;
            x
        }
    }

    #[test]
    fn test_test_util_xorshift_works() {
        let seed = 43;
        let iteration = 10000;
        let mut rng = XorShift { state: seed };
        let mut blacklist = std::collections::HashSet::new();
        for _ in 0..iteration {
            let current = rng.next();
            assert!(!blacklist.contains(&current));
            blacklist.insert(current);
        }
    }

    #[test]
    fn test_fast_int_dictionary() {
        use super::*;
        let seed = 42;
        let iteration = 10000;
        let mut rng = XorShift { state: seed };
        let mut map_truth: std::collections::HashMap<usize, u64> = std::collections::HashMap::new();
        let mut map = FastIntDictionary::new();
        for _ in 0..iteration {
            let is_insert_op = map_truth.len() == 0 || rng.next() & 3 != 0;
            if is_insert_op {
                let value = rng.next();
                let key = map.insert(value);
                map_truth.insert(key, value);
            } else {
                let key_pos = rng.next() % (map_truth.len() as u64);
                let key = *map_truth.keys().nth(key_pos as usize).unwrap();
                let value_truth = map_truth.remove(&key).unwrap();
                let value = map.remove(key);
                assert_eq!(value, Some(value_truth));
            }
        }
    }
}
