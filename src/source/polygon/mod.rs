//! `polygon` can read and write polygon files (PLY).
//!
//! # Examples
//!
//! **Note:** Click triangle to view content.
//!
//! <details>
//! <summary>
//!     <strong><code>another-cube.greg-turk.ascii.ply</code>:</strong>
//! </summary>
//! <pre class=language-plaintext>
#![doc = include_str!("../../../examples/data/polygon/another-cube.greg-turk.ascii.ply")]
//! </pre>
//! </details>
#![doc = include_str!("SUPPLEMENT.md")]
#![doc = include_str!("LICENSE.md")]

pub mod body;
pub mod head;
pub mod object;

pub use body::Body;
pub use head::Head;
pub use object::Object;

macro_rules! impl_variant_matchers {
    ($subject:ident, $( $variant:ident ),* ) => {
        paste::paste! {
            impl [<$subject Variant>] {
                $(
                    #[inline]
                    pub const fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[inline]
                    pub const fn [<as_ $variant:snake>](&self) -> Option<&[<$variant $subject>]> {
                        match self {
                            Self::$variant(data) => Some(data),
                            _ => None,
                        }
                    }


                    #[inline]
                    pub fn [<as_ $variant:snake _mut>](&mut self) -> Option<&mut [<$variant $subject>]> {
                        match self {
                            Self::$variant(data) => Some(data),
                            _ => None,
                        }
                    }
                )*
            }
        }
    };
}
use impl_variant_matchers;
