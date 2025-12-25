fn main() {
    let numbers = vec![4, 7, 2, 5, 12, 1];
    dbg!(stalin_sort(&numbers));
}

fn stalin_sort(numbers: &[i32]) -> Vec<i32> {
    // Return empty or 1 item arrays
    if numbers.len() <= 1 {
        return vec![];
    }

    let mut survivors: Vec<i32> = vec![];

    // Consider the leftmost as "standard" and judge the next ones based on the standard.
    let mut standard = numbers[0];
    survivors.push(numbers[0]);

    for position in 1..numbers.len() {
        // If a superior leader (number) is found.
        if standard <= numbers[position] {
            standard = numbers[position];
            survivors.push(numbers[position]);
        }
    }

    survivors
}

#[cfg(test)]
mod tests {
    use super::*;

    // Perfect society. All numbers are already sorted.
    #[test]
    fn test_perfect_society() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(numbers, stalin_sort(&numbers));
    }

    // The Great Purge. All numbers are in complete reverse order
    #[test]
    fn test_great_purge() {
        let numbers = vec![10, 9, 8, 7, 6];
        assert_eq!(vec![10], stalin_sort(&numbers));
    }

    // Chaotic society. Random order of numbers.
    #[test]
    fn test_chaotic_society() {
        let numbers = vec![3, 1, 4, 2, 5];
        assert_eq!(vec![3, 4, 5], stalin_sort(&numbers));
    }

    // Empty nation. There are no numbers to sort.
    #[test]
    fn test_empty_nation() {
        let numbers: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, stalin_sort(&numbers));
    }

    // Single leader. Only one number in the array.
    #[test]
    fn test_single_leader() {
        let numbers = vec![42];
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, stalin_sort(&numbers));
    }

    // Equal comrades. All numbers are the same.
    #[test]
    fn test_equal_comrades() {
        let numbers = vec![5, 5, 5, 5];
        assert_eq!(numbers, stalin_sort(&numbers));
    }
}
