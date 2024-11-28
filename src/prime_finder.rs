pub fn find_primes(until: u128) -> Vec<u128> {
    assert!(until >= 3);
    let mut primes = vec![3];

    for i in 0..((until - 2) / 2) {
        let val = 3 + i * 2;
        // we only have to check for number up until the sqareroot of val
        let max_test = sqrt_newton(val) + 1;

        let is_prime = 'prime: {
            for prime in primes.iter() {
                if *prime > max_test {
                    break 'prime true;
                }
                if val % *prime == 0 {
                    break 'prime false;
                }
            }

            true
        };
        if !is_prime {
            continue;
        }
        primes.push(val);
    }
    primes.insert(0, 2);
    primes
}

// FIXME: implement: AKS Primality Test
// FIXME: implement Elliptic Curve Primality Proving
// FIXME: implement Miller-Rabin Primality Test   |   ---  FERMAT'S THEOREM

pub fn is_prime(val: u128) -> bool {
    // we only have to check for number up until the sqareroot of val
    let max_test = sqrt_newton(val) + 1;
    for div in 0..max_test {
        if val % div == 0 {
            return false;
        }
    }
    true
}

fn sqrt_newton(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut guess = n / 2; // Startwert, z.B. die Hälfte von n

    loop {
        let next_guess = (guess + n / guess) / 2;
        
        // Wenn der Unterschied zwischen der Schätzung und der nächsten Schätzung sehr klein ist, brechen wir ab
        if guess == next_guess {
            break;
        }

        guess = next_guess;
    }

    guess
}