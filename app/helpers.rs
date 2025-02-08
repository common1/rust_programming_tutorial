pub mod namehelpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        return full_name;
    }    
}

pub mod database {

}

pub mod private_fns {
    pub fn get_age_plus_5(age: u16)  -> u16 {
        return age + 5;
    }
}