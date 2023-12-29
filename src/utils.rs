#[macro_export]
macro_rules! int_enum {
    (
        $(#[$enum_attr:meta])*
        $name:ident ($vtype:ty) {
            $(
                $(#[$meta:meta])*
                $key:ident = $value:literal
            ),+$(,)?
        }
    ) => {
        $(#[$enum_attr])*
        pub enum $name {
            $(
                $(#[$meta])*
                $key
            ),+,
            Reserved($vtype),
        }

        impl $name {
            pub fn value(&self) -> $vtype {
                match self {
                    $(
                        Self::$key => $value
                    ),+,
                    Self::Reserved(n) => *n,
                }
            }
            pub fn from_value(value: $vtype) -> Self {
                match value {
                    $(
                        $value => Self::$key
                    ),+,
                    n => Self::Reserved(n),
                }
            }
        }
    };
}
