use std::fs;

fn main() {
    let total = calculate_file("./src/input.txt");
    println!("Total: {}", total);
}

pub fn calculate_file(file_path: &str) -> u32 {
    let file = read_file(file_path);
    let lines = split_by_newline(&file);

    let mut total = 0;
    for line in lines {
        let line_total = calculate_line(line);
        total += line_total;
    }

    return total;
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Something went wrong reading the file");
}

fn split_by_newline(contents: &String) -> Vec<&str> {
    return contents.split('\n').collect();
}

fn calculate_line(line: &str) -> u32 {
    let mut first_number = None;
    let mut second_number = None;
    for (_, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if first_number == None {
                first_number = Some(c);
            } else {
                second_number = Some(c);
            }
        }
    }

    if second_number == None {
        second_number = first_number;
    }

    let first_number = first_number
        .expect("Line did not contain a number")
        .to_digit(10)
        .expect("Could not parse number");

    let second_number = second_number
        .expect("Line did not contain a number")
        .to_digit(10)
        .expect("Could not parse number");

    return first_number * 10 + second_number;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_file() {
        let total = calculate_file("./src/test_input.txt");
        assert_eq!(total, 142);
    }

    #[test]
    fn test_calculate_line() {
        let total = calculate_line("1abcd2");
        assert_eq!(total, 12);

        let total = calculate_line("1abc");
        assert_eq!(total, 11);

        let total = calculate_line("1ab2cd3");
        assert_eq!(total, 13);

        let total = calculate_line("1234569");
        assert_eq!(total, 19);
    }
}
