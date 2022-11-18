pub fn get_primes_until(n: i32) -> Vec<i32> {
  let mut is_primes = Vec::new();
  let mut primes = Vec::<i32>::new();
  for _ in 0..n+1 {
    is_primes.push(true);
  }

  is_primes[0] = false;
  is_primes[1] = false;
  let mut c = 2;

  while c < is_primes.len() {
    for x in (2*c..is_primes.len()).step_by(c) {
      is_primes[x] = false;
    }
    c += 1;
    let next_prime = is_primes[c..].iter().position(|&r| r == true);
    if next_prime == None {
      break;
    }
    c += next_prime.unwrap();
  }
  
  for (i, is_prime) in is_primes.iter().enumerate() {
    if *is_prime {
      primes.push(i as i32);
    }
  }

  return primes;
}