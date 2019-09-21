/**
 * Author: Christian Visintin
 * Description: An introduction to automated tests in Rust
 */

//Tells to rust this is 'test', so will be compiled or run using cargo test
#[cfg(test)]
mod tests {
    //#[test] before fn, indicates that this function is used to test the module
    #[test]
    fn ends_with() {
        //@! assert
        //This test module is a standard in rust and it is used to test a module
        //In order to call from the test module a child module, we need to use the super:: syntax
        //If you're wondering why super, well remember that you're in module test, so from test go up and then enter utils
        //You can also use :: in front of the module name to start from the root
        //Super can be also used to invoke a parent module in any module though, so it's very useful

        //The test will fail if ends_with says that foobar doesn't end with bar

        //The test fails if the result of the expression is 'FALSE'
        assert!(super::utils::strutils::ends_with(&String::from("foobar"), &String::from("bar")));
        //To run test just type into your console 'cargo test'
    }
    #[test]
    fn concat() {
        //@! assert_eq
        //Assert eq ---> Check if foo + bar is equal to foobar
        //The test fails if foo + bar is not equal to foobar
        assert_eq!(super::utils::strutils::concat(&String::from("foo"), &String::from("bar")), String::from("foobar"));
        //NOTE: Also assert_ne! exists, which is the opposite of assert_eq
        //It's also possible to add custom messages to tests

        let fail_test = false; //NOTE: set to TRUE to fail the test
        if fail_test {
            let result = super::utils::strutils::broken_concat(&String::from("foo"), &String::from("bar"));
            assert_eq!(result, String::from("foobar"), "'foo' + 'bar' should be 'foobar', got '{}'", result);
        }
    }

    #[test]
    #[should_panic]
    fn second_word_panic() {
        //Return the first word
        //@! Should panic
        super::utils::strutils::get_second_word(&String::from("test"), ' ');
    }

    #[test]
    fn second_word_ok() {
        let s1 = String::from("Apple Bone Carrot");
        let second_word = super::utils::strutils::get_second_word(&s1, ' ');
        assert_eq!(second_word, String::from("Bone"), "Second word of '{}' should be 'Bone' but got '{}'", s1, second_word);
    }
}

//We'll create a utils mod, which is located at utils/ (the directory must matches the module name)
//Inside we'll have mod.rs which is standard name to define the module
//and its submodule with their own name (strutils)

//Here we define the utils module as public
pub mod utils;

//NOTE: by convention if you have a module named foo which doesn't have submodules, you should define it in foo.rs file
//Otherwise if it has submodules you should implement foo module in foo/ folder in mod.rs file and its submodules in the same directory
