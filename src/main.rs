// Troyifare 1.0
// by ryzenstech
use std::io::{self, Write};

fn main() {
    println!("=======================\nTroyifare 1.0 INSIDER BUILD");
    
    loop {
        print!("Scrivi il nome della persona che vuoi Troyifare (scrivi 'exit' per uscire): ");
        io::stdout().flush().expect("Errore nel flush del buffer");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Errore nella lettura dell'input");
        
        let input = input.trim(); // sugarify troia di merda PUTTANA bastarda

        if input == "exit" {
            println!("Chiusura del programma...");
            break;
        } else {
            println!("\n=======================\n{} è una troia\n=======================",input); // here's where the magic happens 😛😛😛😛😛😛😛😛😛
            println!("\nGrazie per aver usato Troyifare 1.0 INSIDER BUILD\nrate 5 stars pls thx bbg<3\n");
        }
    }
}