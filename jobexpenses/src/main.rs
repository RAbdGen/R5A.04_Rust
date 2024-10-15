use std::io::{self, Read};

fn main() {
    // Lire toute l'entrée
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Erreur de lecture");

    let mut lines = input.lines();

    // Lire le nombre de transactions (on ne l'utilise pas, mais c'est requis)
    let n: usize = lines.next().unwrap().parse().expect("Erreur de parsing du nombre de transactions");

    // Lire la ligne des transactions
    let transactions = lines.next().unwrap();

    // Calculer la somme des dépenses en utilisant des itérateurs
    let total_expenses: i32 = transactions
        .split_whitespace() // Séparer les transactions
        .filter_map(|x| {
            // Essayer de parser chaque transaction en i32
            match x.parse::<i32>() {
                Ok(value) => Some(value),  // Si la conversion réussit, retourner Some(value)
                Err(e) => {
                    eprintln!("Erreur de parsing pour la valeur '{}': {:?}", x, e);
                    None  // En cas d'erreur, retourner None
                }
            }
        })
        .filter(|&x| x < 0) // Garder uniquement les dépenses (négatives)
        .sum(); // Calculer la somme des dépenses

    // Afficher le total des dépenses
    println!("{}", total_expenses);
}
