use std::cmp::min;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

fn main() {
    let start = 2;
    let end = 100_000_000;
    let threads = 32;

    let t0 = Instant::now();
    let primes = find_primes(start, end, threads);
    let dt = Instant::now() - t0;

    println!("Found {} primes between {} and {}.", primes.len(), start, end);
    println!("The run took {:.2} seconds.", dt.as_secs_f32());
}



fn find_primes(start: u64, end: u64, number_of_threads: usize) -> Vec<u64> {
    let (tx, rx) = channel();
    let numbers_per_thread = (end - start) / number_of_threads as u64 + 1;

    for thread_number in 0..number_of_threads as u64 {
        let start_range = start +  thread_number * numbers_per_thread;
        let mut end_range = start + (thread_number + 1) * numbers_per_thread;
        end_range = min(end, end_range);

        let tx = tx.clone();
        thread::spawn(move || {
            for i in start_range..end_range {
                if is_prime(i) {
                    tx.send(i).unwrap();
                }
            }
            println!("Finished thread {}.", thread_number);
        });
    }

    drop(tx);

    let mut primes = Vec::new();
    while let Ok(prime) = rx.recv() {
        primes.push(prime);
    }

    primes
}

fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
