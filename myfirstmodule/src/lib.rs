/**
 * Author: Christian Visintin
 * Description: A module example
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //This test module is a standard in rust and it is used to test a module
        //In order to call from the test module a child module, we need to use the super:: syntax
        //If you're wondering why super, well remember that you're in module test, so from test go up and then enter utils
        //You can also use :: in front of the module name to start from the root
        //Super can be also used to invoke a parent module in any module though, so it's very useful

        //The test will fail if ends_with says that foobar doesn't end with bar
        assert!(super::utils::strutils::ends_with(&String::from("foobar"), &String::from("bar")));

        //To run test just type into your console 'cargo test'
    }
}

//We'll create a utils mod, which is located at utils/ (the directory must matches the module name)
//Inside we'll have mod.rs which is standard name to define the module
//and its submodule with their own name (strutils and sysutils)

//Here we define the utils module as public
pub mod utils;

//NOTE: by convention if you have a module named foo which doesn't have submodules, you should define it in foo.rs file
//Otherwise if it has submodules you should implement foo module in foo/ folder in mod.rs file and its submodules in the same directory
