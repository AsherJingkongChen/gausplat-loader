//! This module provides functionality for reading and writing polygon files (PLY).
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
pub mod group;
pub mod head;

// TODO: Use AsciiStr internally and str externally.
// TODO: Philosphy:
// - A polygon file is an ordered sequence of blocks.
// - Everything is a decoder and encoder.
// - Enum follows partial variant pattern.
// - Relations should be tracked.
