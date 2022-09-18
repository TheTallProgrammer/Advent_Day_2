// Libraries
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn instantiator(){ // Head function
    let (mut _depth, mut _h_pos) = (0,0);
    let (directions, shifts) = read_file(); // Calls read_file function and returns the input that would caught from the data.txt
    determiner(&directions, &shifts, &mut _depth, &mut _h_pos); // uses pass by reference for each argument to avoid returning 
    calclations(_depth, _h_pos);
} // End of instantiator

pub fn read_file() -> (Vec<String>, Vec<i32>){ // Returns directions, _shifts vectors
    let file = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(file);
    let (mut directions, mut _shifts) = (Vec::new(),vec![0i32]); // Vectors that will hold either directions or shifts
    for (_index,line) in reader.lines().enumerate() { // loop that iterates through the data file line by line
        let line = line.unwrap(); // line variable is each line in data.txt
        for substr in line.split_whitespace(){ // Puts each word before a whitespace inside of the data file into its own line
            if substr == "forward" || substr == "down" || substr == "up" {directions.push(substr.to_string());} // Converts &str to string
            else {_shifts.push(FromStr::from_str(substr).unwrap());} // Converts from &str to i32
        } // End of inside loop
    } // End of outside loop
    return (directions, _shifts);
} // End of read_file

pub fn determiner(directions: &Vec<String>, shifts: &Vec<i32>, _depth: &mut i32, _h_pos: &mut i32 ){ // Use pass by reference to not have to return anything, just updates the vectors if anything needs to change  
    let mut _shifts_i = 1; // Have to set to 1 because the first value is 0 for some reason
    for i in 0..directions.len(){
        let _item = &directions[i].to_string();
        if directions[i] == "forward"{(*_h_pos) += shifts[_shifts_i]} // Dereference the _depth with*
        if directions[i] == "up"{(*_depth) -= shifts[_shifts_i];} 
        if directions[i] == "down"{(*_depth) += shifts[_shifts_i];}
        _shifts_i += 1;
    } // End of loop
} // End of determiner 

pub fn calclations(_depth: i32, _h_pos: i32){ //multiply the depth * hPos to get final position
    println!("Depth: {}, hPos: {}, Total Shift: {} ", _depth, _h_pos, (_depth * _h_pos));
} // End of calculations
