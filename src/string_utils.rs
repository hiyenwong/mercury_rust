pub mod string_utils {
    /**
     * string split
     */
    pub fn split_ext(separator: String, resource: &String) -> Vec<String> {
        let mut ret = Vec::new();
        let sep = separator.as_bytes();
        let res = resource.as_bytes();
        let mut cnt = 0;
        for (id, &item) in res.iter().enumerate() {
            if item == sep[0] {
                println!("{}", &resource[cnt..id]);
                ret.push(String::from(&resource[cnt..id]));
                cnt = id + 1;
            }
        }
        ret.push(String::from(&resource[cnt..resource.len()]));
        ret
    }

    // pub fn trim()
}

