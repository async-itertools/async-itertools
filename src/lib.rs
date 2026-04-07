//! `itertools` for `Stream`s. Work-in-progress.
//!
//! Made with [`futures-lite`](<https://docs.rs/futures-lite/2.6.1>) in mind rather than
//! [`futures-util`](<https://docs.rs/futures-util/0.3.31>).
//!
//! Large parts of the code are taken from [the original crate](<https://docs.rs/itertools/0.14.0>).
//! Their licenses:
//!
//! - [MIT](<https://github.com/rust-itertools/itertools/blob/6bd5053fca990a8c7a3de6429faa53fef81de41d/LICENSE-MIT>)
//! - [Apache-2.0](<https://github.com/rust-itertools/itertools/blob/6bd5053fca990a8c7a3de6429faa53fef81de41d/LICENSE-APACHE>)

#![no_std]

use futures_lite::Stream;

pub use self::{
    features::{
        coalesce::{Coalesce, DedupBy, coalesce, dedup_by},
        zip_longest::zip_longest,
    },
    types::either_or_both::EitherOrBoth,
};

mod features;
mod internal;
mod types;

/// `Itertools` extension trait but for [`Stream`]s instead of [`Iterator`]s.
pub trait AsyncItertools: Stream {
    /// Yields pairs, [`EitherOrBoth::Both`], of items from both streams, until at least one runs
    /// out, then either [`EitherOrBoth::Left`] (if right ends) or [`EitherOrBoth::Right`] (if left
    /// ends).
    fn zip_longest<U>(self, other: U) -> ZipLongest<Self, U>
    where
        Self: Sized,
        U: Stream,
    {
        zip_longest(self, other)
    }

    fn coalesce<F>(self, f: F) -> Coalesce<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Result<Self::Item, (Self::Item, Self::Item)>,
    {
        coalesce(self, f)
    }

    fn dedup_by<F>(self, f: F) -> DedupBy<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        dedup_by(self, f)
    }
}

impl<T: ?Sized + Stream> AsyncItertools for T {}

/// Stream for the [`AsyncItertools::zip_longest()`] method.
pub type ZipLongest<L, R> = self::features::zip_longest::ZipLongest<L, R>;
