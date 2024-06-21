use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::mem::{replace, swap};
use std::ops::{RemAssign, ShrAssign};
use rand::{Rng, thread_rng};

#[test]
fn test_jacobi_base() {
    let a = BigInt::from(10);
    let m = BigInt::from(21);
    assert_eq!(jacobi_base(&a, &m), -1);
}

#[test]
fn test_jacobi_new() {
    let a = BigInt::from(10);
    let m = BigInt::from(21);
    assert_eq!(jacobi_new(&a, &m), -1);

    let mut rnd = thread_rng();
    for _ in 0..100 {
        let mut a = BigInt::from(rnd.gen_range(3..1000));
        a.set_bit(0, true);
        let mut m = BigInt::from(rnd.gen_range(3..1000));
        m.set_bit(0, true);
        println!("a = {}, m = {}", a, m);
        assert_eq!(jacobi_base(&a, &m), jacobi_new(&a, &m));
    }
}

pub fn jacobi_base(a: &BigInt, m: &BigInt) -> i8 {
    if !m.is_positive() || m.is_even() {
        panic!("Invalid input");
    }

    // After the reduction, we know that both a and m are positive
    let mut a = a.mod_floor(m).into_parts().1;
    let mut m = m.magnitude().clone();

    // The output
    let mut t = true;

    while !a.is_zero() {
        while a.is_even() {
            a.shr_assign(1);
            let r = m.mod_floor(&BigUint::from(8u8));
            if r == BigUint::from(3u8) || r == BigUint::from(5u8) {
                t = !t;
            }
        }
        swap(&mut a, &mut m);
        if a.mod_floor(&BigUint::from(4u8)) == BigUint::from(3u8)
            && m.mod_floor(&BigUint::from(4u8)) == BigUint::from(3u8)
        {
            t = !t;
        }
        a.rem_assign(&m);
    }

    if m.is_one() {
        return if t { 1 } else { -1 };
    }
    0
}

pub fn jacobi_new(a: &BigInt, m: &BigInt) -> i8 {
    if !m.is_positive() || m.is_even() {
        panic!("Invalid input");
    }

    // After the reduction, we know that both a and m are positive
    let mut a = a.mod_floor(m).into_parts().1;
    let mut m = m.magnitude().clone();

    // The output
    let mut t = true;

    // The second bit of m
    let mut m_2 = m.bit(1);

    while !a.is_zero() {
        // Remove all trailing zeros from a and adjust t accordingly
        let trailing_zeros = a.trailing_zeros().expect("a is not zero");
        if !trailing_zeros.is_zero() {
            a.shr_assign(trailing_zeros);
        }

        // Swap a and m
        swap(&mut a, &mut m);
        let a_2 = replace(&mut m_2, m.bit(1));

        if (trailing_zeros.is_odd() && (a.bit(2) ^ a_2)) ^ (m_2 && a_2) {
            t = !t;
        }

        a.rem_assign(&m);
    }

    if m.is_one() {
        return if t { 1 } else { -1 };
    }
    0
}

pub fn jacobi_num_bigint_dig(a: &num_bigint_dig::BigInt, m: &num_bigint_dig::BigInt) -> Option<i8> {
    if !m.is_positive() || m.is_even() {
        return None;
    }
    Some(num_bigint_dig::algorithms::jacobi(a, m) as i8)
}
