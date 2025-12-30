// Collections in Rust
// Learn about Vec, HashMap, HashSet, and more!

use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};

fn main() {
    println!("=== Collections in Rust ===\n");

    // 1. VECTORS - Dynamic arrays
    println!("--- Vectors (Vec<T>) ---");

    // Creating vectors
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Vector: {:?}", numbers);

    // Using vec! macro
    let mut fruits = vec!["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);

    // Accessing elements
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {}", fruits.get(1).unwrap());

    // Safe access with get()
    match fruits.get(10) {
        Some(fruit) => println!("Fruit at index 10: {}", fruit),
        None => println!("No fruit at index 10"),
    }

    // Modifying vectors
    fruits.push("date");
    println!("After push: {:?}", fruits);

    let last = fruits.pop();
    println!("Popped: {:?}, Remaining: {:?}", last, fruits);

    // Iterating
    print!("Fruits: ");
    for fruit in &fruits {
        print!("{} ", fruit);
    }
    println!("\n");

    // 2. HASHMAPS - Key-value pairs
    println!("--- HashMap<K, V> ---");

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);

    println!("Scores: {:?}", scores);

    // Accessing values
    let alice_score = scores.get("Alice");
    match alice_score {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice not found"),
    }

    // Updating values
    scores.insert(String::from("Alice"), 98); // Overwrites
    println!("After update: {:?}", scores);

    // Only insert if key doesn't exist
    scores.entry(String::from("David")).or_insert(85);
    scores.entry(String::from("Alice")).or_insert(100); // Won't overwrite
    println!("After entry: {:?}", scores);

    // Update based on old value
    let alice = scores.entry(String::from("Alice")).or_insert(0);
    *alice += 2; // Add 2 to Alice's score
    println!("Alice's new score: {}", scores.get("Alice").unwrap());

    // Iterating
    println!("All scores:");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }
    println!();

    // 3. HASHSETS - Unique values
    println!("--- HashSet<T> ---");

    let mut visited_pages: HashSet<String> = HashSet::new();
    visited_pages.insert(String::from("home"));
    visited_pages.insert(String::from("about"));
    visited_pages.insert(String::from("contact"));
    visited_pages.insert(String::from("home")); // Duplicate ignored

    println!("Visited pages: {:?}", visited_pages);
    println!("Number of unique pages: {}", visited_pages.len());

    // Check if value exists
    if visited_pages.contains("home") {
        println!("Already visited home page");
    }

    // Set operations
    let set1: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();

    let union: HashSet<_> = set1.union(&set2).collect();
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    let difference: HashSet<_> = set1.difference(&set2).collect();

    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);
    println!("Union: {:?}", union);
    println!("Intersection: {:?}", intersection);
    println!("Difference (1-2): {:?}", difference);
    println!();

    // 4. VECTOR OPERATIONS
    println!("--- Advanced Vector Operations ---");

    let mut nums = vec![5, 2, 8, 1, 9, 3];
    println!("Original: {:?}", nums);

    // Sorting
    nums.sort();
    println!("Sorted: {:?}", nums);

    // Reverse
    nums.reverse();
    println!("Reversed: {:?}", nums);

    // Filter with iterators
    let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("Even numbers: {:?}", evens);

    // Map
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Reduce (fold)
    let sum: i32 = nums.iter().sum();
    println!("Sum: {}", sum);

    // Find
    let first_even = nums.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);

    // Any / All
    let has_large = nums.iter().any(|&x| x > 5);
    let all_positive = nums.iter().all(|&x| x > 0);
    println!("Has number > 5? {}", has_large);
    println!("All positive? {}", all_positive);
    println!();

    // 5. BTREEMAP - Sorted key-value pairs
    println!("--- BTreeMap<K, V> (Sorted) ---");

    let mut sorted_scores: BTreeMap<String, i32> = BTreeMap::new();
    sorted_scores.insert(String::from("Charlie"), 92);
    sorted_scores.insert(String::from("Alice"), 95);
    sorted_scores.insert(String::from("Bob"), 87);

    println!("Scores (alphabetically sorted):");
    for (name, score) in &sorted_scores {
        println!("  {}: {}", name, score);
    }
    println!();

    // 6. VECDEQUE - Double-ended queue
    println!("--- VecDeque<T> ---");

    let mut queue: VecDeque<i32> = VecDeque::new();

    // Add to back
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    // Add to front
    queue.push_front(0);

    println!("Queue: {:?}", queue);

    // Remove from front
    let front = queue.pop_front();
    println!("Popped front: {:?}, Remaining: {:?}", front, queue);

    // Remove from back
    let back = queue.pop_back();
    println!("Popped back: {:?}, Remaining: {:?}", back, queue);
    println!();

    // 7. WORD FREQUENCY COUNTER (Practical Example)
    println!("--- Word Frequency Counter ---");

    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_count: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word frequencies:");
    for (word, count) in &word_count {
        println!("  '{}': {}", word, count);
    }
    println!();

    // 8. GROUPING DATA (Practical Example)
    println!("--- Grouping Students by Grade ---");

    let students = vec![
        ("Alice", "A"),
        ("Bob", "B"),
        ("Charlie", "A"),
        ("David", "C"),
        ("Eve", "B"),
        ("Frank", "A"),
    ];

    let mut grade_groups: HashMap<&str, Vec<&str>> = HashMap::new();

    for (student, grade) in students {
        grade_groups.entry(grade).or_insert(Vec::new()).push(student);
    }

    for (grade, students) in &grade_groups {
        println!("Grade {}: {:?}", grade, students);
    }
    println!();

    // 9. CHAINING ITERATOR METHODS
    println!("--- Iterator Chains ---");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)     // Keep even numbers
        .map(|&x| x * x)                // Square them
        .take(3)                        // Take first 3
        .sum();                         // Sum them up

    println!("Sum of squares of first 3 even numbers: {}", result);
    println!("Process: filter evens → square → take 3 → sum");
    println!("Numbers: [1,2,3,4,5,6,7,8,9,10] → [2,4,6] → [4,16,36] → 56");
    println!();

    // 10. COLLECTING INTO DIFFERENT COLLECTIONS
    println!("--- Collecting Results ---");

    let nums = vec![1, 2, 3, 2, 1, 4, 3];

    // To Vec
    let as_vec: Vec<i32> = nums.iter().copied().collect();
    println!("As Vec: {:?}", as_vec);

    // To HashSet (removes duplicates)
    let as_set: HashSet<i32> = nums.iter().copied().collect();
    println!("As HashSet (unique): {:?}", as_set);

    // To String
    let words = vec!["Hello", "Rust", "World"];
    let sentence = words.join(" ");
    println!("As String: {}", sentence);

    println!("\n🎉 You've mastered Rust collections!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_vec_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_hashmap_insert() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
        assert_eq!(map.get("missing"), None);
    }

    #[test]
    fn test_hashset_unique() {
        let mut set = HashSet::new();
        assert_eq!(set.insert(1), true);  // First insert returns true
        assert_eq!(set.insert(1), false); // Duplicate returns false
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_vec_filter_map() {
        let nums = vec![1, 2, 3, 4, 5];
        let evens: Vec<i32> = nums.iter()
            .filter(|&&x| x % 2 == 0)
            .copied()
            .collect();
        assert_eq!(evens, vec![2, 4]);
    }

    #[test]
    fn test_word_frequency() {
        let text = "hello world hello";
        let mut counts = HashMap::new();

        for word in text.split_whitespace() {
            *counts.entry(word).or_insert(0) += 1;
        }

        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
    }
}
