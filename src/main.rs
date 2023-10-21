// Step 1: Define a struct called FilterCondition with a single field of the desired type for filtering.
struct FilterCondition<T> {
    condition: T,
}

// Step 2: Implement a method called is_match on the FilterCondition struct that checks whether an item matches the condition.
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

// Step 3: Define a function called custom_filter to filter elements based on the condition.
fn custom_filter<T: PartialEq + Clone>(collection: Vec<T>, filter: &FilterCondition<T>) -> Vec<T> {
    collection
        .into_iter()
        .filter(|item| filter.is_match(item))
        .collect()
}

fn main() {
    // Step 4: Create a collection (e.g., a vector) with some elements.
    let numbers = vec![1, 2, 2, 3, 4, 4, 5];

    // Initialize a FilterCondition object with the desired value.
    let condition = FilterCondition { condition: 2 };

    // Step 5: Call the custom_filter function with the collection and the FilterCondition object.
    let filtered_numbers = custom_filter(numbers, &condition);

    // Step 6: Print the filtered result to the console.
    println!("Filtered elements: {:?}", filtered_numbers);
}

