#[cfg(test)]
mod tests {
    use crate::{about, ResultType, run};

    #[test]
    fn is_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_about() {
        let res = about();
        assert_eq!(res.type_result, ResultType::OK);
        assert!(res.message.is_some())
    }

    #[test]
    fn test_no_args() {
        let about_res = about();
        let res = run(vec!("Lol".to_string()));
        assert_eq!(about_res.type_result, res.type_result);
        assert_eq!(about_res.message, res.message);
    }

    #[test]
    fn test_no_file_path() {
        let res = run(vec!["Lol".to_string(), "-f".to_string()]);
        assert_eq!(ResultType::Fail, res.type_result);
        assert!(res.message.is_some());
    }
    #[test]
    fn test_read_line() {
        let res = run(vec!["Lol", "-f", "./test-texts/test.txt"]
            .iter().map(|&x|x.to_string()).collect());
        assert_eq!(ResultType::OK, res.type_result);
        assert!(res.message.is_some());
        assert_eq!("Hello, World!\n1\n", res.message.unwrap());
    }
}