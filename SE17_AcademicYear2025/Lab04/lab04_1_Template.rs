fn main() {
    // ## 1.1) Create and Display ##
    println!("--- 1.1) Create and Display ---");
    // Creates a vector with the specified test scores.
    let scores = vec![85, 92, 78, 96, 88, 73, 91, 84];

    // Prints all the scores using the debug trait.
    println!("All scores: {:?}", scores);

    // Prints the number of elements (length) of the vector.
    println!("Number of scores: {}", scores.len());

    println!("\n"); // Add a newline for better formatting.

    // ## 1.2) Find Information ##
    println!("--- 1.2) Find Information ---");
    // Find and print the highest score.
    // .iter().max() returns an Option, so we use .unwrap() as we know the vector isn't empty.
    if let Some(highest) = scores.iter().max() {
        println!("Highest score: {}", highest);
    }

    // Find and print the lowest score.
    if let Some(lowest) = scores.iter().min() {
        println!("Lowest score: {}", lowest);
    }

    // Check if a score of 90 exists in the list.
    let target_score = 90;
    if scores.contains(&target_score) {
        println!("Found a score of {}.", target_score);
    } else {
        println!("Did not find a score of {}.", target_score);
    }

    // Print the first and the last scores.
    if let (Some(first), Some(last)) = (scores.first(), scores.last()) {
        println!("First score: {}, Last score: {}", first, last);
    }

    println!("\n");

    // ## 1.3) Modify the Vector ##
    println!("--- 1.3) Modify the Vector ---");
    // Re-create the vector as mutable to perform modifications.
    let mut modifiable_scores = vec![85, 92, 78, 96, 88, 73, 91, 84];
    println!("Original vector: {:?}", modifiable_scores);

    // Add a new score of 87 to the end.
    modifiable_scores.push(87);
    println!("After adding 87: {:?}", modifiable_scores);

    // Remove the last score.
    modifiable_scores.pop();
    println!("After removing the last score: {:?}", modifiable_scores);

    // Sort all scores from lowest to highest (in-place).
    modifiable_scores.sort();
    println!("Sorted vector: {:?}", modifiable_scores);

    println!("\n");

    // ## 1.4) Filter and Count ##
    println!("--- 1.4) Filter and Count ---");
    // Re-create the vector as mutable for filtering operations.
    let mut filterable_scores = vec![85, 92, 78, 96, 88, 73, 91, 84];
    println!("Original vector: {:?}", filterable_scores);

    // Count how many scores are 85 or higher.
    let high_scores_count = filterable_scores.iter().filter(|&&s| s >= 85).count();
    println!("Number of scores 85 or higher: {}", high_scores_count);

    // Create a new vector containing only scores above 80.
    let scores_above_80: Vec<i32> = filterable_scores
        .iter()
        .filter(|&&s| s > 80)
        .cloned() // Create owned values from references.
        .collect(); // Collect the items into a new vector.
    println!("New vector with scores > 80: {:?}", scores_above_80);

    // Remove all scores below 75 from the original vector.
    // The .retain() method keeps only the elements that satisfy the predicate.
    filterable_scores.retain(|&s| s >= 75);
    println!("Vector after removing scores below 75: {:?}", filterable_scores);
}