use std::io::{ stdin, stdout, Write };
use regex::Regex;

pub struct PasswordStrength {
    pub score: u8,
    pub strength: String,
}

fn main() {
    let mut input = String::new();
    print!("Enter the password: ");

    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");

    let password = input.trim();
    let result = check_password(password);

    println!("Password strength: {}", result.strength);
    println!("Password score: {}", result.score);
}

fn check_password(password: &str) -> PasswordStrength {
    let mut score = 0;

    if password.len() >= 8 {
        score += 1;
    }

    let has_uppercase = Regex::new(r"[A-Z]").unwrap().is_match(password);
    if has_uppercase {
        score += 1;
    }

    let has_lowercase = Regex::new(r"[a-z]").unwrap().is_match(password);
    if has_lowercase {
        score += 1;
    }

    let has_number = Regex::new(r"[0-9]").unwrap().is_match(password);
    if has_number {
        score += 1;
    }

    let has_special = Regex::new(r"[!@#$%^&*(),.?\':{}|<>]").unwrap().is_match(password);
    if has_special {
        score += 1;
    }

    let strength = match score {
        0 | 1 => "Very Weak".to_string(),
        2 => "Weak".to_string(),
        3 => "Medium".to_string(),
        4 => "Strong".to_string(),
        5 => "Very Strong".to_string(),
        _ => "Invalid".to_string(),
    };

    PasswordStrength {
        score,
        strength,
    }
}
