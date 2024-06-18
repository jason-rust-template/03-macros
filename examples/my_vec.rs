use anyhow::{Ok, Result};
use macros::my_vec;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
    ];

    let v1 = <[_]>::into_vec(Box::new([1, 2, 3, 4]));
    println!("{:?}", v1);

    // let v = vec![1, 2, 3, 4];
    println!("{:?}", v);
    Ok(())
}
