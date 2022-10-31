use bstr::ByteSlice;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct TrieNode {
    node_char: u8,
    weight: u32,
    final_count: u32,
    data: u32,
    children: Vec<TrieNode>,
}

#[derive(Debug)]
pub struct Trie {
    roots: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Self { roots: Vec::new() }
    }

    pub fn insert(&mut self, s: &[u8], data: u32) {
        insert_trie(&mut self.roots, s, data)
    }

    pub fn lookup<'a>(&'a self, _s: &[u8]) -> TrieNodeIterator<'a> {
        TrieNodeIterator::new(&self.roots)
    }
}

fn insert_trie_then_sort(nodes: &mut Vec<TrieNode>, s: &[u8], data: u32) {
    insert_trie(nodes, s, data);
    nodes.sort_by(|x, y| y.weight.cmp(&x.weight));
}

fn insert_trie(nodes: &mut Vec<TrieNode>, s: &[u8], data: u32) {
    if (s.len() == 0) {
        return;
    }

    for mut node in nodes.iter_mut() {
        if (node.node_char == s[0]) {
            if (s.len() == 1) {
                node.weight += 1;
                node.final_count += 1;
            } else {
                node.weight += 1;
                insert_trie_then_sort(&mut node.children, &s[1..], data);
            }

            return;
        }
    }

    // Insert new node

    let final_count = if (s.len() == 1) { 1 } else { 0 };
    let data = if (s.len() == 1) { data } else { u32::MAX };

    nodes.push(TrieNode {
        node_char: s[0],
        weight: 1,
        final_count: final_count,
        data,
        children: Vec::new(),
    });

    insert_trie_then_sort(&mut nodes.last_mut().unwrap().children, &s[1..], data);
}

pub struct TrieNodeIterator<'a> {
    roots: &'a [TrieNode],
    stack: Vec<usize>,
    string: Vec<u8>,

    first_hack: bool,
}

impl<'a> TrieNodeIterator<'a> {
    pub fn new(roots: &'a [TrieNode]) -> Self {
        assert!(roots.len() > 0);

        Self {
            roots,
            //indexes: vec![0],
            stack: Default::default(),
            string: Default::default(),
            first_hack: true,
        }
    }

    fn current_node_level(&self) -> &'a [TrieNode] {
        let mut nl = self.roots;
        for index in &self.stack {
            let current_node = Some(&nl[*index]);
            nl = &nl[*index].children[..];
        }

        nl
    }

    fn current_node(&self) -> Option<&TrieNode> {
        let mut nl = self.roots;
        let mut current_node: Option<&TrieNode> = None;
        for index in &self.stack {
            current_node = Some(&nl[*index]);
            nl = &nl[*index].children[..];
        }

        current_node
    }

    pub fn next(&mut self) -> Option<&[u8]> {
        if (self.first_hack) {
            self.first_hack = false;

            while (self.current_node_level().len() > 0) {
                self.stack.push(0);
                let node = self.current_node().unwrap();
                self.string.push(node.node_char);
            }

            return Some(&self.string);
        }

        while let Some(last) = self.stack.pop() {
            _ = self.string.pop().unwrap();
            let level = self.current_node_level();

            if (last + 1 >= level.len()) {
                continue;
            }

            self.stack.push(last + 1);
            let node = self.current_node().unwrap();
            self.string.push(node.node_char);

            while (self.current_node_level().len() > 0) {
                self.stack.push(0);
                let node = self.current_node().unwrap();
                self.string.push(node.node_char);
            }

            return Some(&self.string);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_trie() {
        let mut trie = Trie::new();
        trie.insert(b"aabb", 0);
        trie.insert(b"aabc", 1);
        trie.insert(b"aacc", 2);

        println!("{:#?}", trie);
        //assert!(false);

        let mut iter = TrieNodeIterator::new(&trie.roots);
        let one = iter.next();
        assert_eq!(b"aabb".as_bstr(), one.unwrap().as_bstr());
        let two = iter.next();
        assert_eq!(b"aabc".as_bstr(), two.unwrap().as_bstr());
        let three = iter.next();
        assert_eq!(b"aacc".as_bstr(), three.unwrap().as_bstr());
        assert_eq!(None, iter.next())
    }
}
