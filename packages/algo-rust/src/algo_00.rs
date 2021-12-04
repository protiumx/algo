use std::collections::HashMap;

fn most_frequent(arr: &[i32]) -> i32 {
    let mut ret = 0;
    let mut max_count = -1;
    let mut counter = HashMap::new();
    for n in arr {
        let counter = counter.entry(n).or_insert(0);
        *counter += 1;
        if *counter > max_count {
            max_count = *counter;
            ret = *n;
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_most_frequent() {
        assert_eq!(most_frequent(&[1, 2, 1, 2, 1]), 1);
    }
}
