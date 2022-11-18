

pub fn can_parse_i32(string_number: &std::string::String) -> bool {
  match string_number.parse::<i32>() {
      Ok(_n) => return true,
      Err(_e) => return false,
  }
}
