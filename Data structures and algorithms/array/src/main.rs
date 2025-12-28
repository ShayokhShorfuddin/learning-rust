use crate::array::Array;

mod array;

fn main() {
    let my_array = Array::new(Some(vec![1, 2, 3]));
    let my_empty_array: Array<i32> = Array::new(None);

    dbg!(my_array, my_empty_array);
}
