#[derive(Default)]
pub(crate) enum EitherOrNone<L, R> {
    #[default]
    None,
    Left(L),
    Right(R),
}

pub(crate) struct NoneEntry<'a, L, R> {
    eon: &'a mut EitherOrNone<L, R>,
}

pub(crate) struct LeftEntry<'a, L, R> {
    eon: &'a mut EitherOrNone<L, R>,
}

pub(crate) struct RightEntry<'a, L, R> {
    eon: &'a mut EitherOrNone<L, R>,
}

impl<'a, L, R> NoneEntry<'a, L, R> {
    fn new(eon: &'a mut EitherOrNone<L, R>) -> Self {
        assert!(matches!(eon, EitherOrNone::None));
        Self { eon }
    }

    pub(crate) fn insert_left(self, l: L) -> LeftEntry<'a, L, R> {
        *self.eon = EitherOrNone::Left(l);
        LeftEntry::new(self.eon)
    }

    pub(crate) fn insert_right(self, r: R) -> RightEntry<'a, L, R> {
        *self.eon = EitherOrNone::Right(r);
        RightEntry::new(self.eon)
    }
}

impl<'a, L, R> LeftEntry<'a, L, R> {
    fn new(eon: &'a mut EitherOrNone<L, R>) -> Self {
        assert!(matches!(eon, EitherOrNone::Left(_)));
        Self { eon }
    }

    pub(crate) fn remove(self) -> L {
        match core::mem::take(self.eon) {
            EitherOrNone::Left(l) => l,
            _ => unreachable!(),
        }
    }
}

impl<'a, L, R> RightEntry<'a, L, R> {
    fn new(eon: &'a mut EitherOrNone<L, R>) -> Self {
        assert!(matches!(eon, EitherOrNone::Right(_)));
        Self { eon }
    }

    pub(crate) fn remove(self) -> R {
        match core::mem::take(self.eon) {
            EitherOrNone::Right(r) => r,
            _ => unreachable!(),
        }
    }
}

pub enum Entry<'a, L, R> {
    None(NoneEntry<'a, L, R>),
    Left(LeftEntry<'a, L, R>),
    Right(RightEntry<'a, L, R>),
}

impl<L, R> EitherOrNone<L, R> {
    pub(crate) fn entry(&mut self) -> Entry<'_, L, R> {
        match self {
            EitherOrNone::None => Entry::None(NoneEntry::new(self)),
            EitherOrNone::Left(_) => Entry::Left(LeftEntry::new(self)),
            EitherOrNone::Right(_) => Entry::Right(RightEntry::new(self)),
        }
    }
}
