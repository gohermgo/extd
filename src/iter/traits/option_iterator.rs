pub trait OptionIterator: Iterator {
    type Inner;
    fn inner_iter(self) -> impl Iterator<Item = Self::Inner>;
    fn fold_inner<F, B>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Inner) -> B;
    #[inline]
    #[allow(clippy::while_let_on_iterator)]
    fn fold_inner_i<F, B>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(usize, B, Self::Inner) -> B,
    {
        let (mut accum, mut inner_enumerate) = (init, self.inner_iter().enumerate());
        while let Some((i, x)) = inner_enumerate.next() {
            accum = f(i, accum, x);
        }
        accum
    }
    fn reduce_inner<F>(self, f: F) -> Option<Self::Inner>
    where
        Self: Sized,
        F: FnMut(Self::Inner, Self::Inner) -> Self::Inner;
    fn reduce_inner_i<F>(self, f: F) -> Option<Self::Inner>
    where
        Self: Sized,
        F: FnMut(usize, Self::Inner, Self::Inner) -> Self::Inner;
}
impl<I, T> OptionIterator for I
where
    Self: Sized,
    I: Iterator<Item = Option<T>>,
{
    type Inner = T;
    fn inner_iter(self) -> impl Iterator<Item = Self::Inner> {
        self.flatten()
    }
    #[inline]
    #[allow(clippy::while_let_on_iterator)]
    fn fold_inner<F, B>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Inner) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            if let Some(inner) = x {
                accum = f(accum, inner);
            } else {
                break;
            }
        }
        accum
    }
    #[inline]
    fn reduce_inner<F>(mut self, f: F) -> Option<Self::Inner>
    where
        Self: Sized,
        F: FnMut(Self::Inner, Self::Inner) -> Self::Inner,
    {
        let first = self.next()??;
        Some(self.fold_inner(first, f))
    }
    #[inline]
    fn reduce_inner_i<F>(mut self, f: F) -> Option<Self::Inner>
    where
        Self: Sized,
        F: FnMut(usize, Self::Inner, Self::Inner) -> Self::Inner,
    {
        let first = self.next()??;
        Some(self.fold_inner_i(first, f))
    }
}
