//NOTE je veux ecrire une fonction en rust qui va prendre en parametre la quamtiter d'elemt a afficher pui les affiche sous forme de poin
// Par exemple je veux pouvoir afficher ce type de message
/*
================================================================================
=============================== MESSAGE ========================================
================================================================================

Donc ma fontion prend en parametre la taille du message puis determine le nombre de d'gale avant et apres pour que le message soit structurer en suite affiche au dessu les  `=` autemps de fois que l'utilisateur l'aurait determine et c'est ce nombre qui sera la base pour determiner a la ligne suivant le nombre de `=` avant et apres le message.

*/

pub fn sweetprint(message: &str, nbregl: i32) -> String {
    //<---------- Declaration de ma fonction ici le mot pub rend la fonction public et donc accecible hor du fichier
    let mut valoutput = String::new(); //<------------------- On initialise la chaine avec une valeur vide.
    let length_message = message.chars().count(); //<------- Orecupere la taille du mot passe en parametre.
    for _ in 0..nbregl {
        valoutput.push('='); //<----------------------------- Ajoute a la variable valoutput la caracter `=` a chaque iteration.
    }

    let espace_dispo = if nbregl as usize > length_message {
        (nbregl as usize - length_message) / 2
    } else {
        1 // Sécurité si le message est plus long que la ligne
    };
    let valside = "=".repeat(espace_dispo);
    format!(
        "{}\n{} {} {}\n{}",
        valoutput, valside, message, valside, valoutput
    )

    // let valoutput = "=".repeat(nbregl as usize); //------ Ceci a la meme fonction que la boucle for precedente.
}
