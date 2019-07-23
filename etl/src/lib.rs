use std::collections::btree_map::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map = BTreeMap::new();

    for (k, v) in h {
        for c in v {
            map.insert(c.to_ascii_lowercase(), *k);
        }
    }

    map
}
