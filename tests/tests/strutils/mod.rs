/**
 * Author: Christian Visintin
 * Description: This is an example of an integration test
 */

use tests::utils;

#[test]
fn test_strutils() {
  assert!(utils::strutils::ends_with(&String::from("foobar"), &String::from("bar")));
  assert_eq!(utils::strutils::concat(&String::from("foo"), &String::from("bar")), String::from("foobar"));
  let s1 = String::from("Apple Bone Carrot");
  let second_word = utils::strutils::get_second_word(&s1, ' ');
  assert_eq!(second_word, String::from("Bone"), "Second word of '{}' should be 'Bone' but got '{}'", s1, second_word);
}
