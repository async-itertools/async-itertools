use futures_lite::Stream;

#[expect(unused, reason = "no futures implemented here yet")]
pub(crate) fn assert_future<F: Future>(future: F) -> F {
    future
}

pub(crate) fn assert_stream<S: Stream>(stream: S) -> S {
    stream
}
