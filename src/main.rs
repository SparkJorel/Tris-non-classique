use std::io;
use std::time::Instant;

use crate::sorts::{
    bionic_sort, bucket_sort, counting_sort, cub_sort, heap_sort, radix_sort, shell_sort,
};

mod sorts;
mod utils;

fn main() {
    loop {
        println!("\n=== Menu des algorithmes de tri ===");
        println!("1. Heap Sort");
        println!("2. Radix Sort");
        println!("3. Tri par paquets (Bucket Sort)");
        println!("4. Shell Sort");
        println!("5. Counting Sort");
        println!("6. Cub Sort");
        println!("7. Tri Bitonique");
        println!("8. Quitter");
        println!("Entrez votre choix (1-8) :");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix: u32 = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };

        if choix == 8 {
            println!("Au revoir !");
            break;
        }

        let mut arr = vec![64, 34, 25, 12, 22, 11, 90, 100, 1, 45];
        println!("Tableau avant tri : {:?}", arr);

        let start = Instant::now();
        match choix {
            1 => heap_sort(&mut arr),
            2 => radix_sort(&mut arr),
            3 => bucket_sort(&mut arr),
            4 => shell_sort(&mut arr),
            5 => counting_sort(&mut arr),
            6 => cub_sort(&mut arr),
            7 => bionic_sort(&mut arr),
            _ => {
                println!("Choix invalide !");
                continue;
            }
        }
        let duration = start.elapsed();

        println!("Tableau après tri : {:?}", arr);
        println!("Temps d'exécution : {:?}", duration);
    }
}