//! `polygon` can read and write polygon files (PLY).
//!
//! # Examples
//!
//! **Note:** Click triangle to view content.
//!
//! <details>
//! <summary>
//!     <strong><code>another-cube.greg-turk.ply</code>:</strong>
//! </summary>
//! <pre class=language-plaintext>
#![doc = include_str!("../../../examples/data/polygon/another-cube.greg-turk.ply")]
//! </pre>
//! </details>
#![doc = include_str!("SUPPLEMENT.md")]
#![doc = include_str!("LICENSE.md")]

pub mod body;
pub mod head;
pub mod object;

pub use body::Body;
pub use head::Head;
