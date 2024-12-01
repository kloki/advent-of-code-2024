use std::collections::HashMap;
pub fn build_counts_hash_map<T>(input: Vec<T>) -> HashMap<T, usize>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    let mut counts: HashMap<T, usize> = HashMap::new();
    for i in input {
        let counter = counts.entry(i).or_insert(0);
        *counter += 1
    }
    counts
}
