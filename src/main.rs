use std::io; // Entrée utilisateur
use rand::Rng; // Nombres aléatoires

/// Les variantes de cet enum représentent toutes les valeurs
/// discrètes possibles des comparaisons numériques entre le
/// nombre inséré par l'utilisateur et le nombre aléatoire
/// généré
enum GuessResult {
    Greater,
    Less,
    Equal,
}

/// Compare la valeur proposée par l'utilisateur (guessed_num) 
/// à celle générée aléatoirement par le programme (rnd_num) 
/// et renvoie une variante de l'enum GuessResult selon le résultat 
/// de cette comparaison.
/// 
/// # Méthode cmp
/// La méthode cmp permet de comparer deux entiers et renvoie
/// un énumérateur de type Ordering qui peut être soit : 
/// Less, Greater ou Equal
/// 
/// # Exemple :
/// let num1 = 15;
/// let num2 = 22;
/// let compare = num1.cmp(&num2)
/// 'let compare' contient maintenant : std::cmp::Ordering::Less
fn equality_match(guessed_num: i32, rnd_num: i32) -> GuessResult {
    match guessed_num.cmp(&rnd_num) {
        std::cmp::Ordering::Less => GuessResult::Greater,
        std::cmp::Ordering::Greater => GuessResult::Less,
        std::cmp::Ordering::Equal => GuessResult::Equal,
    }
}

fn main() {
    // Nombre de tentatives
    let mut attempts: i32 = 0;

    // Génération du nombre aléatoire
    // Un nombre entier compris entre 1 (compris) et 100 (compris)
    let rnd_num: i32 = rand::thread_rng().gen_range(1..=100);

    // La boucle continue tant que true
    let mut in_guess: bool = true;

    // Création d'un String vide pour stocker l'entrée utilisateur
    let mut input: String = String::new();

    // Début de la boucle
    while in_guess {
        input.clear(); // L'entrée est effacée
        println!("Essayez de deviner le nombre entre 0 et 100 :");

        // Lecture de l'entrée utilisateur
        // read_line() : toute l'entrée, jusqu'à la pression de 
        // la touche Entrée, sera enregistrée dans 'input'
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        // Conversion de l'entrée en nombre
        // trim() : supprime les espaces
        // parse() : convertie selon le type de la variable assignée
        // expect() : Message d'erreur en cas d'erreur
        let num_guessed: i32 = input.trim()
            .parse()
            .expect("Please type a valid number");
        
        // Soumet le nombre entrée par l'utilisateur et le nombre
        // aléatoire généré à la fonction afin de les comparer.
        // La fonction renvoi une variante de l'enum GuessResult
        let guess:GuessResult = equality_match(num_guessed, rnd_num);

        // Instructions à executer selon le résultat de la 
        // comapraison
        match guess {
            // C'est plus petit
            GuessResult::Less => {
                println!("C'est moins !");
                attempts += 1;
            }

            // C'est plus grand 
            GuessResult::Greater => {
                println!("C'est plus !");
                attempts += 1;
            }

            // C'est égal
            // La variable flag 'in_guess' passe à false afin de
            // mettre fin à la boucle while
            GuessResult::Equal => {
                println!("Bien joué !");
                attempts += 1;
                in_guess = false;
            },
        } // match guess
        } // While

    println!("Vous avez réussi en {} coups", attempts);
}