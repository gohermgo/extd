// pub mod index;
// pub use index::IndexableIterator;

pub use traits::{IndexableIterator, OptionIterator, ResultIterator};
// impl<I, T, E> Iterator for I
// where
//     I: ResultIterator,
//     E: std::error::Error,
// {
//     type Item = Result<T, E>;
//     fn next(&mut self) -> Option<Self::Item> {}
// }
mod traits;
