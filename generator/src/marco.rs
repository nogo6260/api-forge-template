#[macro_export]
macro_rules! restify_config {
    // pub Test {
    //  ...
    // }
    ($(#[$outer:meta])* pub struct $struct_name:ident {
        $(
            $(#[$inner:ident $($args:tt)*])*
            pub $field_name:ident : $field_type:ty
        ),* $(,)?
    }) => {
        $(#[$outer])*
        pub struct $struct_name {
            $(
                $(#[$inner $($args)*])*
                pub $field_name: $field_type
            ),*
        }
    };

    // pub Test;
    ($(#[$outer:meta])* pub struct $struct_name:ident;) => {
        $(#[$outer])*
        pub struct $struct_name;
    };
}
