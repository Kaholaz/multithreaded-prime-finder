fn main() {
    for prime in primes_between(1000, 10000).iter() {
        println!("{}", prime);
    }
}


struct Arguments {
    rest: Vec<String>
}

impl Arguments {
    fn get_joined(&self, separator: &str) -> String {
        self.rest.join(separator)
    }
}

fn is_prime(a: u32) -> bool {
    if a % 2 == 0 {
        return false;
    }

    let limit = (a as f64).sqrt() as u32;
    for i in (3..limit).step_by(2) {
        if a % i == 0 {
            return false;
        }
    }
    true
}

fn primes_between(a: u32, b: u32) -> Box<Vec<u32>>{
    let mut list: Box<Vec<u32>> = Box::new(Vec::new());

    for n in a..b {
        if is_prime(n) { list.push(n); }
    }

    list
}
