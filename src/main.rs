use itertools::Itertools;
use std::io::{self, Write};

fn how_many_correct(my_guess: &[usize], correct_vec: &[usize]) -> usize{
    let mut count = 0;
    for (guess_val, correct_val) in my_guess.iter().zip(correct_vec.iter()) {
       if guess_val == correct_val {
            count +=1;
        }
    }
count
}
// return the number of times the algorithm needed to call howManyCorrect in order to get the
// correct array
// in a situation where we choose 5 from 10 possible values and order matters there are 30240
// possible permutations
fn brute_force(correct_vec: &Vec<usize>, num_possible_values: usize) -> usize{
    let length = correct_vec.len();
    let mut num_guesses = 0;
    let elements: Vec<usize> = (0..num_possible_values).collect();
    for permutation in elements.into_iter().permutations(length) {
        num_guesses+=1;
        if how_many_correct(&permutation, &correct_vec) == length {
            return num_guesses;
        }
    }
    0
}
// this strategy removes the values
fn my_strategy(correct_vec: &Vec<usize>, num_possible_values: usize) -> usize {
    let mut num_guesses = 0;
    let mut possible_guesses: Vec<Vec<usize>> = Vec::new();
    let mut correct_positions: usize;
    let elements: Vec<usize> = (0..num_possible_values).collect();
    let length = correct_vec.len();

    for permutation in elements.into_iter().permutations(length) {
        possible_guesses.push(permutation);
    }

    while possible_guesses.len() > 0 {
        let current_guess = possible_guesses[0].clone();
        num_guesses += 1;
        correct_positions = how_many_correct(&current_guess, &correct_vec);
        if correct_positions == current_guess.len() {
            return num_guesses
        }
        possible_guesses.retain(|v| how_many_correct(&current_guess, &v) == correct_positions);
    }
    return 0
}
fn main() {
    let mut possible_values_str = String::new();
    let mut input = String::new();
    let mut correct_vec: Vec<usize> = Vec::new();
    
    let mut num_possible_values: usize = 10;
    println!("Choose the number of cups to pick from:");
    print!("Number: ");
    io::stdout().flush().unwrap();
    possible_values_str.clear();
    io::stdin().read_line(&mut possible_values_str).expect("Failed to read line");
    let possible_values_str = possible_values_str.trim();
    match possible_values_str.parse::<usize>() {
        Ok(num) => num_possible_values = num,
        Err(_) => println!("please enter a valid usize number")
    }


    println!("Enter numbers to represent the cups, type done when finished:");
    loop {
        print!("Number: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.eq_ignore_ascii_case("done") {
            break;
        }
        match input.parse::<usize>() {
            Ok(num) => correct_vec.push(num),
            Err(_) => println!("please enter a valid usize number or 'done' to finish.")
        }
    }
    println!("Brute Force Approach : {}", brute_force(&correct_vec, num_possible_values));
    println!("Only guess sequences that match previous guesses : {}", my_strategy(&correct_vec, num_possible_values))
}
