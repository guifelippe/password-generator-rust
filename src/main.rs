use rand::Rng;
use std::io;

fn main() {
    println!("Bem-vindo ao Gerador de Senhas!");

    let password_length = get_password_length();

    let include_special_chars = get_include_special_chars();

    let password = generate_password(password_length, include_special_chars);

    println!("Sua senha gerada: {}", password);
}

fn get_password_length() -> usize {
    loop {
        println!("Digite o tamanho da senha desejada:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

        match input.trim().parse() {
            Ok(length) => {
                return length;
            }
            Err(_) => {
                println!("Por favor, digite um número válido.");
            }
        }
    }
}

fn get_include_special_chars() -> bool {
    loop {
        println!("Incluir caracteres especiais (por exemplo, !@#) na senha? (S/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

        match input.trim().to_lowercase().as_str() {
            "s" => {
                return true;
            }
            "n" => {
                return false;
            }
            _ => {
                println!("Por favor, digite 's' para Sim ou 'n' para Não.");
            }
        }
    }
}

fn generate_password(length: usize, include_special_chars: bool) -> String {
    let charset = if include_special_chars {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+=[]{}|;:,.<>?"
    } else {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
    };

    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    password
}