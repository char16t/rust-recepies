pub fn approx_equal(x: f64, y: f64, epsilon: f64) -> bool {
    return (x - y).abs() < epsilon;
}

pub fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    let mut x: usize = 2;
    while x * x <= n {
        if n % x == 0 {
            return false;
        }
        x += 1
    }
    true
}

pub fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut f: Vec<usize> = Vec::new();
    let mut x: usize = 2;
    while x * x <= n {
        while n % x == 0 {
            f.push(x);
            n /= x;
        }
        x += 1;
    }
    if n > 1 {
        f.push(n);
    }
    f
}

pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = vec![true; n + 1];
    let mut p: usize = 2;

    while p * p <= n {
        if primes[p] {
            let mut i = p * p;
            while i <= n {
                primes[i] = false;
                i += p;
            }
        }
        p += 1;
    }

    let mut result: Vec<usize> = Vec::new();
    for (num, &is_prime) in primes.iter().enumerate().skip(2) {
        if is_prime {
            result.push(num);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_equal() {
        assert!(approx_equal(0.001, 0.002, 0.01) == true);
        assert!(approx_equal(-0.001, 0.002, 0.01) == true);
        assert!(approx_equal(-0.001, -0.002, 0.01) == true);
        assert!(approx_equal(0.000024115, 0.000023115, 0.00001) == true);
        assert!(approx_equal(0.000024115, 0.000013115, 0.00001) == false);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(23), true);
        assert_eq!(is_prime(29), true);

        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(18), false);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(3), vec![3]);
        assert_eq!(prime_factors(4), vec![2, 2]);
        assert_eq!(prime_factors(5), vec![5]);
        assert_eq!(prime_factors(6), vec![2, 3]);
        assert_eq!(prime_factors(32), vec![2, 2, 2, 2, 2]);
        assert_eq!(prime_factors(34), vec![2, 17]);
    }

    #[test]
    fn test_sieve_of_eratosthenes() {
        let n: usize = 30;
        let primes: Vec<usize> = sieve_of_eratosthenes(n);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
    }
}
