mod benchmark;
mod prime;
mod rsa;
mod sieve_of_eratosthenes;
mod simple_prime_calculation;
mod utils;

fn main() {
    prime::primes();
    rsa::run();
}
