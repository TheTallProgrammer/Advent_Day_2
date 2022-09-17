// Libraries
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn instantiator(){ // Head function
    let (mut _depth, mut _h_pos) = (0,0);
    let (directions, shifts) = read_file(); // Calls read_file and returns the input that would caught from the data.txt

} // End of instantiator

pub fn read_file() -> (Vec<String>, Vec<i32>){
    let file = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut directions:Vec<String> = Vec::new(); // Vector to hold directions from data file
    let mut shifts:Vec<i32> = vec![0i32]; // Vector to contain the shifts of each direction from data file
    for (_index,line) in reader.lines().enumerate() { // loop that iterates through the data file line by line
        let line = line.unwrap(); // line is each line in data.txt
        for substr in line.split_whitespace(){ // Puts each word in data file into its own line
            if substr == "forward" || substr == "down" || substr == "up" { // If the input data is a direction, add it to the directions vector
                directions.push(substr.to_string()); // Converts &str to string
            } else {
                let temp_str:i32 = FromStr::from_str(substr).unwrap(); // Converts from &str to i32
                shifts.push(temp_str);
            }
        } // End of inside loop
    } // End of outside loop
    return (directions, shifts);
} // End of read_file

pub fn determiner(directions: &Vec<String>, shifts: &Vec<i32>){ // Use pass by reference to not have to return anything, just updates the vectors if anything needs to change  

} // End of determiner 

pub fn forward(){ // Moves forward changing the hPos by input
    todo!();
} // End of forward

pub fn up(){ // Moves up changing the depth by input
    todo!();
} // End of up

pub fn down(){ // Moves down changing the depth by input
    todo!();
} // End of down

pub fn calclations(){ //multiply the depth * hPos to get final position
    todo!();
} // End of calculations