use std::collections::HashSet;
use std::hash::Hash;

/// Remove duplicate elements from a vector
///
/// Example:
/// ```rust
/// use light_tool::array;
/// assert_eq!(array::duplicate(&vec![1, 2, 2, 3, 4, 4, 5]), vec![1, 2, 3, 4, 5]);
/// ```
pub fn duplicate<T>(elems: &[T]) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut ans = Vec::new();
    let mut seen = HashSet::new();

    for elem in elems {
        if !seen.contains(elem) {
            ans.push(elem.clone());
            seen.insert(elem.clone());
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate() {
        let input1 = vec![1, 2, 2, 3, 4, 4, 5];
        let expected1 = vec![1, 2, 3, 4, 5];
        assert_eq!(duplicate(&input1), expected1);

        let input2 = vec!["apple", "banana", "apple", "orange", "banana"];
        let expected2 = vec!["apple", "banana", "orange"];
        assert_eq!(duplicate(&input2), expected2);
    }
}