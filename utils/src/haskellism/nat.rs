//! Type-level natural numbers.

use std::marker::PhantomData;

/// A type-level natural number.
pub trait Nat {
    /// The value of the natural number at the value level.
    const VALUE: usize;
}

/// The type-level representation of zero.
pub struct NZero;

impl Nat for NZero {
    const VALUE: usize = 0;
}

/// The type-level representation of `n+1` for some `n`.
pub struct NSucc<T: Nat>(PhantomData<T>);

impl<T: Nat> Nat for NSucc<T> {
    const VALUE: usize = 1 + T::VALUE;
}

/// A trait for the length of a structure.
pub trait Length<N: Nat> {}
