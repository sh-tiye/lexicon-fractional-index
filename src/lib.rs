const BASE62_DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const SMALLEST_INT: &str = "A00000000000000000000000000";
const ZERO: &str = "a0";

/// key_between returns a key that sorts lexicographically between a and b.
/// Either a or b can be empty strings. If a is empty it indicates smallest key,
/// If b is empty it indicates largest key.
/// b must be empty string or > a.
pub fn key_between(a: &str, b: &str) -> Result<String, String> {
  if !a.is_empty() {
    validate_order_key(a)?;
  }
  if !b.is_empty() {
    validate_order_key(b)?;
  }
  if !a.is_empty() && !b.is_empty() && a >= b {
    return Err(format!("invalid order: {} >= {}", a, b));
  }
  if a.is_empty() {
    if b.is_empty() {
      return Ok(ZERO.to_owned());
    }

    let ib = get_int_part(b)?;

    let fb = &b[ib.len()..];
    if ib == SMALLEST_INT {
      return Ok((ib as String) + &midpoint("", fb));
    }
    if ib.as_str() < b {
      return Ok(ib);
    }
    let res = decrement_int(&ib)?;

    if res.is_empty() {
      return Err("range underflow".to_owned());
    }
    return Ok(res);
  }

  if b.is_empty() {
    let ia = get_int_part(a)?;

    let fa = &a[ia.len()..];
    let i = increment_int(&ia)?;
    if i.is_empty() {
      return Ok(ia + &midpoint(fa, ""));
    }
    return Ok(i);
  }

  let ia = get_int_part(a)?;

  let fa = &a[ia.len()..];
  let ib = get_int_part(b)?;

  let fb = &b[ib.len()..];
  if ia == ib {
    return Ok(ia + &midpoint(fa, fb));
  }
  let i = increment_int(&ia)?;

  if i.is_empty() {
    return Err("range overflow".to_owned());
  }
  if i.as_str() < b {
    return Ok(i);
  }
  Ok(ia + &midpoint(fa, ""))
}

/// `a < b` lexicographically if `b` is non-empty.
/// a == "" means first possible string.
/// b == "" means last possible string.
fn midpoint(a: &str, b: &str) -> String {
  if !b.is_empty() {
    // remove longest common prefix.  pad `a` with 0s as we
    // go.  note that we don't need to pad `b`, because it can't
    // end before `a` while traversing the common prefix.
    let mut i = 0;
    for _ in 0..a.len() {
      let mut c: char = 0 as char;
      if a.len() > i {
        c = a.chars().nth(i).unwrap()
      }
      if i >= b.len() || c != b.chars().nth(i).unwrap() {
        break;
      }
      i += 1;
    }
    if i > 0 {
      return b[0..i].to_string() + midpoint(&a[i..], &b[i..]).as_str();
    }
  }

  // first digits (or lack of digit) are different
  let mut digit_a: usize = 0;
  if !a.is_empty() {
    digit_a = BASE62_DIGITS.find(a.chars().next().unwrap()).unwrap()
  }
  let mut digit_b = BASE62_DIGITS.len();
  if !b.is_empty() {
    digit_b = BASE62_DIGITS.find(b.chars().next().unwrap()).unwrap()
  }
  if digit_b - digit_a > 1 {
    let mid_digit = (0.5 * (digit_a + digit_b) as f64).round() as usize;
    return BASE62_DIGITS.chars().nth(mid_digit).unwrap().to_string();
  }

  // first digits are consecutive
  if b.len() > 1 {
    return b[0..1].to_string();
  }

  // `b` is empty or has length 1 (a single digit).
  // the first digit of `a` is the previous digit to `b`,
  // or 9 if `b` is null.
  // given, for example, midpoint('49', '5'), return
  // '4' + midpoint('9', null), which will become
  // '4' + '9' + midpoint('', null), which is '495'
  let mut sa = "";
  if !a.is_empty() {
    sa = &a[1..]
  }
  return BASE62_DIGITS.chars().nth(digit_a).unwrap().to_string() + &midpoint(sa, "");
}

fn validate_int(i: &str) -> Result<(), String> {
  let exp = get_int_len(i.chars().next().unwrap())?;

  if i.len() != exp {
    return Err(format!("invalid integer part of order key: {}", i));
  }
  Ok(())
}

fn get_int_len(head: char) -> Result<usize, String> {
  if ('a'..='z').contains(&head) {
    Ok((head as usize - 'a' as usize + 2) as usize)
  } else if ('A'..='Z').contains(&head) {
    Ok(('Z' as usize - head as usize + 2) as usize)
  } else {
    Err(format!("invalid order key head: {}", head))
  }
}

fn get_int_part(key: &str) -> Result<String, String> {
  let int_part_len = get_int_len(key.chars().next().unwrap())?;

  if int_part_len > key.len() {
    return Err(format!("invalid order key: {}", key));
  }
  Ok(key[0..int_part_len].to_string())
}

fn validate_order_key(key: &str) -> Result<(), String> {
  if key == SMALLEST_INT {
    return Err(format!("invalid order key: {}", key));
  }
  // get_int_part will return error if the first character is bad,
  // or the key is too short.  we'd call it to check these things
  // even if we didn't need the result
  let i = get_int_part(key)?;

  let f = &key[i.len()..];
  if f.ends_with('0') {
    return Err(format!("invalid order key: {}", key));
  }
  Ok(())
}

/// returns error if x is invalid, or if range is exceeded
fn increment_int(x: &str) -> Result<String, String> {
  validate_int(x)?;

  let mut digs: Vec<String> = x
    .split("")
    .collect::<Vec<&str>>()
    .iter()
    .map(|s| s.to_string())
    .collect();
  let head = digs[0].to_owned();
  digs = digs[1..].to_vec();
  let mut carry = true;

  let mut i = digs.len() as i64 - 1;
  while carry && i >= 0 {
    let d = BASE62_DIGITS.find(&digs[i as usize]).unwrap() + 1;
    if d == BASE62_DIGITS.len() {
      digs[i as usize] = "0".to_owned()
    } else {
      digs[i as usize] = BASE62_DIGITS.chars().nth(d).unwrap().to_string();
      carry = false;
    }

    i -= 1;
  }
  if carry {
    if head == "Z" {
      return Ok("a0".to_owned());
    }
    if head == "z" {
      return Ok("".to_owned());
    }
    let h = ((head.chars().next().unwrap() as u8 + 1) as char).to_string();
    if h.as_str() > "a" {
      digs.push("0".to_owned())
    } else {
      digs = digs[1..].to_owned();
    }
    return Ok((h as String) + &digs.join(""));
  }
  Ok(head.to_owned() + &digs.join(""))
}

fn decrement_int(x: &str) -> Result<String, String> {
  validate_int(x)?;

  let mut digs: Vec<String> = x
    .split("")
    .collect::<Vec<_>>()
    .iter()
    .map(|s| s.to_string())
    .collect();
  let head = digs[0].to_owned();
  digs.remove(0);
  let mut borrow = true;

  let mut i = digs.len() as i64 - 1;
  while borrow && i >= 0 {
    let d: i64 = match BASE62_DIGITS.find(&digs[0]) {
      Some(n) => (n - 1) as i64,
      None => -2, // TODO
    };

    if d == -1 {
      digs[i as usize] = BASE62_DIGITS.chars().nth_back(0).unwrap().to_string();
    } else {
      digs[i as usize] = BASE62_DIGITS.chars().nth(d as usize).unwrap().to_string();
      borrow = false
    }
    i -= 1;
  }

  if borrow {
    if head == "a" {
      return Ok("Z".to_owned() + &BASE62_DIGITS.chars().nth_back(0).unwrap().to_string());
    }
    if head == "A" {
      return Ok("".to_owned());
    }
    let h: char = (head.chars().next().unwrap() as u8 - 1) as char;
    if h < 'Z' {
      digs.push((BASE62_DIGITS.chars().nth_back(0).unwrap()).to_string());
    } else {
      digs.remove(0);
    }
    return Ok(h.to_string() + &digs.join(""));
  }

  Ok(head.to_owned() + &digs.join(""))
}

/// float64_approx converts a key as generated by key_between() to a float64.
/// Because the range of keys is far larger than float64 can represent
/// accurately, this is necessarily approximate. But for many use cases it should
/// be, as they say, close enough for jazz.
pub fn float64_approx(key: &str) -> Result<f64, String> {
  if key.is_empty() {
    return Err("invalid order key".to_string());
  }

  validate_order_key(key)?;

  let ip = get_int_part(key)?;

  let mut digs: Vec<&str> = ip.split("").collect::<Vec<_>>();
  let head = digs[0];
  digs.remove(0);
  let mut rv: f64 = 0.0;
  for i in 0..digs.len() {
    let d = digs[digs.len() - i - 1];
    let p = BASE62_DIGITS.find(d);
    if p == None {
      return Err(format!("invalid order key: {}", key));
    }
    rv += (BASE62_DIGITS.len() as f64).powf(i as f64) * p.unwrap() as f64
  }

  let fp = key[ip.len()..].to_owned();
  for (i, d) in fp.chars().enumerate() {
    let p = BASE62_DIGITS.find(d as char);
    if p == None {
      return Err(format!("invalid key: {}", key));
    }
    rv += (p.unwrap() as f64) / (BASE62_DIGITS.len() as f64).powf((i + 1) as f64)
  }

  if head < "a" {
    rv *= -1.0;
  }

  Ok(rv)
}

/// n_keys_between returns n keys between a and b that sorts lexicographically.
/// Either a or b can be empty strings. If a is empty it indicates smallest key,
/// If b is empty it indicates largest key.
/// b must be empty string or > a.
pub fn n_keys_between(a: &str, b: &str, n: usize) -> Result<Vec<String>, String> {
  if n == 0 {
    return Ok(vec![]);
  }
  if n == 1 {
    let c = key_between(a, b)?;

    return Ok(vec![c]);
  }
  if b.is_empty() {
    let mut c = key_between(a, b)?;
    let mut result: Vec<String> = Vec::with_capacity(n);
    result.push(c.to_owned());

    for _i in 0..((n as usize) - 1) {
      c = key_between(&c, b)?;
      result.push(c.to_owned());
    }

    return Ok(result);
  }
  if a.is_ascii() {
    let c = key_between(a, b)?;

    let mut result: Vec<String> = Vec::with_capacity(n);
    result.push(c.to_owned());
    for _i in 0..(n as usize - 1) {
      let c = key_between(a, &c)?;
      result.push(c.to_owned());
    }
    result.reverse();
    return Ok(result);
  }
  let mid = n / 2;
  let c = key_between(a, b)?;

  let mut result: Vec<String> = Vec::with_capacity(n);
  {
    let key_r = n_keys_between(a, &c, mid)?;
    for item in key_r {
      result.push(item.clone());
    }
  }
  result.push(c.to_owned());
  {
    let key_r = n_keys_between(&c, b, n - mid - 1)?;
    for item in key_r.iter() {
      result.push(item.to_owned());
    }
  }
  Ok(result)
}
