use std::io;

fn main() {
    println!("Hello, world!");
    println!("La calculatrice 2.0");

    // ====== nombre 1 ======
    let mut input = String::new();
    println!("Entrez un nombre positif : ");
    io::stdin().read_line(&mut input).expect("Erreur lecture");

    let nombre1: i32 = input.trim().parse().expect("Nombre invalide");
    input.clear();

    // ====== opérateur ======
    println!("Entrez l'opération (+ - * /) : ");
    io::stdin().read_line(&mut input).expect("Erreur lecture");

    let operateur: char = input.trim().chars().next().expect("Opérateur invalide");
    input.clear();

    // ====== nombre 2 ======
    println!("Entrez le second nombre : ");
    io::stdin().read_line(&mut input).expect("Erreur lecture");

    let nombre2: i32 = input.trim().parse().expect("Nombre invalide");

    // ====== calcul ======
    let resultat = match operateur {
        '+' => nombre1 + nombre2,
        '-' => nombre1 - nombre2,
        '*' => nombre1 * nombre2,
        '/' => {
            if nombre2 == 0 {
                println!("Erreur : division par zéro");
                return;
            }
            nombre1 / nombre2
        }
        _ => {
            println!("Opérateur invalide");
            return;
        }
    };

    println!("Résultat : {}", resultat);
}
