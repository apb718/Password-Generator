use std::{io};

use rand::seq::SliceRandom;
extern crate rand;

fn main() {
    println!("Welcome to APB's Password Generator!");
    
    let mut password_settings: PasswordGuidelines = PasswordGuidelines {
         length: 7, 
         contains_symbols: true, 
         contains_uppercase: true 
    };


    
    loop {
        print_menu(PasswordGuidelines { 
            length: password_settings.length, 
            contains_symbols: password_settings.contains_symbols, 
            contains_uppercase: password_settings.contains_uppercase 
        });

        let input: String = get_input();
        match input.as_str() {
            "1" => generate_password(PasswordGuidelines {  // generate password
                length: password_settings.length, 
                contains_symbols: password_settings.contains_symbols, 
                contains_uppercase: password_settings.contains_uppercase 
            }),
            
            
            "2" => password_settings.length = change_length(),// change len
            
            
            "3" => match password_settings.contains_symbols {  // change uppercase toggle
                        true  => password_settings.contains_symbols = false,
                        false => password_settings.contains_symbols = true,
                    },// change symbols toggle
            "4" =>  match password_settings.contains_uppercase {  // change uppercase toggle
                        true  => password_settings.contains_uppercase = false,
                        false => password_settings.contains_uppercase = true,
                    },
            "5" => break,
            _ => println!("huh?"),
        }

    }
    


}

fn change_length() -> u32{
    let mut user_input: String = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    
    println!("What length would you like?");
    stdin.read_line(&mut user_input);

    user_input = String::from(user_input.trim());

    let num: u32 = user_input.parse().unwrap();

    return num;



}

fn generate_password(password_template: PasswordGuidelines) {
    let uppercase_values = vec!["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
    let symbol_values = vec!["!","@","#","$","%","^","&","*","(",")","[","]","{","}"];
    let mut total_nums = password_template.length;

    let mut possible_values = vec!["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];

    if password_template.contains_uppercase {
        //add uppercase
        possible_values.extend(&uppercase_values);
    }

    if password_template.contains_symbols {
        //add symbols
        possible_values.extend(&symbol_values);
    }
    
    let mut password: String = String::new();

    while total_nums != 0 {
        let mut rng = rand::thread_rng();
        
        password += possible_values.choose(&mut rng).unwrap();

        
        total_nums -= 1;
    }
    print_line();
    println!();println!();println!();println!();
    println!("Your Password is \"{}\"", password);
    println!();println!();println!();println!();
    print_line();
    println!();

   
    main();
}


struct PasswordGuidelines {
    length: u32,
    contains_symbols: bool,
    contains_uppercase: bool,
}

fn print_menu(guidelines: PasswordGuidelines) {
    print_line();
    println!("Configure Settings Below");
    
    println!("\t1.) Generate Password with specifications");
    println!("\t2.) \"Change Length\"                  Current Setting: {}", guidelines.length);
    println!("\t3.) \"Change Contains Symbols\"        Current Setting: {}", guidelines.contains_symbols);
    println!("\t4.) \"Change Contains Uppercase\"      Current Setting: {}", guidelines.contains_uppercase);
    println!("");
    println!("\t5.) End Program");
    print_line();
}

fn print_line() {
    println!("_________________________");
}

fn get_input() -> String {

    let mut user_input: String = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input);

    user_input = String::from(user_input.trim());


    match user_input.as_str(){
        "1" => return String::from("1"),
        "2" => return String::from("2"),
        "3" => return String::from("3"),
        "4" => return String::from("4"),
        "5" => return String::from("5"),
        _   => invalid_input_made(user_input),
    }


    
}

fn invalid_input_made(bad_input: String) -> String {
    print_line();
    println!("Invalid Input: \"{}\"", bad_input.trim());
    println!("Try Inputting a Valid Value");
    print_line();

    return get_input();
}