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

macro_rules! impl_map_filters {
    ($subject:ident, $( $map:ident ),* ) => {
        paste::paste! {
            $(
                #[inline]
                pub fn [<get_ $map:snake>](
                    &self,
                    id: &Id,
                ) -> Option<&[<$map $subject>]> {
                    self.[<$subject:snake _map>]
                        .get(id)
                        .and_then(|c| c.variant.[<as_ $map:snake>]())
                }

                #[inline]
                pub fn [<get_ $map:snake _mut>](
                    &mut self,
                    id: &Id,
                ) -> Option<&mut [<$map $subject>]> {
                    self.[<$subject:snake _map>]
                        .get_mut(id)
                        .and_then(|c| c.variant.[<as_ $map:snake _mut>]())
                }

                #[inline]
                pub fn [<iter_ $map:snake>](
                    &self,
                ) -> impl Iterator<Item = (&Id, &[<$map $subject>])> {
                    self.[<$subject:snake _map>]
                        .iter()
                        .filter_map(|(id, c)| {
                            c.variant.[<as_ $map:snake>]().map(|c| (id, c))
                        })
                }

                #[inline]
                pub fn [<iter_ $map:snake _mut>](
                    &mut self,
                ) -> impl Iterator<Item = (&Id, &mut [<$map $subject>])> {
                    self.[<$subject:snake _map>]
                        .iter_mut()
                        .filter_map(move |(id, c)| {
                            c.variant.[<as_ $map:snake _mut>]().map(|c| (id, c))
                        })
                }
            )*
        }
    };
}
use impl_map_filters;

macro_rules! impl_variant_matchers {
    ($subject:ident, $( $variant:ident ),* ) => {
        paste::paste! {
            impl  [<$subject Variant>] {
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
