#[cfg(test)]
pub use mercury_rust::string_utils;
mod test {
    use mercury_rust::string_utils;

    #[test]
    fn it_test() {
        let sp = String::from(",");
        let re = String::from("hello, world");
        let ret = string_utils::string_utils::split_ext(sp,&re);
        let c = vec!["hello", " world"];
        assert_eq!(ret,c);
    }

}