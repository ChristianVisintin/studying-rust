/**
 * Author: Christian Visintin
 * Description: grep module
 */

extern crate regex;
use regex::Regex;

///Find lines in haystack which have a match with the passed pattern
/// Pattern can be a regex or a plain string
pub fn match_pattern<'a, 'b>(haystack: &'a String, pattern: &String) -> Result<Vec<&'a str>, &'b str> {
    //Original code
    //for line in haystack.lines() {
    //  match Regex::new(pattern) {
    //    Ok(regex) => {
    //      if regex.is_match(line) {
    //        matches.push(line);
    //      }
    //    },
    //    Err(_err) => {
    //      return Err("Invalid regex");
    //    }
    //  };
    //}
    //@! Using iterators from chapter 12
    match Regex::new(pattern) {
      Ok(regex) => {
        return Ok(haystack.lines().filter(|line| regex.is_match(line)).collect())
      },
      Err(_err) => {
        return Err("Invalid regex");
      }
    };
}
