pub mod index;
pub use index::IndexableIterator;
pub mod option;
pub use option::OptionIterator;
pub mod result;
pub use result::ResultIterator;

// impl<I, T, E> Iterator for I
// where
//     I: ResultIterator,
//     E: std::error::Error,
// {
//     type Item = Result<T, E>;
//     fn next(&mut self) -> Option<Self::Item> {}
// }
