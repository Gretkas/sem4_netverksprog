# Oving 3 - Sigmund Granaas

Finn alle primtall mellom to gitte tall ved hjelp av et gitt antall
tråder.
Skriv til slutt ut en sortert liste av alle primtall som er funnet
Pass på at de ulike trådene får omtrent like mye arbeid
Valgfritt programmeringsspråk, men bruk gjerne et
programmeringsspråk dere ikke har prøvd før (ikke JavaScript eller
Python), eller for de som vil ha litt extra utfordring: Rust eller C++.
De som vil bruke Rust eller C++ kan ta utgangspunkt i
ntnu-tdat2004/threads

The program is currently configured to look for primes in an set interval `main.rs` The default number of threads used is 15, but should be changed to match the current programs number of logical CPU cores - 1. `let num_of_threads = 15;`

To change the defualt range, which is `let prime_range = prime_range { upper: 50000000, lower: 10, };` you just have to change the numbers.

The work is ranges are split ten times the amount of cores specified, this is to ensure that one cpu core won't be out of work if it finishes early, although it might cause some overhead on small ranges.

Calculating the work done is not accurate, but you get the general idea. from looking at it. View the code for more details of implementation.

Run the project with: `cargo run`
