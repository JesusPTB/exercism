use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    ($a:expr => $b:expr) => ({
        let mut hm = HashMap::new();
        hm.insert($a, $b);
        hm
    });
    () => ({
        HashMap::new();
    });
    // ($a:expr => $b:expr, $($b:tt => $c:tt)*) => ({
    //
    // });
}

fn main() {
    println!("{:?}", hashmap!(1 => "one"));
    println!("{:?}", hashmap!());
}
