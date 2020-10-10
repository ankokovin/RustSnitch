#[cfg(test)]
mod tests {
    use crate::{about};
    use assert_cmd::prelude::*;
    use assert_cmd::Command;
    use predicates::prelude::*;


    #[test]
    fn is_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_about() {
        let res = about();
        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());
    }

    #[test]
    fn test() {
        println!("{:?}", Command::new("ls").arg("-l"));
    }

    #[test]
    fn test_no_args() {
        let mut cmd = Command::cargo_bin("rust_snitch").unwrap();
        cmd.assert()
            .failure()
            .stderr( predicates::str::is_empty().not());
    }

    #[test]
    fn test_no_file_path() {
        let mut cmd = Command::cargo_bin("rust_snitch").unwrap();
        cmd.args(&["-f"])
            .assert()
            .failure()
            .stderr(predicates::str::contains("error"))
            .code(1);
    }

    #[test]
    fn test_aho_defaults() {
        let mut cmd = Command::cargo_bin("rust_snitch").unwrap();
        cmd.arg("-f")
            .arg("./test-texts/test.txt")

            .assert()
            .success()
            .stderr(predicates::str::is_empty())
            .stdout(
                predicates::str::contains("CommandMatch { line: \"TODO: Hello, World!\", line_idx: 0, file: \"./test-texts/test.txt\" }")
                    .and(
                        predicates::str::contains("CommandMatch { line: \"efsfse rgs TODO lol\", line_idx: 0, file: \"./test-texts/test.txt\" }")))
            .code(0);
    }
}