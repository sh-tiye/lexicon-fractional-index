extern crate lexicon_fractional_index;

use lexicon_fractional_index::key_between;

fn main() -> Result<(), String> {
  println!("{:?}", key_between("a", "b"));
  println!("{:?}", key_between("", ""));

  let base = "b".to_owned();
  let mut edge = "c".to_owned();
  for _ in 0..1000 {
    edge = key_between(&base, &edge)?;
    println!("base: {}", edge)
  }

  Ok(())
}
