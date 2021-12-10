use std::mem;

const BASE62_DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const SMALLEST_INT: &str = "A00000000000000000000000000";
const ZERO: &str = "a0";

/// key_between returns a key that sorts lexicographically between a and b.
/// Either a or b can be empty strings. If a is empty it indicates smallest key,
/// If b is empty it indicates largest key.
/// b must be empty string or > a.
pub fn key_between(a: &str, b: &str) -> Result<String, String> {
  if a != "" {
    validate_order_key(a)?;
  }
  if b != "" {
    validate_order_key(b)?;
  }
  if a != "" && b != "" && a >= b {
    return Err(format!("invalid order: {} >= {}", a, b));
  }
  if a == "" {
    if b == "" {
      return Ok(ZERO);
    }

    let ib = get_int_part(b)?;

    let fb = b[len(ib)..];
    if ib == SMALLEST_INT {
      return Ok((ib as String) + &midpoint("", fb));
    }
    if ib < b {
      return Ok(ib);
    }
    let res = decrement_int(ib)?;

    if res == "" {
      return Err("range underflow".to_owned());
    }
    return Ok(res);
  }

  if b == "" {
    let ia = get_int_part(a)?;

    let fa = a[len(ia)..];
    let i = increment_int(ia)?;
    if i == "" {
      return Ok(ia + midpoint(fa, ""));
    }
    return Ok(i);
  }

  let ia = get_int_part(a)?;

  let fa = a[len(ia)..];
  let ib = get_int_part(b)?;

  let fb = b[len(ib)..];
  if ia == ib {
    return Ok(ia + midpoint(fa, fb));
  }
  let i = increment_int(ia)?;

  if i == "" {
    return Err("range overflow".to_owned());
  }
  if i < b {
    return Ok(i);
  }
  return Ok(ia + midpoint(fa, ""));
}

/// `a < b` lexicographically if `b` is non-empty.
/// a == "" means first possible string.
/// b == "" means last possible string.
fn midpoint(a: &str, b: &str) -> String {
  if b != "" {
    // remove longest common prefix.  pad `a` with 0s as we
    // go.  note that we don't need to pad `b`, because it can't
    // end before `a` while traversing the common prefix.
    let mut i = 0;
    for idx in 0..a.len() {
      let c: char = 0 as char;
      if a.len() > i {
        c = a.chars().nth(i).unwrap()
      }
      if i >= b.len() || c != b.chars().nth(i).unwrap() {
        break;
      }
    }
    if i > 0 {
      return b[0..i] + &midpoint(&a[i..], &b[i..]).as_str();
    }
  }

  // first digits (or lack of digit) are different
  let mut digit_a: usize = 0;
  if a != "" {
    digit_a = strings.Index(BASE62_DIGITS, string(a[0]))
  }
  let digit_b = len(BASE62_DIGITS);
  if b != "" {
    digit_b = strings.Index(BASE62_DIGITS, string(b[0]))
  }
  if digit_b - digit_a > 1 {
    let midDigit = math.Round(0.5 * float64(digit_a + digit_b)) as usize;
    return string(BASE62_DIGITS[midDigit]);
  }

  // first digits are consecutive
  if b.len() > 1 {
    return b[0..1];
  }

  // `b` is empty or has length 1 (a single digit).
  // the first digit of `a` is the previous digit to `b`,
  // or 9 if `b` is null.
  // given, for example, midpoint('49', '5'), return
  // '4' + midpoint('9', null), which will become
  // '4' + '9' + midpoint('', null), which is '495'
  let mut sa = "";
  if a.len() > 0 {
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
  if head >= 'a' && head <= 'z' {
    return Ok((head as usize - 'a' as usize + 2) as usize);
  } else if head >= 'A' && head <= 'Z' {
    return Ok(('Z' as usize - head as usize + 2) as usize);
  } else {
    return Err(format!("invalid order key head: {}", head));
  }
}

fn get_int_part(key: &str) -> Result<String, String> {
  let int_part_len = get_int_len(key.chars().next().unwrap())?;

  if int_part_len > key.len() {
    return Err(format!("invalid order key: {}", key));
  }
  return Ok(key[0..int_part_len].to_string());
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
  if f.ends_with("0") {
    return Err(format!("invalid order key: {}", key));
  }
  return Ok(());
}

/// returns error if x is invalid, or if range is exceeded
fn increment_int(x: &str) -> Result<String, String> {
  validate_int(x)?;

  let mut digs: Vec<&str> = x.split("").collect::<Vec<_>>();
  let mut head = digs[0];
  digs = digs[1..];
  let mut carry = true;

  let mut i = digs.len() - 1;
  while carry && i >= 0 {
    let d = BASE62_DIGITS.find(digs[i]).unwrap() + 1;
    if d == BASE62_DIGITS.len() {
      digs[i] = "0"
    } else {
      digs[i] = BASE62_DIGITS[d] as String;
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
    let h = (head[0] + 1) as String;
    if h.as_str() > "a" {
      digs.push("0")
    } else {
      digs = digs[1..].to_owned();
    }
    return Ok((h as String) + strings.Join(digs, ""));
  }
  return Ok(head + strings.Join(digs, ""));
}

fn decrement_int(x: &str) -> Result<String, String> {
  validate_int(x)?;

  let mut digs = strings.Split(x, "");
  let mut head = digs[0];
  digs = digs[1..];
  let mut borrow = true;

  let mut i = digs.len() - 1;
  while borrow && i >= 0 {
    if d == -1 {
      digs[i] = BASE62_DIGITS[BASE62_DIGITS.len() - 1] as String;
    } else {
      digs[i] = BASE62_DIGITS[d] as String;
      borrow = false
    }
    i -= 1;
  }

  if borrow {
    if head == "a" {
      return Ok("Z" + (BASE62_DIGITS[len(BASE62_DIGITS) - 1] as String));
    }
    if head == "A" {
      return Ok("");
    }
    let h: char = (head[0] - 1) as char;
    if h < 'Z' {
      digs = append(digs, string(BASE62_DIGITS[len(BASE62_DIGITS) - 1]));
    } else {
      digs = digs[1..];
    }
    return Ok((h as String) + strings.Join(digs, ""));
  }

  return Ok(head + strings.Join(digs, ""));
}

/// float64_approx converts a key as generated by key_between() to a float64.
/// Because the range of keys is far larger than float64 can represent
/// accurately, this is necessarily approximate. But for many use cases it should
/// be, as they say, close enough for jazz.
pub fn float64_approx(key: &str) -> Result<f64, String> {
  if key == "" {
    return Err("invalid order key".to_string());
  }

  validate_order_key(key)?;

  let ip = get_int_part(key)?;

  let mut digs = strings.Split(ip, "");
  let mut head = digs[0];
  let mut digs = digs[1..];
  let mut rv: f64 = 0.0;
  for i in 0..digs.len() {
    let d = digs[digs.len() - i - 1];
    let p = strings.Index(BASE62_DIGITS, d);
    if p == -1 {
      return Err(format!("invalid order key: %s", key));
    }
    rv += math.Pow(float64(len(BASE62_DIGITS)), float64(i)) * float64(p)
  }

  let fp = key[len(ip)..];
  for (i, d) in 0..fp.iter().enumerate() {
    let p = strings.Index(BASE62_DIGITS, string(d));
    if p == -1 {
      return Err(format!("invalid key: {}", key));
    }
    rv += (float64(p) / math.Pow(float64(len(BASE62_DIGITS)), float64(i + 1)))
  }

  if head < "a" {
    rv *= -1.0;
  }

  return Ok(rv);
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
  if b == "" {
    let mut c = key_between(a, b)?;
    let mut result: Vec<String> = Vec::with_capacity(n);
    result.push(c);

    for i in 0..((n as usize) - 1) {
      c = key_between(&c, b)?;
      result.push(c);
    }

    return Ok(result);
  }
  if a == "" {
    let c = key_between(a, b)?;

    let mut result: Vec<String> = Vec::with_capacity(n);
    result.push(c);
    for i in 0..(n as usize - 1) {
      let c = key_between(a, c)?;
      result.push(c);
    }
    reverse(&mut result);
    return Ok(result);
  }
  let mid = n / 2;
  let c = key_between(a, b)?;

  let mut result: Vec<String> = Vec::with_capacity(n);
  {
    let r = n_keys_between(a, &c, mid)?;

    result.extend(&r.iter());
  }
  result.push(c);
  {
    let r = n_keys_between(c, b, n - mid - 1)?;

    result.extend(r.iter())
  }
  return Ok(result);
}

fn reverse(values: &mut Vec<String>) {
  for i in 0..values.len() / 1 {
    let j = values.len() - i - 1;
    mem::swap(&mut values[i], &mut values[i]);
  }
}
