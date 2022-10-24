#[derive(Debug)]
struct TrieNode {
    node_char: u8,
    weight: u32,
    final_count: u32,
    children: Vec<TrieNode>,
}

#[derive(Debug)]
struct Trie {
    roots: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Self { roots: Vec::new() }
    }

    pub fn insert(&mut self, s: &[u8]) {
        insert_trie(&mut self.roots, s)
    }
}

fn insert_trie(nodes: &mut Vec<TrieNode>, s: &[u8]) {
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
                insert_trie(&mut node.children, &s[1..]);
            }

            return;
        }
    }

    // Insert new node

    let final_count = if (s.len() == 1) { 1 } else { 0 };

    nodes.push(TrieNode {
        node_char: s[0],
        weight: 1,
        final_count: final_count,
        children: Vec::new(),
    });

    insert_trie(&mut nodes.last_mut().unwrap().children, &s[1..]);
}

struct TrieNodeIterator<'a> {
    roots: &'a [TrieNode],
    indexes: Vec<usize>,
    current: Vec<u8>,
}

impl<'a> TrieNodeIterator<'a> {
    pub fn new(roots: &'a [TrieNode]) -> Self {
        assert!(roots.len() > 0);

        Self {
            roots,
            indexes: vec![0],
        }
    }
}

impl<'a> Iterator for TrieNodeIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_trie() {
        let mut trie = Trie::new();
        trie.insert(b"aabb");
        trie.insert(b"aacc");

        println!("{:#?}", trie);
        assert!(false);
    }
}
