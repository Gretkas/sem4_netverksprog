mod threadpool;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

struct PrimeRange {
    upper: u128,
    lower: u128,
}

fn main() {
    //Creating a pool of * amount of threads
    let num_of_threads = 12;
    let thread_pool: ThreadPool = ThreadPool::new(num_of_threads as usize);

    // Creating the range of number which should be computed
    let prime_range = PrimeRange {
        upper: 50000000,
        lower: 10,
    };

    // A vector for holding the ranges once they have been split up
    let mut range_vec: Vec<PrimeRange> = Vec::new();

    // Setting the amount og number per range, this takes into account that i wil create ten times more chunks of twork than there are cores.
    let numbers_per_thread = (prime_range.upper - prime_range.lower) / (num_of_threads * 10);

    // Splitting the ranges into chunks ten times the amount of cores, this will ensure threads won't sit still of they finish early
    for x in 0..num_of_threads * 10 {
        let constructed_prime_range = PrimeRange {
            upper: (x + 1) * numbers_per_thread + prime_range.lower, // calculating upper range
            lower: (x + 1) * numbers_per_thread - numbers_per_thread + prime_range.lower, //calculating lower range
        };
        range_vec.push(constructed_prime_range);
    }

    //Creating a Arch mutex which holds the list of primes, this is shared across the threads
    // Creating a shared Mutex in an Arc Is not the most efficient way of writing the result. But it is a good way to learn how to share memory safely
    // To optimize the program, I would make the threads create their own separate lists, which would already be sorted instead of waiting for the mutex lock every time they found a new prime.
    // This list would then be joined at the end of the operation, where every chunk would already be sorted and could be placed in order.
    let prime_list: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(Vec::new()));

    let mut iter = 0; // holding the number of iterations
    for chunk in range_vec {
        iter += 1;
        let result = Arc::clone(&prime_list);
        //Moving each chunk of work over to the thread pool
        thread_pool.excecute(move || {
            compute_primes(chunk, result, iter, num_of_threads);
        });
    }

    //dropping the pool, which will ensure all work is finished before continuing
    drop(thread_pool);

    // Creating a vec for holding and sorting the results.
    let mut result_set: Vec<u128> = Vec::new();

    // Transferring the results over to a vec. There are probably ways to sort vec wrapped in a Mutex, but I had way to much trouble with it.
    for prime in prime_list.lock().unwrap().iter() {
        result_set.push(prime.clone());
    }

    //sorting and printing the finished list
    result_set.sort();
    for prime in result_set {
        println!("{}", prime);
    }
}

fn compute_primes(
    prime_range: PrimeRange,
    result: Arc<Mutex<Vec<u128>>>,
    iter: usize,
    num_of_threads: u128,
) {
    for number in prime_range.lower..prime_range.upper {
        if compute_prime(number.clone()) {
            // This will write to the shared vector every time it finds a new prime number, it would be more efficient to write it to a list before adding, but it works suprisingly well.
            //This probably causes a lot of waiting for locks
            result.lock().unwrap().push(number.clone());
        }
    }
    //Extremely stupid way of writing progress. It assumes that every chunk is processed in order, which is not always the case.
    // I should just make a shared Arc to keep track of how many chunks has ben processed and calculate the progress from that
    println!(
        "{} % done",
        (((iter as f32 / ((num_of_threads * 10) as f32) as f32) * 100 as f32) as usize)
    );
}

//Simple function for checking wheter a number is a prime or not.
fn compute_prime(prime: u128) -> bool {
    let limit = (prime as f64).sqrt() as u128; // casting number from floats to u128 again, not neccesary to check primes above this limit

    for i in 2..=limit {
        if prime % i == 0 {
            return false;
        }
    }

    true
}
