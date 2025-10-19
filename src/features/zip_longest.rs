use core::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::{Stream, StreamExt, ready, stream::Fuse};
use pin_project_lite::pin_project;

use crate::{
    EitherOrBoth,
    internal::{
        check::assert_stream,
        either_or_none::{EitherOrNone, Entry},
    },
};

/// See [`crate::AsyncItertools::zip_longest`].
pub fn zip_longest<L: Stream, R: Stream>(l: L, r: R) -> crate::ZipLongest<L, R> {
    assert_stream(ZipLongest {
        l: l.fuse(),
        r: r.fuse(),
        ready: EitherOrNone::None,
    })
}

pin_project! {
    pub struct ZipLongest<L, R, Lt = <L as Stream>::Item, Rt = <R as Stream>::Item> {
        #[pin]
        l: Fuse<L>,
        #[pin]
        r: Fuse<R>,
        ready: EitherOrNone<Lt, Rt>,
    }
}

impl<L: Stream<Item = Lt>, R: Stream<Item = Rt>, Lt, Rt> Stream for ZipLongest<L, R, Lt, Rt> {
    type Item = EitherOrBoth<Lt, Rt>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.ready.entry() {
            Entry::None(entry) => {
                let l = this.l.poll_next(cx);
                let r = this.r.poll_next(cx);
                match (l, r) {
                    (Poll::Ready(Some(l)), Poll::Ready(Some(r))) => {
                        Poll::Ready(Some(EitherOrBoth::Both(l, r)))
                    }
                    (Poll::Ready(None), Poll::Ready(Some(r))) => {
                        Poll::Ready(Some(EitherOrBoth::Right(r)))
                    }
                    (Poll::Ready(Some(l)), Poll::Ready(None)) => {
                        Poll::Ready(Some(EitherOrBoth::Left(l)))
                    }
                    (Poll::Ready(None), Poll::Ready(None)) => Poll::Ready(None),
                    (Poll::Ready(Some(l)), Poll::Pending) => {
                        entry.insert_left(l);
                        Poll::Pending
                    }
                    (Poll::Pending, Poll::Ready(Some(r))) => {
                        entry.insert_right(r);
                        Poll::Pending
                    }
                    _ => Poll::Pending,
                }
            }
            Entry::Left(entry) => match ready!(this.r.poll_next(cx)) {
                Some(r) => Poll::Ready(Some(EitherOrBoth::Both(entry.remove(), r))),
                None => Poll::Ready(Some(EitherOrBoth::Left(entry.remove()))),
            },
            Entry::Right(entry) => match ready!(this.l.poll_next(cx)) {
                Some(l) => Poll::Ready(Some(EitherOrBoth::Both(l, entry.remove()))),
                None => Poll::Ready(Some(EitherOrBoth::Right(entry.remove()))),
            },
        }
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
