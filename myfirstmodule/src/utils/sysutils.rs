/**
 * Author: Christian Visintin
 * Description: The sysutils module implementation which is a submodule of utils
 * I Won't implement each module, is just to show how modules work
 */

pub fn reboot() {
  println!("System is going down for reboot NOW!");
  std::thread::sleep(std::time::Duration::new(2, 0));
  println!("Just kidding.");
}