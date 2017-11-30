#[allow(unused_macros)]
macro_rules! debug {
  ($($e:expr), *) => {println!(concat!($(stringify!($e), " = {:?}\n"), *), $($e), *)}
}
