enum TestEnum {
    Foo = 1,
    Bar = 2,
    Wibble = 3,
    // Wobble = 4,
}

fn main() {
    let my_enum_value = TestEnum::Bar;

    match my_enum_value {
        TestEnum::Foo => println!("Foo"),
        TestEnum::Bar => println!("Bar"),
        TestEnum::Wibble => println!("Wibble"),
        // TestEnum::Wobble => println!("Wobble"),
        // _ => println!("No idea, but this'll do nicely")
    }
}
