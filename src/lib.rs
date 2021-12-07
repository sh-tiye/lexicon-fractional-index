//! Fractional string indexes for ordering items.
//! References
//! - https://www.figma.com/blog/realtime-editing-of-ordered-sequences/
//! Original source
//! - https://observablehq.com/@dgreensp/implementing-fractional-indexing
//!
//! We would like to be able to re-order items in an ordered list by only
//! updating the index of the thing that moved. We can do this by using
//! non-integer indexes, such that moving an item between two other items can be
//! done by making the item's index in between the previous item's index and the
//! next item's index.
//!
//! We will use strings for the indexes instead of decimals to avoid
//! error-prone floating point math and limited ability to represent floats
//! with large decimals.
//!
//! base10 configuration
//! export const ZERO = 48; // "0"
//! export const UPPER_LIMIT = 58; // ASCII code for "9" + 1

// base95 configuration
pub const ZERO: u8 = 32; // " ", beginning of ASCII range
pub const UPPER_LIMIT: u8 = 127; // end of ASCII range + 1
pub const EMPTY_NULL: &str = ""; // to represent JavaScript null

// `a` may be empty string or null, `b` is null or non-empty string.
// `a < b` lexicographically if `b` is non-null.
// no trailing zeros allowed.
pub fn fractional_index(a: &str, b: &str) -> Result<String, String> {
    // println!("doing with {:?} {:?}", a, b);
    if *a >= *b && !b.is_empty() {
        return Err(format!("{} >= {}", a, b));
    }
    if a.chars().nth_back(0) == Some(ZERO as char) || b.chars().nth_back(0) == Some(ZERO as char) {
        return Err(String::from("trailing zero"));
    }
    if !b.is_empty() {
        // remove longest common prefix.  pad `a` with 0s as we
        // go.  note that we don't need to pad `b`, because it can't
        // end before `a` while traversing the common prefix.
        let mut n = 0;
        while a.chars().nth(n).or(Some(ZERO as char)) == b.chars().nth(n) {
            n += 1;
        }
        if n > 0 {
            let next = fractional_index(&a[n..a.len()], &b[n..b.len()])?;
            return Ok(format!("{}{}", &b[0..n], next));
        }
    }
    // first digits (or lack of digit) are different
    let digit_a: u8 = if !a.is_empty() {
        a.chars().next().unwrap() as u8
    } else {
        ZERO
    };
    let digit_b: u8 = if !b.is_empty() {
        b.chars().next().unwrap() as u8
    } else {
        UPPER_LIMIT
    };
    if digit_b - digit_a > 1 {
        let mid_digit = (0.5 * (digit_a + digit_b) as f32).round() as u8;
        Ok((mid_digit as char).to_string())
    } else {
        // first digits are consecutive
        if b.len() > 1 {
            return Ok(b.chars().next().unwrap().to_string());
        } else {
            // `b` is null or has length 1 (a single digit).
            // the first digit of `a` is the previous digit to `b`,
            // or the largest digit if `b` is null.
            // given, for example in base10, midpoint('49', '5'), return
            // '4' + midpoint('9', null), which will become
            // '4' + '9' + midpoint('', null), which is '495'
            let next = fractional_index(&a[1..a.len()], "")?;
            return Ok(format!("{}{}", digit_a as char, next));
        }
    }
}
