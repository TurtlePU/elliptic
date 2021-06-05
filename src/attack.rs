use std::time::{Duration, Instant};

use num_bigint::BigUint;
use num_traits::{One, Pow, Zero};
use rand::{Rng, thread_rng};

use crate::algebra::{curve::Curve, fields::zn::{BigPrime, Zn}};

pub fn crack() {
    pollard();
    println!("====================");
    brute_force();
}

const CURVE_ORDER: usize = 126;

fn brute_force() {
    let gen = Curve126::affine(Zn::from(4), Zn::from(14)).unwrap();
    let mut bench = vec![];
    for x in 1..CURVE_ORDER {
        let y = gen.clone() * x.into();
        let now = Instant::now();
        for sol in 1..CURVE_ORDER {
            if gen.clone() * sol.into() == y {
                break;
            }
        }
        let elapsed = now.elapsed();
        bench.push(elapsed);
        if x % 10 == 0 {
            println!("Cracked {} / {}", x, CURVE_ORDER);
        }
    }
    let total: Duration = bench.into_iter().sum();
    println!("bruteforce all points elapsed {:?}", total);
}

#[derive(Debug)]
struct Curve126;
type Z127 = Zn<Curve126>;

impl BigPrime for Curve126 {
    fn value() -> BigUint {
        BigUint::from(127usize)
    }
}

impl Curve<Z127> for Curve126 {
    fn group_order() -> BigUint {
        BigUint::from(CURVE_ORDER)
    }

    fn a() -> Z127 {
        Z127::one()
    }

    fn b() -> Z127 {
        Z127::one()
    }
}

const PRIME_ORDER: usize = 3258;

fn pollard() {
    //let gen = loop {
    //    let x: ZN = thread_rng().gen();
    //    if x.is_zero() { continue; }
    //    break x;
    //};
    let gen = ZN::from(23);
    println!("generator is {:?}", gen);
    let mut bench = vec![];
    for x in 1..PRIME_ORDER {
        let x = BigUint::from(x);
        let y = gen.clone().pow(x.clone());
        let now = Instant::now();
        let z = solve_pollard(&gen, &y);
        let elapsed = now.elapsed();
        assert!(gen.clone().pow(z.unwrap().into()) == y);
        bench.push(elapsed);
        if (&x % BigUint::from(100_usize)).is_zero() {
            println!("Cracked {} / {}", x, PRIME_ORDER);
        }
    }
    let total: Duration = bench.into_iter().sum();
    println!("bruteforce all primes elapsed {:?}", total);
}

fn solve_pollard(gen: &ZN, y: &ZN) -> Option<BigUint> {
    let step = |(x, a, b): (ZN, ZP, ZP)| match usize::from(&x) % 3 {
        0 => (x.clone() * x, a.clone() + a, b.clone() + b),
        1 => (x * gen.clone(), a + ZP::one(), b),
        _ => (x * y.clone(), a, b + ZP::one()),
    };
    let mut x1 = (ZN::one(), Zero::zero(), Zero::zero());
    let mut x2 = x1.clone();
    loop {
        x1 = step(x1);
        x2 = step(step(x2));

        if x1.0 == x2.0 {
            let (_, a1, b1) = x1;
            let (_, a2, b2) = x2;
            let r = b1 - b2;
            break if r.is_zero() {
                None
            } else {
                let q = a2 - a1;
                Some(BigUint::from(dbg!(q) / dbg!(r)))
            };
        }
    }
}

#[derive(Debug)]
struct N;
#[derive(Debug)]
struct N1;

type ZN = Zn<N>;
type ZP = Zn<N1>;

impl BigPrime for N {
    fn value() -> BigUint {
        BigUint::from(PRIME_ORDER)
    }
}

impl BigPrime for N1 {
    fn value() -> BigUint {
        N::value() - BigUint::one()
    }
}
