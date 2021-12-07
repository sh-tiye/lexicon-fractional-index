#[cfg(test)]
mod tests {
  use lexicon_fractional_index::{fractional_index, EMPTY_NULL, UPPER_LIMIT, ZERO};

  fn validate_fractional_index(a: &str, b: &str) -> Result<(), String> {
    let index = fractional_index(a, b)?;
    let normalized_a = if !a.is_empty() {
      a.to_string()
    } else {
      (ZERO as char).to_string()
    };
    let normalized_b = if !b.is_empty() {
      b.to_string()
    } else {
      (UPPER_LIMIT as char).to_string()
    };
    // `${index} is not greater than ${normalized_a}`
    assert!(index > normalized_a);

    // `${index} is not less than ${normalized_b}`
    assert!(index < normalized_b);
    Ok(())
  }

  #[test]
  fn errors_when_the_first_argument_is_greater_than_the_second() {
    assert!(fractional_index("2", "1").is_err());
    assert!(fractional_index("11", "1").is_err());
  }

  #[test]
  fn errors_when_the_two_arguments_are_equal() {
    // TODO handle handle this edge case
    // assert!(fractional_index("", "").is_err());
    assert!(fractional_index("1", "1").is_err());
    assert!(fractional_index("555", "555").is_err());
  }

  #[test]
  fn errors_when_there_are_trailing_zeros() {
    assert!(fractional_index(&format!("1{}", ZERO as char), EMPTY_NULL).is_err());
    assert!(fractional_index(EMPTY_NULL, &format!("1{}", ZERO as char)).is_err());
  }

  #[test]
  fn calculates_an_index_for_the_first_item_in_a_list() -> Result<(), String> {
    validate_fractional_index(EMPTY_NULL, EMPTY_NULL)?;
    Ok(())
  }

  #[test]
  fn calculates_an_index_when_inserting_to_the_beginning_of_the_list() -> Result<(), String> {
    validate_fractional_index(EMPTY_NULL, "5")?;
    validate_fractional_index(EMPTY_NULL, "3")?;
    validate_fractional_index(EMPTY_NULL, "2")?;
    validate_fractional_index(EMPTY_NULL, "1")?;
    validate_fractional_index(EMPTY_NULL, "05")?;
    validate_fractional_index(EMPTY_NULL, "03")?;
    validate_fractional_index(EMPTY_NULL, "02")?;
    validate_fractional_index(EMPTY_NULL, "01")?;
    Ok(())
  }

  #[test]
  fn calculates_an_index_when_inserting_to_the_end_of_the_list() -> Result<(), String> {
    validate_fractional_index("5", EMPTY_NULL)?;
    validate_fractional_index("8", EMPTY_NULL)?;
    validate_fractional_index("9", EMPTY_NULL)?;
    validate_fractional_index("95", EMPTY_NULL)?;
    validate_fractional_index("98", EMPTY_NULL)?;
    validate_fractional_index("99", EMPTY_NULL)?;
    Ok(())
  }

  #[test]
  fn calculates_an_index_when_inserting_between_two_indexes() -> Result<(), String> {
    validate_fractional_index("05", "1")?;
    validate_fractional_index("001", "001002")?;
    validate_fractional_index("001", "001001")?;
    validate_fractional_index("499", "5")?;
    validate_fractional_index("O", "OO")?;

    Ok(())
  }
}
