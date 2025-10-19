use core::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::{Stream, StreamExt, ready, stream::Fuse};
use option_entry::{Entry, OptionEntry};
use pin_project_lite::pin_project;

use crate::{EitherOrBoth, internal::check::assert_stream};

/// See [`crate::AsyncItertools::zip_longest`].
pub fn zip_longest<L: Stream, R: Stream>(l: L, r: R) -> crate::ZipLongest<L, R> {
    assert_stream(ZipLongest {
        l: l.fuse(),
        r: r.fuse(),
        left_item: None,
    })
}

pin_project! {
    pub struct ZipLongest<L, R, Lt = <L as Stream>::Item> {
        #[pin]
        l: Fuse<L>,
        #[pin]
        r: Fuse<R>,
        left_item: Option<Lt>,
    }
}

impl<L: Stream<Item = Lt>, R: Stream<Item = Rt>, Lt, Rt> Stream for ZipLongest<L, R, Lt> {
    type Item = EitherOrBoth<Lt, Rt>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let l = match this.left_item.entry() {
            Entry::Vacant(entry) => {
                let Some(l) = ready!(this.l.poll_next(cx)) else {
                    return Poll::Ready(ready!(this.r.poll_next(cx)).map(EitherOrBoth::Right));
                };
                entry.insert_entry(l)
            }
            Entry::Occupied(entry) => entry,
        };
        let Some(r) = ready!(this.r.poll_next(cx)) else {
            return Poll::Ready(Some(EitherOrBoth::Left(l.remove())));
        };
        Poll::Ready(Some(EitherOrBoth::Both(l.remove(), r)))
    }
}

#[cfg(test)]
mod test {
    use crate::{AsyncItertools, EitherOrBoth};

    #[test]
    fn left_longer() {
        let l = futures_lite::stream::iter([1, 2, 3]);
        let r = futures_lite::stream::iter([1, 2]);
        assert!(futures_lite::stream::block_on(l.zip_longest(r)).eq([
            EitherOrBoth::Both(1, 1),
            EitherOrBoth::Both(2, 2),
            EitherOrBoth::Left(3),
        ]));
    }

    #[test]
    fn equal_length() {
        let l = futures_lite::stream::iter([1, 2, 3]);
        let r = futures_lite::stream::iter([1, 2, 3]);
        assert!(futures_lite::stream::block_on(l.zip_longest(r)).eq([
            EitherOrBoth::Both(1, 1),
            EitherOrBoth::Both(2, 2),
            EitherOrBoth::Both(3, 3),
        ]));
    }

    #[test]
    fn right_longer() {
        let l = futures_lite::stream::iter([1, 2]);
        let r = futures_lite::stream::iter([1, 2, 3]);
        assert!(futures_lite::stream::block_on(l.zip_longest(r)).eq([
            EitherOrBoth::Both(1, 1),
            EitherOrBoth::Both(2, 2),
            EitherOrBoth::Right(3),
        ]));
    }
}
