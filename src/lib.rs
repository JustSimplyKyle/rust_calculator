use std::error;
use std::f64::consts::PI;
use std::fmt::Error;

use statrs::function::factorial::factorial;

fn parenthesis(operator_array: &mut String, num_array: &mut Vec<f64>) {
    loop {
        if !(operator_array.contains("(") || operator_array.contains(")")) {
            break;
        }
        let mut i = 0;
        while i < operator_array.len() {
            if operator_array.chars().nth(i).unwrap() == '(' {
                let mut j = i + 1;
                let mut count = 1;
                while count != 0 {
                    if operator_array.chars().nth(j).unwrap() == '(' {
                        count += 1;
                    } else if operator_array.chars().nth(j).unwrap() == ')' {
                        count -= 1;
                    }
                    j += 1;
                }
                let mut sub_operator_array = operator_array[i + 1..j - 1].to_string();
                //count how many non parenthesis operators
                let mut not_parenthesis_count = i;
                for k in sub_operator_array.chars() {
                    if k != '(' && k != ')' {
                        not_parenthesis_count += 1;
                    }
                }
                let mut sub_num_array = num_array[i..not_parenthesis_count + 1].to_vec();
                calculate(&mut sub_operator_array, &mut sub_num_array);
                num_array.drain(i..not_parenthesis_count + 1);
                operator_array.drain(i..j);
                num_array.insert(i, sub_num_array[0]);
            } else {
                i += 1;
            }
        }
    }
}

pub fn extract_numbers(mut input: String) -> Result<(Vec<f64>, String), Box<dyn error::Error>> {
    let mut num_array: Vec<f64> = Vec::new();
    let mut operator_array = String::new();
    let mut num_string = String::new();
    for (u, i) in input.chars().enumerate() {
        if i.is_numeric() || i == '.' {
            num_string.push(i.to_string().parse().unwrap());
        } else if i.is_ascii() {
            if num_string.len() > 0 {
                num_array.push(num_string.parse::<f64>().unwrap());
                num_string = String::new();
            } else if i == '-' {
                num_string.push(i.to_string().parse().unwrap());
                continue;
            }
            if i == ')' && (input.chars().nth(u + 1).ok_or(Error) == Ok('(') || input.chars().nth(u + 1).expect("Not a character").is_numeric()) {
                operator_array.push(i);
                operator_array.push('*');
            } else if i == 'e' {
                num_array.push(std::f64::consts::E);
            } else if i == 'p' && input.chars().nth(u + 1).ok_or(Error) == Ok('i') {
                num_array.push(PI);
            } else if !(i == 'i' && input.chars().nth(u - 1).ok_or(Error) == Ok('p')) {
                operator_array.push(i);
                if i == '!' {
                    num_array.push(1.0);
                }
            }
        }
    }
    operator_array.pop();
    Ok((num_array, operator_array))
}

pub fn calculate(operator_array: &mut String, num_array: &mut Vec<f64>) {
    let mut found_function = false;
    if operator_array.contains("sin") {
        found_function = true;
    } else if operator_array.contains("cos") {
        found_function = true;
    } else if operator_array.contains("tan") {
        found_function = true;
    } else if operator_array.contains("asin") {
        found_function = true;
    } else if operator_array.contains("acos") {
        found_function = true;
    } else if operator_array.contains("atan") {
        found_function = true;
    } else if operator_array.contains("log") {
        found_function = true;
    } else if operator_array.contains("ln") {
        found_function = true;
    } else if operator_array.contains("sqrt") {
        found_function = true;
    } else if operator_array.contains("cbrt") {
        found_function = true;
    } else if operator_array.contains("abs") {
        found_function = true;
    } else if operator_array.contains("ceil") {
        found_function = true;
    } else if operator_array.contains("floor") {
        found_function = true;
    } else if operator_array.contains("round") {
        found_function = true;
    }
    if found_function == false {
        parenthesis(operator_array, num_array);
    } else {
        evalulate_function(operator_array, num_array);
    }
    loop {
        if operator_array.len() == 0 {
            break;
        }
        let mut i = 0;
        while i < operator_array.len() {
            if operator_array.chars().nth(i).unwrap() == '^' {
                num_array[i] = num_array[i].powf(num_array[i + 1]);
                num_array.remove(i + 1);
                operator_array.remove(i);
            } else if operator_array.chars().nth(i).unwrap() == '!' {
                num_array[i] = factorial(num_array[i] as u64);
                num_array.remove(i + 1);
                operator_array.remove(i);
            } else {
                i += 1;
            }
        }
        i = 0;
        while i < operator_array.len() {
            if operator_array.chars().nth(i).unwrap() == '*' {
                num_array[i] = num_array[i] * num_array[i + 1];
                num_array.remove(i + 1);
                operator_array.remove(i);
            } else if operator_array.chars().nth(i).unwrap() == '/' {
                num_array[i] = num_array[i] / num_array[i + 1];
                num_array.remove(i + 1);
                operator_array.remove(i);
            } else {
                i += 1;
            }
        }
        i = 0;
        while i < operator_array.len() {
            if operator_array.chars().nth(i).unwrap() == '+' {
                num_array[i] = num_array[i] + num_array[i + 1];
                num_array.remove(i + 1);
                operator_array.remove(i);
            } else if operator_array.chars().nth(i).unwrap() == '-' {
                num_array[i] = num_array[i] - num_array[i + 1];
                num_array.remove(i + 1);
                operator_array.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

pub fn contain_from_pos(u: usize, operator_array: &String, word: &str) -> bool {
    if u >= word.len() {
        let mut i = u - word.len();
        let mut j = 0;
        let mut found = word.len();
        while i < operator_array.len() {
            while j < word.len() {
                if operator_array.chars().nth(i).unwrap() == word.chars().nth(j).unwrap() {
                    found -= 1;
                }
                i += 1;
                j += 1;
            }
            return if found == 0 {
                true
            } else {
                false
            }
        }
    }
    false
}

pub fn evalulate_function(operator_array: &mut String, num_array: &mut Vec<f64>) {
    let mut i = 0;
    loop {
        if !(operator_array.contains("(") || operator_array.contains(")")) {
            break;
        }
        let mut i = 0;
        while i < operator_array.len() {
            if operator_array.chars().nth(i).unwrap() == '(' {
                let mut par_minus_size = 0;
                let mut total_word_length=0;
                let mut function_name = String::new();

                if contain_from_pos(i, operator_array, "asin") {
                    par_minus_size=4;
                    function_name = "asin".to_string();
                } else if contain_from_pos(i, operator_array, "acos") {
                    par_minus_size=4;
                    function_name = "acos".to_string();
                } else if contain_from_pos(i, operator_array, "tan") {
                    par_minus_size=3;
                    function_name = "tan".to_string();
                } else if contain_from_pos(i, operator_array, "sin") {
                    par_minus_size=3;
                    function_name = "sin".to_string();
                } else if contain_from_pos(i, operator_array, "cos") {
                    par_minus_size=3;
                    function_name = "cos".to_string();
                } else if contain_from_pos(i, operator_array, "sqrt") {
                    par_minus_size=4;
                    function_name = "sqrt".to_string();
                } else if contain_from_pos(i, operator_array, "atan") {
                    par_minus_size=4;
                    function_name = "atan".to_string();
                } else if contain_from_pos(i, operator_array, "log") {
                    par_minus_size=3;
                    function_name = "log".to_string();
                } else if contain_from_pos(i, operator_array, "ln") {
                    par_minus_size=2;
                    function_name = "ln".to_string();
                } else if contain_from_pos(i, operator_array, "sqrt") {
                    par_minus_size=4;
                    function_name = "sqrt".to_string();
                } else if contain_from_pos(i, operator_array, "abs"){
                    par_minus_size=3;
                    function_name = "abs".to_string();
                } else if contain_from_pos(i, operator_array, "cbrt"){
                    par_minus_size=4;
                    function_name = "cbrt".to_string();
                } else if contain_from_pos(i, operator_array, "ceil"){
                    par_minus_size=4;
                    function_name = "ceil".to_string();
                } else if contain_from_pos(i, operator_array, "floor"){
                    par_minus_size=5;
                    function_name = "floor".to_string();
                } else if contain_from_pos(i, operator_array, "round") {
                    par_minus_size = 5;
                    function_name = "round".to_string();
                }

                for i in operator_array.chars(){
                    if i.is_alphanumeric() {
                        total_word_length+=1;
                    }
                }
                let mut j = i + 1;
                let mut count = 1;
                while count != 0 {
                    if operator_array.chars().nth(j).unwrap() == '(' {
                        count += 1;
                    } else if operator_array.chars().nth(j).unwrap() == ')' {
                        count -= 1;
                    }
                    j += 1;
                }
                let mut sub_operator_array = operator_array[i + 1..j - 1].to_string();
                let mut not_parenthesis_count = i - par_minus_size;
                for k in sub_operator_array.chars() {
                    if k != '(' && k != ')' && !k.is_alphabetic(){
                        not_parenthesis_count += 1;
                    }
                }
                let mut sub_num_array = num_array[i - par_minus_size..not_parenthesis_count + 1].to_vec();
                calculate(&mut sub_operator_array, &mut sub_num_array);
                num_array.drain(i - par_minus_size..not_parenthesis_count + 1);
                operator_array.drain(i - par_minus_size..j);
                num_array.insert(i - par_minus_size, sub_num_array[0]);
                if function_name == "sin" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].sin();
                } else if function_name == "cos" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].sin();
                } else if function_name == "tan" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].tan();
                } else if function_name == "asin" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].asin();
                } else if function_name == "acos" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].acos();
                } else if function_name == "sqrt" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].sqrt();
                } else if function_name == "atan" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].atan();
                } else if function_name == "log" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].ln();
                } else if function_name == "ln" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].ln();
                } else if function_name == "abs" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].abs();
                } else if function_name == "round" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].round();
                } else if function_name == "ceil" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].ceil();
                } else if function_name == "floor" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].floor();
                } else if function_name == "cbrt" {
                    num_array[i - par_minus_size] = num_array[i - par_minus_size].cbrt();
                }
            } else { i += 1; }
        }
    }
}