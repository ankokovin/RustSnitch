use assert_cmd::Command;
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
    fn test_read_line() {
        let mut cmd = Command::cargo_bin("rust_snitch").unwrap();
        cmd.arg("-f")
            .arg("./test-texts/test.txt")

            .assert()
            .success()
            .stderr(predicates::str::is_empty())
            .stdout("Hello, World!\n1\n\n")
            .code(0);
    }
}