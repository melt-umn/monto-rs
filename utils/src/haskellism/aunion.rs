use haskellism::nat::{NSucc, NZero, Nat};

/// A trait for anonymous unions.
pub trait AUnion<T, U, Idx: Nat> {
    /// Places a value into the union.
    fn inject(t: T) -> Self;

    /// Attempts to get a value from the union.
    fn select(self) -> Result<T, U>;
}

/// The nil case of an anonymous union. Equivalent to the void/false/never/`!`
/// type; that is, an enum with no branches (and thus no values).
///
/// Note that this does not impl `AUnion` at all -- you can't `inject` to or
/// `select` from an empty union!
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AUnionNil {}

/// The cons case of an anonymous union.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AUnionCons<Hd, Tl> {
    /// The head value is present.
    Hd(Hd),

    /// The head value is not present, so a value must be present in the tail.
    Tl(Tl),
}

impl<Hd, Tl> AUnion<Hd, Tl, NZero> for AUnionCons<Hd, Tl> {
    fn inject(h: Hd) -> AUnionCons<Hd, Tl> {
        AUnionCons::Hd(h)
    }

    fn select(self) -> Result<Hd, Tl> {
        match self {
            AUnionCons::Hd(h) => Ok(h),
            AUnionCons::Tl(t) => Err(t),
        }
    }
}

impl<Hd, Tl, T, U, N: Nat> AUnion<T, AUnionCons<Hd, U>, NSucc<N>>
    for AUnionCons<Hd, Tl>
where
    Tl: AUnion<T, U, N>,
{
    fn inject(t: T) -> AUnionCons<Hd, Tl> {
        AUnionCons::Tl(AUnion::inject(t))
    }

    fn select(self) -> Result<T, AUnionCons<Hd, U>> {
        match self {
            AUnionCons::Hd(h) => Err(AUnionCons::Hd(h)),
            AUnionCons::Tl(t) => t.select().map_err(AUnionCons::Tl),
        }
    }
}
