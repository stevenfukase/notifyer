// iterators5.rs
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. These counting functions use
// imperative style for loops. Recreate this counting functionality using
// iterators. Only the two iterator methods (count_iterator and
// count_collection_iterator) need to be modified.
// Execute `rustlings hint iterators5` for hints.
//
// Make the code compile and the tests pass.

// I AM NOT DONE

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
}

fn get_vec_map() -> Vec<HashMap<String, Progress>> {
    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }
    use Progress::*;
    let map = get_map();
    let mut other = HashMap::new();
    other.insert(String::from("variables2"), Complete);
    other.insert(String::from("functions2"), Complete);
    other.insert(String::from("if1"), Complete);
    other.insert(String::from("from_into"), None);
    other.insert(String::from("try_from_into"), None);
    vec![map, other]
}
fn main() {
    let collection = get_vec_map();
    assert_eq!(
        6,
        count_collection_iterator(&collection, Progress::Complete)
    );
}
