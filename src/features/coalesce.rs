use core::{
    pin::Pin,
    task::{Context, Poll, ready},
};

use futures_lite::Stream;
use option_entry::OptionEntry;
use pin_project_lite::pin_project;

use crate::internal::check::assert_stream;

pin_project! {
    #[project = Proj]
    pub struct CoalesceBy<S: Stream, F, C: CountItem<S::Item>> {
        #[pin]
        stream: S,
        last: Option<Option<C::CItem>>,
        f: F,
    }
}

pub trait CoalescePredicate<Item, T> {
    fn coalesce_pair(&mut self, t: T, item: Item) -> Result<T, (T, T)>;
}

impl<S: Stream, F: CoalescePredicate<S::Item, C::CItem>, C: CountItem<S::Item>> Stream
    for CoalesceBy<S, F, C>
{
    type Item = C::CItem;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Proj {
            mut stream,
            last,
            f,
        } = self.project();
        let current = match last {
            Some(elt) => elt,
            None => last.insert(ready!(stream.as_mut().poll_next(cx)).map(C::new)),
        };
        let option_entry::Entry::Occupied(mut entry) = current.entry() else {
            return Poll::Ready(None);
        };
        while let Some(next) = ready!(stream.as_mut().poll_next(cx)) {
            match f.coalesce_pair(entry.remove(), next) {
                Ok(item) => entry = current.entry().insert_entry(item),
                Err((last, next)) => {
                    *current = Some(next);
                    return Poll::Ready(Some(last));
                }
            }
        }
        Poll::Ready(Some(entry.remove()))
    }
}

pub struct NoCount;

pub trait CountItem<T> {
    type CItem;
    fn new(t: T) -> Self::CItem;
}

impl<T> CountItem<T> for NoCount {
    type CItem = T;
    fn new(t: T) -> Self::CItem {
        t
    }
}

pub type Coalesce<S, F> = CoalesceBy<S, F, NoCount>;

impl<F, Item, T> CoalescePredicate<Item, T> for F
where
    F: FnMut(T, Item) -> Result<T, (T, T)>,
{
    fn coalesce_pair(&mut self, t: T, item: Item) -> Result<T, (T, T)> {
        self(t, item)
    }
}

pub fn coalesce<S: Stream, F>(stream: S, f: F) -> Coalesce<S, F>
where
    F: FnMut(S::Item, S::Item) -> Result<S::Item, (S::Item, S::Item)>,
{
    assert_stream(CoalesceBy {
        stream,
        last: None,
        f,
    })
}

pub type DedupBy<S, Pred> = CoalesceBy<S, DedupPred2CoalescePred<Pred>, NoCount>;

pub struct DedupPred2CoalescePred<DP>(DP);

pub trait DedupPredicate<T> {
    fn dedup_pair(&mut self, a: &T, b: &T) -> bool;
}

impl<DP, T> CoalescePredicate<T, T> for DedupPred2CoalescePred<DP>
where
    DP: DedupPredicate<T>,
{
    fn coalesce_pair(&mut self, t: T, item: T) -> Result<T, (T, T)> {
        if self.0.dedup_pair(&t, &item) {
            Ok(t)
        } else {
            Err((t, item))
        }
    }
}

pub struct DedupEq;

impl<T: PartialEq> DedupPredicate<T> for DedupEq {
    fn dedup_pair(&mut self, a: &T, b: &T) -> bool {
        a == b
    }
}

impl<T, F: FnMut(&T, &T) -> bool> DedupPredicate<T> for F {
    fn dedup_pair(&mut self, a: &T, b: &T) -> bool {
        self(a, b)
    }
}

pub fn dedup_by<S: Stream, Pred>(stream: S, dedup_pred: Pred) -> DedupBy<S, Pred>
where
    Pred: FnMut(&S::Item, &S::Item) -> bool,
{
    assert_stream(CoalesceBy {
        stream,
        last: None,
        f: DedupPred2CoalescePred(dedup_pred),
    })
}

pub type Dedup<S> = DedupBy<S, DedupEq>;

pub fn dedup<S: Stream<Item: PartialEq>>(stream: S) -> Dedup<S> {
    assert_stream(CoalesceBy {
        stream,
        last: None,
        f: DedupPred2CoalescePred(DedupEq),
    })
}
