use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::mem::swap;
use std::ops::{RemAssign, ShrAssign};

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
    let mut m_2nd_bit = m.bit(1);

    while !a.is_zero() {
        // To check if m is 3 or 5 mod 8 we check that only one of the second and third bits are set
        let n = m_2nd_bit ^ m.bit(2);

        // Shift a to the right until odd and store the parity of the number of shifts
        while a.is_even() {
            a.shr_assign(1);
            if n {
                t = !t;
            }
        }

        swap(&mut a, &mut m);

        // a and m have been swapped
        let a_2nd_bit = m_2nd_bit;
        m_2nd_bit = m.bit(1);

        // Check if both a and m are 3 mod 4
        if a_2nd_bit && m_2nd_bit {
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
