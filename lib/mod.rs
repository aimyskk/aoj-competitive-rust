struct Combination {
  n: u64,
  fact_table: Vec<u64>
}

#[allow(dead_code)]
impl Combination {
  fn new(n: u64) -> Combination {
    Combination {
      n: n,
      fact_table: (1..n+1).scan(1,|a,x|{*a=modulo::mul(*a,x); Some(*a)}).collect::<Vec<_>>()
    }
  }

  fn comb(&self, r: u64) -> u64 {
    match r {
      _ if self.n < r => 0,
      _ if self.n == r => 1,
      _ => {
        let a = self.fact_table[self.n as usize - 1];
        let b = modulo::pow(self.fact_table[r as usize - 1], modulo::MOD-2);
        let c = modulo::pow(self.fact_table[(self.n - r) as usize - 1], modulo::MOD-2);
        [a,b,c].iter().fold(1, |a,&x| modulo::mul(a,x))
      }
    }
  }
}

#[allow(dead_code)]
mod modulo {
  pub const MOD: usize = 1_000_000_007;

  pub fn add(x: usize, y: usize) -> usize {
    (x + y) % MOD
  }

  pub fn sum(xs: &[usize]) -> usize {
    xs.iter().fold(0,|a,&x|add(a,x))
  }

  pub fn mul(x: usize, y: usize) -> usize {
    (x * y) % MOD
  }

  pub fn product(xs: &[usize]) -> usize {
    xs.iter().fold(1,|a,&x|mul(a,x))
  }

  pub fn pow(x: usize, n: usize) -> usize {
    match n {
      0 => 1,
      _ if n % 2 == 1 => mul(x, pow(x, n-1)),
      _ => {
        let q = pow(x, n>>1);
        mul(q, q)
      }
    }
  }
}
