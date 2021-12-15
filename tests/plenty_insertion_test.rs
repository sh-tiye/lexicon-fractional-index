extern crate lexicon_fractional_index;

use lexicon_fractional_index::key_between;

#[test]
fn insert_end_test() -> Result<(), String> {
  let mut left: Option<String> = None;
  let right: Option<String> = None;

  for _ in 0..100000 {
    let next = key_between(&left, &right.to_owned())?;
    assert_ne!(left.to_owned(), Some(next.to_owned()));
    assert_ne!(right.to_owned(), Some(next.to_owned()));

    if left.is_some() {
      // println!("< {:?} {:?} {:?}", left, next, right);
      assert!(left < Some(next.to_owned()));
    }
    left = Some(next);
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}

#[test]
fn insert_beggining_test() -> Result<(), String> {
  let left: Option<String> = None;
  let mut right: Option<String> = None;

  for _ in 0..100000 {
    let next = key_between(&left.to_owned(), &right.to_owned())?;
    assert_ne!(left.to_owned(), Some(next.to_owned()));
    assert_ne!(right.to_owned(), Some(next.to_owned()));

    if right.is_some() {
      // println!("< {:?} {:?} {:?}", left, next, right);
      assert!(next < right.unwrap());
    }
    right = Some(next);
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}

#[test]
fn insert_middle_left_test() -> Result<(), String> {
  let left = Some("Zj".to_owned());
  let mut right = Some("Zk".to_owned());

  // too slow to have 100k insertions, use 4k only
  for _ in 0..4000 {
    let next = key_between(&left, &right)?;
    assert_ne!(left.as_ref().unwrap(), &next);
    assert_ne!(right.as_ref().unwrap(), &next);

    if right.is_some() {
      // println!("< {:?} {:?} {:?}", left, next, right);
      assert!(left.as_ref().unwrap() < &next);
      assert!(next < right.unwrap());
    }
    right = Some(next);
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}

#[test]
fn insert_middle_right_test() -> Result<(), String> {
  let mut left = Some("Zj".to_owned());
  let right = Some("Zk".to_owned());

  for _ in 0..4000 {
    let next = key_between(&left, &right)?;
    assert_ne!(left.as_ref().unwrap(), &next);
    assert_ne!(right.as_ref().unwrap(), &next);

    if right.as_ref().is_some() {
      // println!("< {:?} {:?} {:?}", left, next, right);
      assert!(left.as_ref().unwrap() < &next);
      assert!(&next < right.as_ref().unwrap());
    }
    left = Some(next);
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}

#[test]
fn insert_middle_left_right_test() -> Result<(), String> {
  let mut left = Some("Zj".to_owned());
  let mut right = Some("Zk".to_owned());

  let mut at_right = false;

  for _ in 0..2000 {
    let next = key_between(&left, &right)?;
    assert_ne!(left.as_ref().unwrap(), &next);
    assert_ne!(right.as_ref().unwrap(), &next);

    // println!("< {:?} {:?}", left, right);
    assert!(left.as_ref().unwrap() < &next);
    assert!(&next < right.as_ref().unwrap());

    if at_right {
      right = Some(next.to_owned());
      at_right = false
    } else {
      left = Some(next.to_owned());
      at_right = true
    }
  }

  println!("< {:?} {:?}", left, right);

  Ok(())
}
