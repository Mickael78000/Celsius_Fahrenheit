use std::io;

fn main() {
    println!("Bienvenue dans le convertisseur de températures !");
    println!("Choisissez une option :");
    println!("1. Convertir de Fahrenheit en Celsius");
    println!("2. Convertir de Celsius en Fahrenheit");

    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Erreur lors de la lecture de l'entrée");

    let choix: u32 = match choix.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide.");
          return;
          }
    };

    match choix {
        1 => convertir_fahrenheit_en_celsius(),
        2 => convertir_celsius_en_fahrenheit(),
        _ => println!("Option invalide. Veuillez relancer le programme."),
    }
}

fn convertir_fahrenheit_en_celsius() {
    println!("Entrez la température en Fahrenheit :");

    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Erreur lors de la lecture de l'entrée");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide.");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F correspond à {celsius:.2}°C");
}

fn convertir_celsius_en_fahrenheit() {
    println!("Entrez la température en Celsius :");

    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Erreur lors de la lecture de l'entrée");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide.");
            return;
        }
    };

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{celsius}°C correspond à {fahrenheit:.2}°F");
}