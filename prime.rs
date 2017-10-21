#[allow(dead_code)]
mod prime {
  pub fn is_prime(n: usize) -> bool {
    let sq = (n as f64).sqrt().floor() as usize;
    let p = (2..sq+1).all(|d| n%d != 0);
    p
  }

  pub fn primes(n: usize) -> Vec<usize> {
    (2..n+1).filter(|&x|is_prime(x)).collect()
  }

  pub fn factorize(n: usize) -> Vec<usize> {
    let sq = (n as f64).sqrt().floor() as usize;
    let mut fs = vec![];
    let mut x = n;
    for d in 2 .. sq+1 {
      if x == 1  {break}
      while x > 1 && x % d == 0 {
        fs.push(d);
        x /= d;
      }
    }
    if x != 1 {fs.push(x)};
    fs
  }
}
