fn main() {
    // let numbers = [5, 7];
    let mut numbers = [5, 7, 3, 9, 0, 2];
    bubble_sort(&mut numbers);
    dbg!(numbers);
}

// TODO: Make test suits

fn bubble_sort(number: &mut [i32]) {
    // Since we are sorting in place, we take a mutable reference to the array, sort it, and return nothing.
    let mut is_sorted = false;

    // Since every time we make a full pass, the largest number goes to very end, 2nd largest number goes to the 2nd very end, 3rd largest goes to 3rd very end and so on, it would be efficient to track the number of passes we made so that we can prevent Rust looping over the already sorted items.
    let mut number_of_passes = 0;

    let length_of_array = number.len();

    // If there is no item or only 1 item in the array, technically it's already sorted. So we return early.
    if length_of_array <= 1 {
        // Returning unit type
        return;
    }

    while !is_sorted {
        let mut switch_occurred = false;

        for position in 0..length_of_array - 1 - number_of_passes {
            if number[position] > number[position + 1] {
                number.swap(position, position + 1);
                switch_occurred = true;
            }
        }

        number_of_passes += 1;

        if !switch_occurred {
            is_sorted = true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut numbers: [i32; 0] = [];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, []);
    }

    #[test]
    fn test_duplicates() {
        let mut numbers = [3, 1, 3, 2];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, [1, 2, 3, 3]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut numbers = [5, 4, 3, 2, 1];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, [1, 2, 3, 4, 5]);
    }
}
