#[derive(Debug, Eq, PartialEq)]
struct Primes {
    limit: usize,
}

struct PrimesIter {
    index: usize,
    computed: Vec<bool>,
}

fn computed_primes(limit: usize) -> Vec<bool> {
    let mut sieve = vec![true; limit];
    let mut m = 2;
    while m * m < limit {
        if sieve[m] {
            for i in (m * 2..limit).step_by(m) {
                sieve[i] = false;
            }
        }
        m += 1
    }
    sieve
}

impl Primes {
    fn new(limit: usize) -> Self {
        Primes { limit }
    }

    fn iter(&self) -> PrimesIter {
        PrimesIter {
            index: 1,
            computed: computed_primes(self.limit),
        }
    }
}

impl Iterator for PrimesIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.index += 1;
            if self.index > self.computed.len() - 1 {
                return None;
            } else if self.computed[self.index] {
                return Some(self.index);
            } else {
                continue;
            }
        }
    }
}
fn main() {
    let primes = Primes::new(100);
    println!("{:?},", primes.iter().next());
    for i in primes.iter() {
        println!("{},", i)
    }
}

#[cfg(test)]
mod tests {
    use crate::Primes;
    #[test]
    fn test_new_primes() {
        assert_eq!(Primes { limit: 100usize }, Primes::new(100))
    }

    #[test]
    fn test_iter() {
        let primes = Primes::new(10);
        assert_eq!(Some(2), primes.iter().next())
    }
}
