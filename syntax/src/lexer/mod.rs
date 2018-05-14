macro_rules! re {
    ($re:expr) => {{
        lazy_static! {
            static ref RE: ::regex::Regex =
                ::regex::Regex::new($re).expect(concat!("Bad regex ", $re));
        };
        &*RE
    }};
}

#[allow(bad_style)]
macro_rules! Kind {
    ($(
        $(#[$meta:meta])*
        $kind:ident,
    )*) => {
        pub type Token = super::Token<Kind>;

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        #[derive(Serialize, Deserialize)]
        #[repr(u8)]
        pub enum Kind {$(
            $(#[$meta])*
            $kind,
        )*}

        impl Kind {
            pub fn as_str(&self) -> &'static str {
                match self {$(
                    Kind::$kind => stringify!($kind),
                )*}
            }
        }

        impl super::private_in_pub::Sealed for Kind {}

        impl ::serde::ser::Serialize for Token {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: ::serde::ser::Serializer {
                serializer.serialize_newtype_variant(
                    "Token",
                    self.kind as u32,
                    self.kind.as_str(),
                    &self.length,
                )
            }
        }
    };
}

mod private_in_pub {
    pub trait Sealed {}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Token<Kind: self::private_in_pub::Sealed> {
    pub length: u32,
    pub kind: Kind, // u8
}

pub mod code;
pub mod string;

#[test]
fn token_size() {
    use std::mem::size_of;
    assert_eq!(size_of::<code::Token>(), size_of::<Option<code::Token>>())
}
