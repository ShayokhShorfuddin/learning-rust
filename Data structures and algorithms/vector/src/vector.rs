// Enum for `append` parameter
pub enum AppendParameter<'a, T> {
    VectorParameter(Vec<T>),
    SliceParameter(&'a [T]),
}

#[derive(Debug)]
pub struct Vector<T> {
    length: usize,
    elements: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Vector {
            length: elements.len(),
            elements,
        }
    }

    // Push
    // Appends an element to the back of a collection.
    pub fn push(&mut self, item: T) {
        self.elements.push(item);

        // Increase the length of our vector
        self.length += 1;
    }

    // Clear
    // Clears the vector, removing all values.
    pub fn clear(&mut self) {
        self.elements.clear();

        // Reset our vector length to 0
        self.length = 0;
    }

    // Len
    // Returns the number of elements in the vector, also referred to as its ‘length’.
    pub fn len(&self) -> usize {
        self.length
    }

    // Is empty
    // Returns true if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// Separate implementation block for Vector<T> with Clone trait bound
impl<T: Clone> Vector<T> {
    // Append
    // Adds elements from either an owned vector or a borrowed slice to this vector.
    // If a slice is provided, elements are cloned.
    pub fn append(&mut self, elements: AppendParameter<T>) {
        match elements {
            AppendParameter::VectorParameter(elements) =>
            // Append the elements to our vector
            {
                for element in elements {
                    self.push(element);
                }
            }

            AppendParameter::SliceParameter(elements) => {
                // Increase the length of our vector too.
                self.length += elements.len();

                // Append the elements to our vector
                self.elements.extend_from_slice(elements);
            }
        }
    }

    // Remove
    // Removes and returns the element at position index within the vector, shifting all elements after it to the left
    // Panics if index is out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        // Panics if index is out of bounds.
        if index >= self.length {
            panic!("Index out of bound.")
        }

        // Targeted element (the element at the given index)
        let targeted_element = self.elements[index].clone();

        // A new vector to store the final result
        let mut new_vector: Vector<T> = Vector {
            length: 0,
            elements: vec![],
        };

        // Get the left side of the vector until the given index and append it to the new vector.
        let left_side = &self.elements[0..index];
        new_vector.append(AppendParameter::SliceParameter(left_side));

        // Add the rest of the elements (the elements on the right side)
        // Index + 1 automatically excludes/ignores the item we want to remove.
        let right_side = &self.elements[index + 1..];
        new_vector.append(AppendParameter::SliceParameter(right_side));

        // Update our vector element to be the new vector.
        self.elements = new_vector.elements;

        // Decrease our vector's length by 1, because we have just removed/ignored an element.
        self.length -= 1;

        targeted_element
    }

    // Insert
    // Inserts an element at position index within the vector, shifting all elements after it to the right.
    // Panics if index > len.
    pub fn insert(&mut self, index: usize, element: T) {
        // Check if index > len. If yes, panic.
        if index > self.len() {
            panic!("Index is greater than the length of vector.")
        }

        // A new vector to store the final result
        let mut new_vector: Vector<T> = Vector {
            length: 0,
            elements: vec![],
        };

        // Get the left side of the vector until the given index and append it to the new vector.
        let left_side = &self.elements[0..index];
        new_vector.append(AppendParameter::SliceParameter(left_side));

        // Now add the given element
        new_vector.push(element);

        // Add the rest of the elements (the elements on the right side)
        let right_side = &self.elements[index..];
        new_vector.append(AppendParameter::SliceParameter(right_side));

        // Update our vector element to be the new vector.
        self.elements = new_vector.elements;

        // Increase our vector's length by 1, because we have just inserted an element.
        self.length += 1;
    }

    // Pop
    // Removes the last element from a vector and returns it, or None if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        // Return None if it is empty.
        if self.is_empty() {
            return None;
        }

        // Remove the last vector
        Some(self.remove(self.length - 1))
    }

    // Truncate
    // Shortens the vector, keeping the first len elements and dropping the rest.
    // If len is greater or equal to the vector’s current length, this has no effect.
    pub fn truncate(&mut self, len: usize) {
        // If len is greater or equal to the vector’s current length, this has no effect.
        if len >= self.length {
            return;
        }

        // A new vector to store the final result
        let mut new_vector: Vector<T> = Vector {
            length: 0,
            elements: vec![],
        };

        // The range of elements which will be safe.
        let safe_elements = &self.elements[0..len];

        // Append them to the new_vector
        new_vector.append(AppendParameter::SliceParameter(safe_elements));

        // Update the vector elements to be the new vector
        self.elements = new_vector.elements;

        // Update the length of our vector
        self.length = len;
    }
}
