// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->
extern crate rand;
extern crate counter;


use counter::Counter;
use rand::distributions::{Distribution, Uniform};

//Function to get max of Ones, Twos, Threes, Fours, Fives, Sixes
fn max_nums(rolls: Vec<u8>) -> u8 {
    let dice_counts = rolls.iter().collect::<Counter<_>>().into_map();
    // println!("{:?}", dice_counts);
    let mut max_score: u8 = 0;
    for key in dice_counts.keys() {
        // println!("{}", **key);
        let  score = (dice_counts[key] as u8)*(**key);
        if score>max_score {
            max_score = score;
        }
    }
    // println!("{}", max_score);
    return max_score;
}

fn max_of_n(rolls: Vec<u8>, n: u8) -> u8 {
    let dice_counts = rolls.iter().collect::<Counter<_>>().into_map();
    // println!("{:?}", dice_counts);
    let mut max_score: u8 = 0;
    for key in dice_counts.keys() {
        if dice_counts[key] as u8 == n{
            let score = (dice_counts[key] as u8)*(**key);
            if score>max_score {
                max_score = score;
            }
        }
    }
    return max_score;
}

fn full_house(rolls: Vec<u8>) -> u8 {
    // let dice_counts = rolls.iter().collect::<Counter<_>>().into_map();
    // println!("{:?}", dice_counts);
    let two: u8 = max_of_n(rolls.clone(),2);
    let three: u8 = max_of_n(rolls.clone(),3);
    let mut max_score: u8 = 0;
    if two != 0 && three != 0{
        max_score = two + three;
    }
    return max_score;
}

fn yatzee(rolls: Vec<u8>) -> u8 {
    // let dice_counts = rolls.iter().collect::<Counter<_>>().into_map();
    // println!("{:?}", dice_counts);
    let yatzee: u8 = max_of_n(rolls.clone(),5);
    // let three: u8 = max_of_n(rolls.clone(),3);
    let mut max_score: u8 = 0;
    if yatzee != 0{
        max_score = 50;
    }
    return max_score;
}

fn chance(rolls: Vec<u8>) -> u8 {
    let mut score: u8 = 0;
    for value in rolls {
        score += value;
    }
    return score;
}

fn max_roll(rolls: Vec<u8>) -> u8 {
    let mut scores: Vec<u8> = Vec::new();
    scores.push(max_nums(rolls.clone()));
    scores.push(max_of_n(rolls.clone(),2));
    scores.push(max_of_n(rolls.clone(),3));
    scores.push(max_of_n(rolls.clone(),4));
    scores.push(yatzee(rolls.clone()));
    scores.push(full_house(rolls.clone()));
    scores.push(chance(rolls.clone()));
    // println!("{:?}", scores);
    let mut max_score = 0;
    for value in scores{
        if value > max_score {
            max_score = value;
        }
    }
    // println!("{}", max_score);
    return max_score;
}

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    let mut rolls: Vec<u8> = Vec::new();


    for _n in 0..5 {
        // println!("{}",n);
        rolls.push(die.sample(&mut rng));
    }
    // let n1: u8 = die.sample(&mut rng);
    println!("roll: {:?}",rolls);
    println!("score: {}",max_roll(rolls));
}
