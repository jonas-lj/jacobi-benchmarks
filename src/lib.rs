use std::mem::swap;
use std::ops::{RemAssign, ShrAssign};

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

mod jacobi_taiko;
pub use jacobi_taiko::jacobi as jacobi_taiko;

#[cfg(test)]
use rand::{thread_rng, Rng};

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
        let mut m = BigInt::from(rnd.gen_range(3..1000));

        // Ensure inputs are odd
        a.set_bit(0, true);
        m.set_bit(0, true);

        let result = jacobi_new(&a, &m);
        println!("a = {}, m = {}, (a/m) = {}", a, m, result);
        let expected = jacobi_base(&a, &m);
        assert_eq!(expected, result);

        let result_taiko = jacobi_taiko::<2>(&a.to_u64_digits().1, &m.to_u64_digits().1);
        assert_eq!(expected, result_taiko as i8);
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

    // The second bit of m (will be a after swap)
    let mut m_2 = m.bit(1);

    while !a.is_zero() {
        // Remove all trailing zeros from a and adjust t accordingly
        let trailing_zeros = a.trailing_zeros().expect("a is not zero");
        if !trailing_zeros.is_zero() {
            a.shr_assign(trailing_zeros);
        }

        let a_2 = a.bit(1);
        if (trailing_zeros.is_odd() && (m_2 ^ m.bit(2))) ^ (m_2 && a_2) {
            t = !t;
        }

        // Swap a and m
        m_2 = a_2;
        swap(&mut a, &mut m);
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
