/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in v {
        new_vec.push(i + n);
    }
    new_vec
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for i in v {
        *i += n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut index: usize = 0;
    for i in 0..v.len() {
        if !visited.contains(&v[i]) {
            visited.insert(v[i]);
            v[index] = v[i];
            index += 1;
        }
    }
    for _ in 0..v.len()-index {
        v.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
