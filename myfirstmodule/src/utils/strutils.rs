/**
 * Author: Christian Visintin
 * Description: The strutils module implementation which is a submodule of utils
 * I Won't implement each module, is just to show how modules work
 */

//Pub keyword makes it public
pub fn ends_with(s: &String, needle: &String) -> bool {
  //I KNOW s.ends_with(needle) exists!
  let needle_len = needle.len();
  let str_len = s.len();
  //If needle len is bigger than str_len it can ends with needle
  if needle_len > str_len {
    return false
  }
  let str_end = &s[str_len - needle_len..];
  //If str_end is needle, then s ends with needle
  str_end == needle

}
