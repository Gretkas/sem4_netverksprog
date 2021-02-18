mod threadpool;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

fn main() {
    let num_of_threads = 8;

    let thread_pool: ThreadPool = ThreadPool::new(num_of_threads);

    let list: Vec<u128> = (3..1000).collect();

    let prime_list: Arc<Mutex<HashSet<u128>>> = Arc::new(Mutex::new(HashSet::new()));

    let chunks: Vec<Vec<u128>> = list
        .chunks(list.len() / num_of_threads)
        .map(|x| x.to_vec())
        .collect();

    for chunk in chunks {
        println!("{:?}", chunk);
        let result = Arc::clone(&prime_list);
        thread_pool.excecute(move || {
            print_and_compute_primes(&chunk, result);
        })
    }

    for prime in prime_list.lock().unwrap().iter() {
        println!("{}", prime)
    }
}

fn print_and_compute_primes(list: &Vec<u128>, result: Arc<Mutex<HashSet<u128>>>) {
    let primes = compute_primes(&list);

    primes
        .iter()
        .map(|&prime_value| result.lock().unwrap().insert(prime_value));
}

fn compute_primes(list: &Vec<u128>) -> Vec<u128> {
    let mut prime_list: Vec<u128> = Vec::new();
    for number in list {
        if compute_prime(number.clone()) {
            prime_list.push(number.clone());
        }
    }
    prime_list
}

fn compute_prime(prime: u128) -> bool {
    let limit = (prime as f64).sqrt() as u128; // casting number from floats to u128 again

    for i in 2..=limit {
        if prime % i == 0 {
            return false;
        }
    }

    true
}
