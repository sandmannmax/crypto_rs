
pub fn get_primes_until(n: i32) -> Vec<i32> {
  let mut primes = Vec::new();
  if n >= 2 {
      primes.push(2);
  }

  while primes.last().unwrap() < &n {
      let next_prime = find_next_prime(*primes.last().unwrap());
      primes.push(next_prime);
  }

  if primes.last().unwrap() > &n {
      primes.pop();
  }

  return primes;
}

fn find_next_prime(n: i32) -> i32 {
  let mut next = n + 1;
  while !is_prime(&next) {
      next += 1;
  }
  return next;
}

fn is_prime(n: &i32) -> bool {
  if n < &2 {
      return false;
  }
  for x in 2..n-1 {
      if n % x == 0 {
          return false;
      }
  }
  return true;
}
