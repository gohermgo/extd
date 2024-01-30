#[allow(unused_imports)]
mod iter;
use iter::{index::IndexableIterator, option::OptionIterator};
use std::ops::Mul;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_number = "101101011";
    println!("binary_number is {}", binary_number);
    let decimal_value = binary_number
        .chars()
        .map(|c| c.to_digit(10))
        .rev()
        .inner_iter()
        .fold_i(0, |index, acc, e| acc + (e.mul(2u32.pow(index as u32))));
    println!("value in decimal {}", decimal_value);
    let arr = [1, 2, 3];
    arr.iter()
        .for_each_i(|x| println!("Element {} is {}", x.0, x.1));
    // let p = std::path::Path::new("/");
    // let x = std::fs::read_dir(p)?;
    // let v = x.inner_iter();

    // let y = x.inner_iter();
    // let s = x.to_option_iter(std::io::Result::ok);
    // let c = s.inner_iter();
    // c.for_each(|z| {
    //     let v = z;
    // });
    Ok(())
}
