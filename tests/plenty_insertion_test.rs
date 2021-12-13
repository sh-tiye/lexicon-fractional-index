extern crate lexicon_fractional_index;

use lexicon_fractional_index::key_between;

#[test]
fn insert_end_test() -> Result<(), String> {
  let mut left = "".to_owned();
  let right = "".to_owned();

  for _ in 0..100000 {
    let next = key_between(&left, &right)?;
    assert_ne!(&left, &next);
    assert_ne!(&right, &next);

    if !left.is_empty() {
      // println!("< {:?} {:?} {:?}", left, next, right);
      assert!(left < next);
    }
    left = next;
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}

#[test]
fn insert_beggining_test() -> Result<(), String> {
  let left = "".to_owned();
  let mut right = "".to_owned();

  for _ in 0..100000 {
    let next = key_between(&left, &right)?;
    assert_ne!(&left, &next);
    assert_ne!(&right, &next);

    if !right.is_empty() {
      // println!("< {:?} {:?} {:?}", left, next, right);
      assert!(next < right);
    }
    right = next;
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}

#[test]
fn insert_middle_left_test() -> Result<(), String> {
  let left = "Zj".to_owned();
  let mut right = "Zk".to_owned();

  // too slow to have 100k insertions, use 4k only
  for _ in 0..4000 {
    let next = key_between(&left, &right)?;
    assert_ne!(&left, &next);
    assert_ne!(&right, &next);

    if !right.is_empty() {
      // println!("< {:?} {:?} {:?}", left, next, right);
      assert!(left < next);
      assert!(next < right);
    }
    right = next;
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}

#[test]
fn insert_middle_right_test() -> Result<(), String> {
  let mut left = "Zj".to_owned();
  let right = "Zk".to_owned();

  for _ in 0..4000 {
    let next = key_between(&left, &right)?;
    assert_ne!(&left, &next);
    assert_ne!(&right, &next);

    if !right.is_empty() {
      // println!("< {:?} {:?} {:?}", left, next, right);
      assert!(left < next);
      assert!(next < right);
    }
    left = next;
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}
