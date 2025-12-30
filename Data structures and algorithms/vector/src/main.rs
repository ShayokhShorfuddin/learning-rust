use crate::vector::{AppendParameter, Vector};

mod vector;

fn main() {
    let mut my_vector = Vector::new(vec![1, 2, 3, 4, 5]);

    my_vector.append(AppendParameter::VectorParameter(vec![6, 7, 8]));
    my_vector.insert(2, 21);
    dbg!(my_vector);
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== new() tests ====================

    #[test]
    fn test_new_with_some_elements() {
        let vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_new_with_none() {
        let vec: Vector<i32> = Vector::new(vec![]);
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_new_with_empty_vec() {
        let vec: Vector<i32> = Vector::new(vec![]);
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    // ==================== push() tests ====================

    #[test]
    fn test_push_single_element() {
        let mut vec: Vector<i32> = Vector::new(vec![]);
        vec.push(42);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_push_multiple_elements() {
        let mut vec: Vector<i32> = Vector::new(vec![]);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_push_to_existing_vector() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2]);
        vec.push(3);
        assert_eq!(vec.len(), 3);
    }

    // ==================== append() tests ====================

    #[test]
    fn test_append_vector_parameter() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.append(AppendParameter::VectorParameter(vec![4, 5, 6]));
        assert_eq!(vec.len(), 6);
    }

    #[test]
    fn test_append_slice_parameter() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        let slice: &[i32] = &[4, 5, 6];
        vec.append(AppendParameter::SliceParameter(slice));
        assert_eq!(vec.len(), 6);
    }

    #[test]
    fn test_append_vector_empty() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.append(AppendParameter::VectorParameter(vec![]));
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_append_slice_empty() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.append(AppendParameter::SliceParameter(&[]));
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_append_to_empty_vector() {
        let mut vec: Vector<i32> = Vector::new(vec![]);
        vec.append(AppendParameter::VectorParameter(vec![1, 2, 3]));
        assert_eq!(vec.len(), 3);
    }

    // ==================== clear() tests ====================

    #[test]
    fn test_clear_non_empty_vector() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3, 4, 5]);
        vec.clear();
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_clear_empty_vector() {
        let mut vec: Vector<i32> = Vector::new(vec![]);
        vec.clear();
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_clear_then_push() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.clear();
        vec.push(42);
        assert_eq!(vec.len(), 1);
    }

    // ==================== insert() tests ====================

    #[test]
    fn test_insert_at_beginning() {
        let mut vec: Vector<i32> = Vector::new(vec![2, 3, 4]);
        vec.insert(0, 1);
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_insert_at_end() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.insert(3, 4);
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_insert_in_middle() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 3, 4]);
        vec.insert(1, 2);
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_insert_into_empty_vector() {
        let mut vec: Vector<i32> = Vector::new(vec![]);
        vec.insert(0, 1);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    #[should_panic(expected = "Index is greater than the length of vector.")]
    fn test_insert_out_of_bounds() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.insert(10, 42);
    }

    // ==================== len() tests ====================

    #[test]
    fn test_len_empty_vector() {
        let vec: Vector<i32> = Vector::new(vec![]);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_len_non_empty_vector() {
        let vec: Vector<i32> = Vector::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn test_len_after_operations() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.push(4);
        assert_eq!(vec.len(), 4);
        vec.pop();
        assert_eq!(vec.len(), 3);
    }

    // ==================== is_empty() tests ====================

    #[test]
    fn test_is_empty_true() {
        let vec: Vector<i32> = Vector::new(vec![]);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let vec: Vector<i32> = Vector::new(vec![1]);
        assert!(!vec.is_empty());
    }

    #[test]
    fn test_is_empty_after_clear() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_is_empty_after_pop_all() {
        let mut vec: Vector<i32> = Vector::new(vec![1]);
        vec.pop();
        assert!(vec.is_empty());
    }

    // ==================== pop() tests ====================

    #[test]
    fn test_pop_returns_last_element() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        let popped = vec.pop();
        assert_eq!(popped, Some(3));
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_pop_single_element() {
        let mut vec: Vector<i32> = Vector::new(vec![42]);
        let popped = vec.pop();
        assert_eq!(popped, Some(42));
        assert!(vec.is_empty());
    }

    #[test]
    fn test_pop_multiple_times() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert!(vec.is_empty());
    }

    // ==================== remove() tests ====================

    #[test]
    fn test_remove_first_element() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        let removed = vec.remove(0);
        assert_eq!(removed, 1);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_remove_last_element() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        let removed = vec.remove(2);
        assert_eq!(removed, 3);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_remove_middle_element() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        let removed = vec.remove(1);
        assert_eq!(removed, 2);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    #[should_panic(expected = "Index out of bound.")]
    fn test_remove_out_of_bounds() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.remove(10);
    }

    #[test]
    #[should_panic(expected = "Index out of bound.")]
    fn test_remove_from_empty_vector() {
        let mut vec: Vector<i32> = Vector::new(vec![]);
        vec.remove(0);
    }

    // ==================== truncate() tests ====================

    #[test]
    fn test_truncate_shortens_vector() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3, 4, 5]);
        vec.truncate(3);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_truncate_to_zero() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.truncate(0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_truncate_greater_than_length() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.truncate(10);
        assert_eq!(vec.len(), 3); // No effect
    }

    #[test]
    fn test_truncate_equal_to_length() {
        let mut vec: Vector<i32> = Vector::new(vec![1, 2, 3]);
        vec.truncate(3);
        assert_eq!(vec.len(), 3); // No effect
    }

    #[test]
    fn test_truncate_empty_vector() {
        let mut vec: Vector<i32> = Vector::new(vec![]);
        vec.truncate(0);
        assert!(vec.is_empty());
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_combined_operations() {
        let mut vec: Vector<i32> = Vector::new(vec![]);

        // Push elements
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);

        // Insert at beginning
        vec.insert(0, 0);
        assert_eq!(vec.len(), 4);

        // Append more elements
        vec.append(AppendParameter::VectorParameter(vec![4, 5]));
        assert_eq!(vec.len(), 6);

        // Remove element
        let removed = vec.remove(2);
        assert_eq!(removed, 2);
        assert_eq!(vec.len(), 5);

        // Pop element
        let popped = vec.pop();
        assert_eq!(popped, Some(5));
        assert_eq!(vec.len(), 4);

        // Truncate
        vec.truncate(2);
        assert_eq!(vec.len(), 2);

        // Clear
        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_with_string_type() {
        let mut vec: Vector<String> =
            Vector::new(vec![String::from("hello"), String::from("world")]);

        vec.push(String::from("!"));
        assert_eq!(vec.len(), 3);

        let popped = vec.pop();
        assert_eq!(popped, Some(String::from("!")));
        assert_eq!(vec.len(), 2);
    }
}
