/// A helper trait for iterators over results
pub trait ResultIterator: Iterator {
    /// The type of `Result::Ok`
    type Inner;
    /// The type of `Result::Err`
    type Error;
    /// Peels off errors, discards them. Whatever ur left with is all u got bud.
    fn inner_iter(self) -> impl Iterator<Item = Self::Inner>;
    /// Peels off inners, i dont know why u would want that tho. fuckin negative nancy...
    fn error_iter(self) -> impl Iterator<Item = Self::Error>;
    /// A function which takes a closure, producing `Option<B>` some `Result`
    ///
    /// # Examples
    ///
    /// use std::{error::Error, path::Path};
    /// use crate::iter::ResultIterator;
    ///
    /// fn example() -> Result<impl Iterator<Item = , Box<dyn Error>> {
    ///     let path = Path::new("/");
    ///     let dir = fs::read_dir(path)?;
    ///     let iterator = dir.to_option_iter();
    ///     Ok(())
    /// }
    fn inner_map<F, B>(self, f: F) -> impl Iterator<Item = B>
    where
        Self: Sized,
        F: FnMut(Self::Inner) -> B,
    {
        self.inner_iter().map(f)
    }
    fn inner_filter_map<F, B>(self, f: F) -> impl Iterator<Item = B>
    where
        Self: Sized,
        F: FnMut(Self::Inner) -> Option<B>,
    {
        self.inner_iter().filter_map(f)
    }
    fn to_option_iter<F, B>(self, f: F) -> impl Iterator<Item = Option<B>>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        self.map(f)
    }
}

impl<I, T, E> ResultIterator for I
where
    Self: Sized,
    I: Iterator<Item = Result<T, E>>,
{
    // type Wrapper = Result<T, E>;
    type Inner = T;
    type Error = E;

    // fn to_option_iter<F, B>(self, f: F) -> impl OptionIterator {
    //     self.map(f)
    // }
    fn inner_iter(self) -> impl Iterator<Item = Self::Inner> {
        self.filter_map(Result::ok)
    }
    fn error_iter(self) -> impl Iterator<Item = Self::Error> {
        self.filter_map(Result::err)
    }
}
