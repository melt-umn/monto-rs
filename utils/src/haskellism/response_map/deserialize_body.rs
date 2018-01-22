use haskellism::aunion::{AUnion, AUnionCons, AUnionNil};
use haskellism::nat::Nat;
use haskellism::response_map::{RespBody, RespError, RespMap, RespMapCons,
                               RespMapNil};
use haskellism::response_map::status_types::StatusCode;
use haskellism::response_map::status_types::*;

/// Deserializes a JSON body into the appropriate response body.
pub fn deserialize_body<M, U, N, UHd, UTl>(
    status: ::hyper::StatusCode,
    s: &str,
) -> Result<U, RespError>
where
    U: AUnion<UHd, UTl, N>,
    U: AUnionForRespMap<M>,
    UHd: RespBody<Status200>,
    UHd: RespBody<Status204>,
    UHd: RespBody<Status400>,
    UHd: RespBody<Status409>,
    UHd: RespBody<Status500>,
    UHd: RespBody<Status502>,
    UHd: RespBody<Status503>,
    N: Nat,
{
    // include!(concat!(env!("OUT_DIR"), "/deserialize_body.rs"))
    unimplemented!()
}

fn respmap_to_aunion<S, T, M, U, MIdx, UIdx, Tmp>(
    s: &str,
) -> Result<U, RespError>
where
    S: StatusCode,
    T: RespBody<S>,
    M: RespMap<S, T, MIdx>,
    U: AUnion<T, Tmp, UIdx>,
    MIdx: Nat,
    UIdx: Nat,
{
    M::deserialize_body(s).map(U::inject)
}

/// Implemented for response maps that convert to the given type.
pub trait AUnionForRespMap<M> {}

impl<S, T, U, Tl> AUnionForRespMap<AUnionCons<T, U>> for RespMapCons<S, T, Tl>
where
    S: StatusCode,
    T: RespBody<S>,
    Tl: AUnionForRespMap<U>,
{
}
impl AUnionForRespMap<AUnionNil> for RespMapNil {}
