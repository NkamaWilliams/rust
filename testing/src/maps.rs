use std::collections::HashMap;

pub fn hashmap_simple(max: usize) {
    let mut map = HashMap::new();
    let _ = (0..max).map(|num| map.insert(num, num));
}

pub fn hashmap_prealloc(max: usize) {
    let mut map = HashMap::with_capacity(max);
    let _ = (0..max).map(|num| map.insert(num, num));
}
