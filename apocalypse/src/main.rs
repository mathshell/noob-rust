use std::io::{self, Write}; // On ajoute Write pour que print! s'affiche immédiatement

fn main() {
    // 1. Utilisation de print! avec stdout().flush()
    // En Rust, print! n'affiche rien tant qu'il n'y a pas de retour à la ligne,
    // sauf si on force l'affichage (flush).
    print!("Bonjour quel est votre nom ? ");
    io::stdout().flush().unwrap();  //<------------------------------- io::stdout() On accède au flux de "Sortie Standard" (ton terminal).
    //--------------------------------------------------------------- .flush() On force l'envoi immédiat de tout ce qui est stocké dans le tampon vers l'écran.
    //--------------------------------------------------------------- .unwrap() C'est la gestion d'erreur "brutale".

    let mut nom = String::new(); //<--------------------------------- mut pour dire variable let sert a reserver un emplacement memoir imuable en gro let cree un nom qui pardefaut ne change pas de valeur et associer a  mut ca devien une variable.
    let mut rep = String::new();

    io::stdin()
        .read_line(&mut nom)
        .expect("Echec de la récupération");

    // On nettoie "nom" tout de suite pour l'affichage
    let nom = nom.trim();

    println!("Vous êtes : {} n'est-ce pas ? (oui/non)", nom);

    io::stdin()
        .read_line(&mut rep)
        .expect("Valeur entrée non récupérée");

    // 2. Comparaison avec .trim()
    if rep.trim() != "oui" {
        println!("AH! Oops .. :-(");
    } else {
        println!("Bienvenue à vous {}", nom);
    }
}
