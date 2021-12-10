use std::cmp;

fn max_substr(input: String) -> i32 {
    let mut max = 0;
    let mut start = 0;
    let mut indexes = arr![-1; 127];
    for (i, c) in input.chars().enumerate() {
        if indexes[c] >= start {
            start = indexes[c] + 1;
        }
        indexes[c] = i;
        max = cmp::max(max, i - start + 1);
    }
    return max;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "abcbghjb";
        assert_eq!(max_substr(input), 5);
    }
}
