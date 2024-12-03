use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

fn compute_distance(l1: &Vec<u32>, l2: &Vec<u32>) -> u32 {
    zip(l1, l2).map(|(x, y)| x.abs_diff(*y)).sum()
}

fn compute_similarity(l1: &Vec<u32>, l2: &Vec<u32>) -> u32 {
    l1.iter()
        .map(|x| x * l2.iter().filter(|y| *y == x).count() as u32)
        .sum()
}

fn main() -> io::Result<()> {
    // Percorso del file di testo
    let path = "./inputs/day01.txt";

    // Apri il file
    let file = File::open(&Path::new(path))?;
    let reader = io::BufReader::new(file);

    // Liste per memorizzare i numeri
    let mut l1 = Vec::<u32>::new();
    let mut l2 = Vec::<u32>::new();

    // Leggi il file riga per riga
    for line in reader.lines() {
        let line = line?;
        // Dividi la riga in due numeri
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error"))
            .collect();
        // Aggiungi i numeri alle rispettive liste
        if numbers.len() == 2 {
            l1.push(numbers[0]);
            l2.push(numbers[1]);
        } else {
            eprintln!("Riga malformata: {}", line);
        }
    }

    l1.sort();
    l2.sort();

    let d = compute_distance(&l1, &l2);
    let s = compute_similarity(&l1, &l2);

    // Stampa le liste
    println!("{d} , {s}");

    Ok(())
}
