use nanoid::nanoid;
use url::{Url};

pub fn generate_short_id() -> String {
    nanoid!(10, &nanoid::alphabet::SAFE)
}

pub fn check_url(url_str: &str) -> bool {
    let res = Url::parse(
        url_str
    );

    match res {
        Ok(_) => {
            true
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_check_url_with_valid_url() {
        let url_str = "https://sample.com";
        let res = check_url(url_str);
        assert_eq!(res, true);
    }

    #[test]
    fn test_check_url_with_invalid_url() {
        let url_str = "fdkdjshfkds";
        let res = check_url(url_str);
        assert_eq!(res, false);
    }

    #[test]
    fn test_generate_short_id() {
        let short_id = generate_short_id();
        assert_eq!(short_id.len(), 10);
    }
}