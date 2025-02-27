use std::{char, collections::HashMap};

struct Node {
    pub is_terminal: bool,
    pub children: HashMap<char, Node>
}

impl Node {
    pub fn new() -> Self{
        Self {
            is_terminal: false,
            children: HashMap::new()
        }
    }
}

struct Trie {
    head: Node
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self {
            head: Node::new()
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut current = &mut self.head;
        for chr in word.chars(){
            current = current.children.entry(chr).or_insert(Node::new());
        }
        current.is_terminal = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut current = &self.head;
        for chr in word.chars() {
            let exists = current.children.contains_key(&chr);
            if exists{
                current = current.children.get(&chr).unwrap();
            } else{
                return false;
            }
        }
        current.is_terminal
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.head;
        for chr in prefix.chars() {
            let exists = current.children.contains_key(&chr);
            if exists{
                current = current.children.get(&chr).unwrap();
            } else{
                return false;
            }
        }
        true
    }
}

struct WordDictionary {
    head: Node
}
impl WordDictionary {

    fn new() -> Self {
        Self {
            head: Node::new()
        }
    }
    
    fn add_word(&mut self, word: String) {
        let chars: Vec<char> = word.chars().collect();
        let mut current = &mut self.head;

        for chr in chars{
            current = current.children.entry(chr).or_insert(Node::new());
        }

        current.is_terminal = false;
    }
    
    fn search(&mut self, word: String) -> bool {
        fn dfs(start: usize, chars: &Vec<char>, current: &mut Node) -> bool {
            if start == chars.len(){
                return current.is_terminal;
            }

            let chr = chars[start];
            if chr == '.' {
                for child in current.children.values_mut(){
                    if dfs(start + 1, chars, child){
                        return true;
                    }
                }
                return false;
            } else if current.children.contains_key(&chr) {
                return dfs(start + 1, chars, current.children.get_mut(&chr).unwrap());
            } else {
                return false;
            }
        }
        dfs(0, &word.chars().collect(), &mut self.head)
    }
}

// /**
//  * Your Trie object will be instantiated and called as such:
//  * let obj = Trie::new();
//  * obj.insert(word);
//  * let ret_2: bool = obj.search(word);
//  * let ret_3: bool = obj.starts_with(prefix);
//  */