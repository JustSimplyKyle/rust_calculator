use rust_calculator::{calculate, extract_numbers};

fn main() {
    //get input line
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let (mut num_array, mut operator_array) = extract_numbers(input).expect("Error in extraction of data");
    calculate(&mut operator_array, &mut num_array);
    //print result
    println!("{}", num_array[0]);
}
