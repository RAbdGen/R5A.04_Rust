use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Erreur de lecture");

    let mut lines = input.lines();

    // Ignorer la première ligne (le nombre de températures)
    lines.next();

    // La seconde ligne contient les températures
    let temperatures = lines.next().unwrap();

    // Utiliser des itérateurs pour filtrer les températures < 0 et les compter
    let count = temperatures
        .split_whitespace()  // Sépare la ligne en mots (températures)
        .map(|temp| temp.parse::<i32>().expect("Erreur de parsing"))  // Convertit chaque température en entier
        .filter(|&temp| temp < 0)  // Filtre celles qui sont strictement inférieures à zéro
        .count();  // Compte les températures filtrées

    // Afficher le résultat
    println!("{}", count);
}
