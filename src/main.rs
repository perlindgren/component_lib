#[macro_use]
extern crate lazy_static;

mod library {
    use std::sync::RwLock;
    lazy_static! {
        pub static ref LIB: RwLock<Vec<&'static str>> = {
            let rw = RwLock::new(Vec::new());
            rw.write().unwrap().push("add");
            rw
        };
    }
}

mod module1 {
    use super::*;

    lazy_static! {
        pub static ref LIB: u32 = {
            let rw = &library::LIB;
            rw.write().unwrap().push("riscv_mem");
            rw.write().unwrap().push("riscv_mem");
            2
        };
    }
}

fn main() {
    println!(
        "The lib has {} entries.",
        library::LIB.read().unwrap().len()
    );

    println!("Entries");
    for e in library::LIB.read().unwrap().iter() {
        println!("{}", e)
    }

    // here the module entries will lazily be added to the global library
    println!("The mod1 has {} entries.", *module1::LIB);

    println!(
        "The lib has {} entries.",
        library::LIB.read().unwrap().len()
    );

    println!("Entries");
    for e in library::LIB.read().unwrap().iter() {
        println!("{}", e)
    }
}
