use std::cmp;

fn max_substr(input: String) -> i32 {
    let mut max: i32 = 0;
    let mut start: i32 = 0;
    let mut indexes = vec![-1; 127];
    for (i, c) in input.chars().enumerate() {
        if indexes[c as usize] >= start {
            start = indexes[c as usize] + 1;
        }
        indexes[c as usize] = i as i32;
        max = cmp::max(max, (i as i32) - start + 1);
    }
    return max;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "abcbghjb";
        assert_eq!(max_substr(input.to_string()), 5);
    }
}
