#[derive(Debug)]
pub struct Array<T> {
    length: usize,
    pub elements: Vec<T>,
}

impl<T> Array<T> {
    pub fn new(elements: Option<Vec<T>>) -> Self {
        match elements {
            Some(elements) => Array {
                length: elements.len(),
                elements,
            },

            None => Array {
                length: 0,
                elements: vec![],
            },
        }
    }
}
