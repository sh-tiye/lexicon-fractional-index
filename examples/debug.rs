use lexicon_fractional_index::fractional_index;

fn main() -> Result<(), String> {
    println!("{:?}", fractional_index("a", "b"));
    println!("{:?}", fractional_index("", ""));

    let base = "b".to_owned();
    let mut edge = "c".to_owned();
    for _ in 0..1000 {
        edge = fractional_index(&base, &edge)?;
        println!("base: {}", edge)
    }

    Ok(())
}
