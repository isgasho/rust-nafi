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
        pub type Token<'a> = super::Token<'a, Kind>;

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

        impl<'a> ::serde::ser::Serialize for Token<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: ::serde::ser::Serializer {
                serializer.serialize_newtype_variant(
                    "Token",
                    u32::from(self.kind as u8),
                    self.kind.as_str(),
                    self.source,
                )
            }
        }
    };
}

mod private_in_pub {
    pub trait Sealed {}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Token<'a, Kind: private_in_pub::Sealed> {
    pub source: &'a str,
    pub kind: Kind,
}

pub mod code;
pub mod string;
