#[cfg(test)]
mod tests {
    use crate::{about};

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
/*
    #[test]
    fn test_no_args() {
        let about_res = about();
        assert!(about_res.is_ok());
        let expected_message = about_res.unwrap();
        assert!(!expected_message.is_empty());

        let res = run(vec!("Lol".to_string()));
        assert!(res.is_ok());
        let actual_message = res.unwrap();
        assert_eq!(expected_message, actual_message);
    }

    #[test]
    fn test_no_file_path() {
        let res = run(vec!["Lol".to_string(), "-f".to_string()]);
        assert!(res.is_err());
        assert!(!res.unwrap_err().is_empty());
    }
    #[test]
    fn test_read_line() {
        let res = run(vec!["Lol", "-f", "./test-texts/test.txt"]
            .iter().map(|&x|x.to_string()).collect());
        assert!(res.is_ok());
        let message = res.unwrap();
        assert!(!message.is_empty());
        assert_eq!("Hello, World!\n1\n", message);
    }*/
}