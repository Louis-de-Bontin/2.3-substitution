use std::io;                                                                               // I'm using the function to ask user input
use std::env;                                                                              // I'm using the function to get command line arguments
use std::process;   

fn main() {
    let args: Vec<String> = env::args().collect();                                         // Get command line arguments and put them in a vector of String
    if args.len() != 2 {
        println!("Please provide the key, and only the key as command line argument.");
        process::exit(1);                                                                  // Exit program with status 1
    }

    let key = to_lower(&args[1]);
    check_key(&key);                                                                       // Check if the key is valid
    let key_ascii = to_ascii(&key);                                                        // Transform the letters in the key by their ascii equivalent

    let plain_text = input("Enter the text to encrypt : ");
    let cypher = encript(&plain_text, &key_ascii);
    println!("Cypher : {}", cypher)
}

fn encript(plain_text: &String, key_ascii: &Vec<u8>) -> String {
    // If the char is lowercase, get his alphabetical index and get the corresponding char in the key
    // If the char is uppercase, same, but uppercase the char
    // If not a letter, keep the char as it is
    let mut cypher = String::from("");
    for cha in plain_text.chars() {
        if is_lower(cha) {   

            let alphabetic_index: u8 = cha as u8 - 97;
            cypher.push(key_ascii[alphabetic_index as usize] as char);

        } else if is_upper(cha) {

            let alphabetic_index: u8 = cha as u8 - 65;
            cypher.push((key_ascii[alphabetic_index as usize] as char).to_ascii_uppercase());
        } else {
            cypher.push(cha);
        }
    }
    cypher
}

fn to_ascii(text: &String) -> Vec<u8> {
    // Get each char of a string and build a vector composed by their ascii index
    let mut ascii = Vec::new();
    for c in text.chars() {
        ascii.push(c as u8);
    }
    ascii
}


fn is_lower(c: char) -> bool {
    c as u8 <= 122 && c as u8 >= 97
}

fn is_upper(c: char) -> bool {
    c as u8 <= 90 && c as u8 >= 65
}

fn check_key(key: &String) {
    // Check if the key is composed by 26 differents char
    if check_chars_duplicate(&key) == true || key.len() != 26{
        println!("Invalid key ! Must contain each 26 letters 1 time.");
        process::exit(1);
    }
}

fn check_chars_duplicate(text: &String) -> bool {
    let mut temporary_text = String::from(text);
    for char in text.chars() {
        temporary_text.remove(0);
        if temporary_text.contains(char) {
            return true
        }
    }
    false
}

fn to_lower(text: &String) -> String {
    let mut text_lower = String::from("");
    for mut char in text.chars() {
        let unicode: u8 = char as u8;
        if unicode <= 90 && unicode >= 65 {                                          // if is uppercase lettre
            char = (unicode + 32) as char;
        }
        text_lower.push(char)
    }
    text_lower
}

// fn to_upper(text: &String) -> String {
//     let mut text_upper = String::from("");
//     for mut char in text.chars() {
//         let unicode: u8 = char as u8;
//         if unicode <= 122 && unicode >= 97 {                                          // if is uppercase lettre
//             char = (unicode + 32) as char;
//         }
//         text_upper.push(char)
//     }
//     text_upper
// }

fn input(message: &str) -> String {
    // This function displays an informative string to the user, and return his input
    println!("{}", message);
    let mut user_input = String::from("");
    io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");
    user_input
}