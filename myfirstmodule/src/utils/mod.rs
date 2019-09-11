/**
 * Author: Christian Visintin
 * Description: The utils module with its sons
 * I Won't implement each module, is just to show how modules work
 */

//Let's declare a parent module called utils
//Let's declare two sub modules called respectively strutils and sysutils
//But we won't implement them in lib.rs, but in their own file
//Which will be strutils.rs and sysutils.rs
//We'll implement them inside the folder utils, because they're submodules of utils module

pub mod strutils;
pub mod sysutils;