pub mod grep;
/**
 * Author: Christian Visintin
 * Description: Minigrep core
 */
pub mod reader;

#[cfg(test)]
mod tests {
    #[test]
    fn search_in_file() {
        let file: String = String::from("Cargo.toml");
        let file_content: Option<String> = super::reader::read_file(&file);
        let content: String;
        match file_content {
            Some(s) => {
                content = s.clone();
            }
            None => {
                panic!("Could not read file {}", file);
            }
        };
        //Match 'name'
        let pattern = String::from("name");
        match super::grep::match_pattern(&content, &pattern) {
            Ok(matches) => {
                for match_str in matches {
                    println!("{}", match_str);
                    assert_eq!(match_str, "name = \"minigrep\"");
                }
            }
            Err(err) => {
                panic!(err);
            }
        };
    }

    #[test]
    fn search_in_string() {
        let haystack: String = String::from("Line 1\nLine 2\nLine 3\nLine 4\n");
        let pattern = String::from("3");
        match super::grep::match_pattern(&haystack, &pattern) {
            Ok(matches) => {
                for match_str in matches {
                    println!("{}", match_str);
                    assert_eq!(match_str, "Line 3");
                }
            }
            Err(err) => {
                panic!(err);
            }
        }
    }

    #[test]
    fn search_with_regex() {
        let haystack: String = String::from("2014-06-02\n1997-05-30\nRandom line\n2019-10-01");
        let pattern = String::from(r"^\d{4}-\d{2}-\d{2}$");
        match super::grep::match_pattern(&haystack, &pattern) {
            Ok(matches) => {
                for match_str in matches {
                    println!("{}", match_str);
                    assert_ne!(match_str, "Random line");
                }
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}
