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

pub fn concat(s1: &String, s2: &String) -> String {
  let mut s = String::from(s1);
  s = s + s2;
  s
}

pub fn broken_concat(s1: &String, s2: &String) -> String {
  let mut s_out = String::from(s2);
  s_out += s1;
  s_out
}

pub fn get_second_word(s: &String, separator: char) -> String {
  let words_iterator = s.split(separator);
  let words: Vec<&str> = words_iterator.collect();
  if words.len() < 2 {
    panic!("There is only one word in {}", s);
  }
  let result = words.get(1);
  
  match result {
    Some(r) => {
      let result = String::from(*r);
      return result;
    },
    None => {
      panic!("Second word is null");
    }
  }
}
