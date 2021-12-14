extern crate lexicon_fractional_index;

use criterion::{criterion_group, criterion_main, Criterion};
use lexicon_fractional_index::{key_between, n_keys_between, float64_approx};
use rand::{thread_rng, Rng, random};

const BASE62_DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn get_int_len(head: char) -> usize {
  if head >= 'a' && head <= 'z' {
    head as usize - 'a' as usize + 2
  } else if head >= 'A' && head <= 'Z' {
    'Z' as usize - head as usize + 2
  } else {
    panic!()
  }
}

fn get_random_char() -> char {
  let mut rng = thread_rng();
  let random_index: usize = rng.gen_range(0..BASE62_DIGITS.len());
  return BASE62_DIGITS.chars().nth(random_index).unwrap();
}

fn get_random_char_non_zero() -> char {
  let mut rng = thread_rng();
  let random_index: usize = rng.gen_range(1..BASE62_DIGITS.len());
  return BASE62_DIGITS.chars().nth(random_index).unwrap();
}

fn get_random_head() -> (char, usize) {
  let mut rng = thread_rng();
  let random_index: usize = rng.gen_range(10..BASE62_DIGITS.len());
  let head = BASE62_DIGITS.chars().nth(random_index).unwrap();
  return (head, get_int_len(head));
}

/**
 * Generate random valid string tuple.
 * 
 * Generated string may be "" or without float part.
 * 
 * @param `min_len` - `u64` Minimal length of float part of generated string
 * 
 * @param `max_len` - `u64` Maximal length of float part generated string
 * 
 * @return `(String, String)` first always less than second
 */
fn generate_str_pair(min_len: u64, max_len: u64) -> (String, String) {
  let mut rng = thread_rng();
  let mut first;
  let mut second;
  loop {
    let is_first_float: bool = random();
    let is_second_float: bool = random();
    let (first_head, first_len) = get_random_head();
    let (second_head, second_len) = get_random_head();
    first = String::from(first_head);
    for _ in 0..first_len - 1 {
      first.push(get_random_char());
    }
    second = String::from(second_head);
    for _ in 0..second_len - 1 {
      second.push(get_random_char());
    }
    if is_first_float {
      let float_first_len = rng.gen_range(min_len..max_len);
      for i in 0..float_first_len {
        if i == float_first_len - 1 {
          first.push(get_random_char_non_zero());
        } else {
          first.push(get_random_char());
        }
      }
    }
    if is_second_float {
      let float_second_len = rng.gen_range(min_len..max_len);
      for i in 0..float_second_len {
        if i == float_second_len - 1 {
          second.push(get_random_char_non_zero());
        } else {
          second.push(get_random_char());
        }
      }
    }
    let is_first_empty: bool = random();
    let is_second_empty: bool = random();
    if is_first_empty {
      first = String::from("");
    }
    if is_second_empty {
      second = String::from("");
    }
    if first < second {
      break;
    }
  }
  return (first, second);
}

/**
 * @param `count` - `u64` Size of data set
 * 
 * @param `min_len` - `u64` Minimal length of float part of generated string
 * 
 * @param `max_len` - `u64` Maximal length of float part of generated string
 * 
 * @return `Vec<(String, String)>`
 */
fn generate_test_data(count: u64, min_len: u64, max_len: u64) -> Vec<(String, String)> {
  let mut res = Vec::new();
  let mut it_count = 0;
  while it_count < count {
    let str_pair = generate_str_pair(min_len, max_len);
    it_count += 1;
    res.push(str_pair);
  }
  return res;
}

fn criterion_benchmark(c: &mut Criterion) {

  c.bench_function(
    "key_between tests, 1e3 <= length < 1e4",
    |b| {
      let normal_test_data = generate_str_pair(1e3 as u64, 1e4 as u64);
      b.iter(
        || {
            match key_between(&normal_test_data.0, &normal_test_data.1) {
              Err(e) => panic!("{}", e),
              _ => (),
            };
        }
      )
    },
  );

  c.bench_function(
    "n_key_between tests, n = 100, 1e3 <= length < 1e4",
    |b| {
      let test_data = generate_str_pair(1e3 as u64, 1e4 as u64);
      b.iter(
        || {
          match n_keys_between(&test_data.0, &test_data.1, 100) {
            Err(e) => panic!("{}", e),
            _ => (),
          };
        }
      )
    }
  );

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
