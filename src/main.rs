fn euler_criterion(a: u64, p: u64) -> Result<bool, &'static str> {
    if p < 2 || p % 2 == 0 {
        return Err("p must be an odd prime");
    }

    // Check if a is coprime to p
    if gcd(a, p) != 1 {
        return Err("a must be coprime to p");
    }

    let exponent = (p - 1) / 2;
    let result = mod_exp(a, exponent, p);

    if result == 1 {
        Ok(true) // a is a quadratic residue modulo p
    } else if result == p - 1 {
        Ok(false) // a is a non-residue modulo p
    } else {
        Err("Unexpected result")
    }
}

// Helper function to compute greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Helper function for modular exponentiation
fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    base = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }

    result
}

fn main() {
    let a =120;
    let p =307;
    
    match euler_criterion(a, p) {
        Ok(true) => println!("{} is a quadratic residue modulo {}", a, p),
        Ok(false) => println!("{} is a non-residue modulo {}", a, p),
        Err(err) => println!("Error: {}", err),
    }
}

