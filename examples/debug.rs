extern crate lexicon_fractional_index;

use lexicon_fractional_index::key_between;

fn main() -> Result<(), String> {
  // let next = key_between("Xb0M", "Xb0M0V")?;

  // println!("next {}", next);

  // expected "Zz"
  // println!("next: {}", next);

  // println!("{:?}", key_between("a", "b"));
  // println!("{:?}", key_between("", ""));

  // let base = "b".to_owned();
  // let mut edge = "c".to_owned();
  // for _ in 0..1000 {
  //   edge = key_between(&base, &edge)?;
  //   println!("base: {}", edge)
  // }

  let left = "Xb0M".to_owned();
  let mut right = "Xb0M0V".to_owned();

  for _i in 0..1000 {
    let next = key_between(&left, &right)?;
    println!("key: {}", next);
    right = next;
  }

  println!("left: {}", left.len());
  println!("left: {}", left);

  Ok(())
}
