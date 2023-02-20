pub fn solve(numbers: &[i32], sum: i32) -> bool {
    if sum == 0 {
        true
    } else if numbers.len() == 0 {
        false
    } else {
        solve(&numbers[1..], sum - numbers[0]) || solve(&numbers[1..], sum)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn case0() {
        let numbers = [1, 2, 4, 7];
        assert!(super::solve(&numbers, 13));
        assert!(!super::solve(&numbers, 15));
    }
}
