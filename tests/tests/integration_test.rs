/**
 * Author: Christian Visintin
 * Description: This is an example of an integration test
 */

//NOTE: integration tests must be located at /tests/

extern crate tests;

use tests::utils;

#[test]
fn test_utils() {
  
}

//NOTE: it's also possible to use test submodules for integration
//Using mod

mod strutils;
