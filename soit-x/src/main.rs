use core::str;
use std::{collections::hash_map::Values, io, iter};

fn main() {
    println!("Hello, world!");
    println!("La calculatrice 2.0");

    // ====== nombre 1 ======
    let mut input = String::new();
    println!("Entrez un nombre positif : ");
    io::stdin().read_line(&mut input).expect("Erreur lecture"); //<-------------- mécanisme d’écriture par référence mutable.

    let userinput: f64 = input.trim().parse().expect("Nombre invalide");
    input.clear();

    // ====== opérateur ======
    println!("Entrez l'opération (+ - * /) : ");
    io::stdin().read_line(&mut input).expect("Erreur lecture");

    let operateur: char = input.trim().chars().next().expect("Opérateur invalide");
    input.clear();

    // ====== nombre 2 ======
    println!("Entrez le second nombre : ");
    io::stdin().read_line(&mut input).expect("Erreur lecture");

    let nombre2: f32 = input.trim().parse().expect("Nombre invalide");

    // ====== calcul ======
    let resultat = match operateur {
        '+' => nombre1 + nombre2,
        '-' => nombre1 - nombre2,
        '*' => nombre1 * nombre2,
        '/' => {
            if nombre2 == 0.0 {
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

fn cleaninput(data: &str) -> Vec<(f64, Option<char>)> {
    let mut cleaned = Vec::new(); // 1. Corrigé Vect -> Vec
    let mut number = String::new();
    let mut last_op: Option<char> = None; // 2. Corrigé Optionchar -> Option<char>

    for iterator in data.chars() {
        if iterator.is_ascii_digit() || iterator == '.' {
            // Ajout du '.' pour gérer les nombres décimaux
            number.push(iterator);
        } else if matches!(iterator, '+' | '-' | '*' | '/' | '^') {
            if !number.is_empty() {
                let value = number.parse::<f64>().unwrap(); // 3. Corrigé parse::parse et unrap
                cleaned.push((value, last_op)); // 4. Corrigé result -> cleaned
                number.clear();
            }
            last_op = Some(iterator);
        } else if iterator.is_whitespace() {
            continue;
        }
    }

    // 5. IMPORTANT : On n'oublie pas d'ajouter le tout dernier nombre après la boucle !
    if !number.is_empty() {
        let value = number.parse::<f64>().unwrap();
        cleaned.push((value, last_op));
    }

    cleaned // 6. On retourne le vecteur
}

fn diviser(numerateur: f64, denominateur: f64) -> Option<f64> {
    if denominateur == 0.0 {
        None
    } else {
        Some(numerateur / denominateur)
    }
}
fn multiplication(tabvaleur: &[f64]) -> f64 {
    //<------------------------------------ On utilise &[f64] pour acepter m'inporte quelle longeur
    let mut produit = 1.0;
    for valeur in tabvaleur {
        produit *= valeur;
    }

    // tabvaleur.iter().product() //<------------ Fais exactement la meme chose que ma boucle for

    produit //<---------------------------------- La derniere ligne dans une fonction est retourne automatiquement.
}

fn addition(tabvaleur: &[f64]) -> f64 {
    let mut somme = 0.0;
    for valeur in tabvaleur {
        somme += valeur;
    }

    somme
}

fn sousstraction(tabvaleur: &[f64]) -> f64 {
    let mut somme = tabvaleur[0];
    for valeur in tabvaleur[1..] {
        //<------------------------- Boucle sur tous les element auf le premier
        somme -= valeur;
    }

    somme
}

fn puissance(base: f64, exposant: f64) -> f64 {
    base.powf(exposant)
}

fn tertration(base: f64, hauteur: u16) -> f64 {
    if hauteur == 0 {
        return 1.0; //Par convention hauteur de 0 donne 1
    }

    let mut resultat = base;

    for _ in 1..hauteur {
        resultat = base.powf(resultat);

        if resultat.is_finite() {
            println!(
                "L\'histoir que tu enguage la ca va te depasser parceque quant on vous parle \n c'est que hoo je maitrise.\n vous aimez voir avec les yeux."
            );
            break;
        }
    }
    resultat
}

// je veux netoyer l'entre de l'utilisateur avec du regex rager chaque expression de l'utilisateur dans un tuple
// fn clerinput(valeur: &str) -> tableau {}
