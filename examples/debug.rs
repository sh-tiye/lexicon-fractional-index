use lexicon_fractional_index::fractional_index;

fn main() -> Result<(), String> {
  println!("{:?}", fractional_index("a", "b"));
  println!("{:?}", fractional_index("", ""));

  let mut base = "a".to_owned();
  let edge = "b".to_owned();
  for _ in 0..1000 {
    base = fractional_index(&base, &edge)?;
    println!("base: {}", base)
  }

  Ok(())
}
