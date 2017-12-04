/// Defines a simple ServiceProvider.
///
/// ```
/// # #[macro_use] extern crate monto;
/// simple_service_provider! {
///     name = Example;
///     product = "errors";
///     language = "text";
///     (path, input) => {
///         unimplemented!();
///     }
/// }
///
/// simple_service_provider! {
///     name = OtherExample;
///     // Both of these are "custom".
///     product = "edu.umn.cs.melt.custom_product";
///     language = "ableC";
///     (path, input) => {
///         unimplemented!();
///     }
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! simple_service_provider {
    (
        name = $name:ident;
        product = $product:expr;
        language = $lang:expr;
        ($path:ident, $input:ident) => $body:block
    ) => {
        pub struct $name;
        impl $crate::ServiceProvider for $name {
            fn descriptor(&self) -> $crate::monto3_common::messages::ProductDescriptor {
                let name = $product.parse().expect("Invalid product name from macro");
                let language = $lang.to_owned().into();
                $crate::monto3_common::messages::ProductDescriptor { name, language }
            }
            fn service(&mut self, $path: &str, $input: ::std::vec::Vec<$crate::monto3_common::messages::Product>) -> (::std::result::Result<$crate::serde_json::Value, ::std::vec::Vec<$crate::messages::ServiceError>>, ::std::vec::Vec<$crate::messages::ServiceNotice>) $body
        }
    }
}
