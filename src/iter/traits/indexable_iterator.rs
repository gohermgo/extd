#[cfg(no_std)]
use ::core::iter::{Enumerate, Filter, FilterMap, Map};
#[cfg(not(no_std))]
use ::std::iter::{Enumerate, Filter, FilterMap, Map};

pub trait IndexableIterator: Iterator {
    #[inline]
    fn for_each_i<F>(self, f: F)
    where
        Self: Sized,
        F: FnMut((usize, Self::Item)),
    {
        self.enumerate().for_each(f)
    }
    #[doc(alias = "inject_i", alias = "foldl_i")]
    #[inline]
    #[allow(clippy::while_let_on_iterator)]
    fn fold_i<F, B>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(usize, B, Self::Item) -> B,
    {
        // let enumerated = self.enumerate();
        let (mut accum, mut enumerate) = (init, self.enumerate());
        while let Some((i, x)) = enumerate.next() {
            accum = f(i, accum, x);
        }
        accum
    }
    #[inline]
    fn reduce_i<F>(mut self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(usize, Self::Item, Self::Item) -> Self::Item,
    {
        let first = self.next()?;
        Some(self.fold_i(first, f))
    }
    #[inline]
    fn map_i<F, B>(self, f: F) -> Map<Enumerate<Self>, F>
    where
        Self: Sized,
        F: FnMut((usize, Self::Item)) -> B,
    {
        self.enumerate().map(f)
    }
    #[inline]
    fn filter_i<P>(self, predicate: P) -> Filter<Enumerate<Self>, P>
    where
        Self: Sized,
        P: FnMut(&(usize, Self::Item)) -> bool,
    {
        self.enumerate().filter(predicate)
    }
    #[inline]
    fn filter_map_i<F, B>(self, f: F) -> FilterMap<Enumerate<Self>, F>
    where
        Self: Sized,
        F: FnMut((usize, Self::Item)) -> Option<B>,
    {
        self.enumerate().filter_map(f)
    }
}
impl<I, T> IndexableIterator for I where I: Iterator<Item = T> {}
