/**
 * Author: Christian Visintin
 * Description: minigrep takes as argument the filename to search in and the string to look for
 */

use std::fs;

/// Read file content and returns the content as std::String
pub fn read_file(file: &String) -> Option<String> {
  let content = match fs::read_to_string(file) {
    Ok(cnt) => {
      Some(cnt)
    },
    Err(_error) => {
      None
    }
  };
  content
}
