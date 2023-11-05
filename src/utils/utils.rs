#[macro_export]
macro_rules! check {
    ($expr:expr) => {
        match $expr {
            Ok(_) => {}
            Err(err) => println!("{:?}", err),
        }
    };
}
