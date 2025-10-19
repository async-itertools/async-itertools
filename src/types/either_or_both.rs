#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Enum for [`zip_longest`]'s item type.
///
/// [`zip_longest`]: crate::AsyncItertools::zip_longest
pub enum EitherOrBoth<L, R> {
    /// Both items are available.
    Both(L, R),
    /// Item from the left stream after right stream ended.
    Left(L),
    /// Item from the right stream after left stream ended.
    Right(R),
}
