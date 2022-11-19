use crate::benchmark::benchmark;
use crate::utils;
use crate::sieve_of_eratosthenes;
use crate::simple_prime_calculation;

fn test_algorithm(name: &str, n: i32, algo: fn(i32) -> Vec<i32>) {
    let mut primes: Vec<i32> = vec![];
    let duration = benchmark(|| { primes = algo(n); });

    println!("{} duration was {:?}", name, duration);
    println!("Count {}", primes.len());
    println!("Largest {}", primes.last().unwrap());
}

pub fn primes() {
    let mut args = std::env::args().skip(1);
    if args.len() != 1 {
        println!("Please provide only one argument.");
        return;
    }
    let arg = args.next().unwrap().to_string();
    if !utils::can_parse_i32(&arg) {
        println!("Please provide a valid integer as argument.");
        return;
    }    
    let n = arg.parse::<i32>().unwrap();
    if n <= 1 {
        println!("Please provide a number greater than 1.");
        return;
    }
    test_algorithm("Simple Prime Calculation", n, simple_prime_calculation::get_primes_until);
    test_algorithm("Sieve of Eratosthenes Prime Calculation", n, sieve_of_eratosthenes::get_primes_until);
}
