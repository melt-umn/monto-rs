use haskellism::aunion::{AUnion, AUnionCons, AUnionNil};
use haskellism::nat::Nat;
use haskellism::response_map::{RespBody, RespError, RespMap, RespMapCons,
                               RespMapNil};
use haskellism::response_map::status_types::StatusCode;

/// Deserializes a JSON body into the appropriate response body.
pub fn deserialize_body<M, U, N, UHd, UTl>(
    status: ::hyper::StatusCode,
    s: &str,
) -> Result<U, RespError>
where
    U: AUnionForRespMap<M>,
    U: AUnion<UHd, UTl, N>,
    N: Nat,
{
    include!(concat!(env!("OUT_DIR"), "/deserialize_body.rs"))
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

trait AUnionForRespMap<M> {}

impl<S, T, U, Tl> AUnionForRespMap<AUnionCons<T, U>> for RespMapCons<S, T, Tl>
where
    S: StatusCode,
    T: RespBody<S>,
    Tl: AUnionForRespMap<U>,
{
}
impl AUnionForRespMap<AUnionNil> for RespMapNil {}
