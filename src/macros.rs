/// Defines a simple ServiceProvider.
///
/// ```
/// # #[macro_use] extern crate monto;
/// simple_service_provider! {
///     name = Example;
///     product = errors;
///     language = text;
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
        product = $product:ident;
        language = $lang:ident;
        ($path:ident, $input:ident) => $body:block
    ) => {
        pub struct $name;
        impl $crate::service::ServiceProvider for $name {
            fn descriptor(&self) -> $crate::common::messages::ProductDescriptor {
                let name = __product_name!($product);
                let language = __product_lang!($lang);
                $crate::common::messages::ProductDescriptor { name, language }
            }
            fn service(&mut self, $path: &str, $input: ::std::vec::Vec<$crate::common::messages::Product>) -> ::std::result::Result<$crate::service::messages::ServiceProduct, $crate::service::messages::ServiceErrors> $body
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __product_name {
    (directory) => { $crate::common::messages::ProductName::Directory };
    (errors) => { $crate::common::messages::ProductName::Errors };
    (highlighting) => { $crate::common::messages::ProductName::Highlighting };
    (source) => { $crate::common::messages::ProductName::Source };
    ($($n:ident .)+ $l:ident) => {{
        let mut s = ::std::string::String::new();
        $({
            s.push_str(stringify!($n));
            s.push('.');
        })+
        s.push_str(stringify!($l));
        s.parse().expect("Invalid product name from a macro")
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __product_lang {
    (json) => { $crate::common::messages::Language::Json };
    (none) => { $crate::common::messages::Language::None };
    (text) => { $crate::common::messages::Language::Text };
    ($i:ident) => { $crate::common::messages::Language::from(stringify!($i).to_owned()) };
    ($e:expr) => { $crate::common::messages::Language::from($e) };
}
