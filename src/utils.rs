use std::str::FromStr;

use fake::{Fake, Faker};

use crate::NonBlankString;

pub fn get_random_nonblank_string() -> NonBlankString {
    let value = Faker.fake::<String>();
    NonBlankString::from_str(&value).expect("unexpected blank string value")
}

#[cfg(test)]
mod tests {
    use crate::utils::get_random_nonblank_string;

    #[test]
    fn should_return_random_values() {
        let value1 = get_random_nonblank_string();
        let value2 = get_random_nonblank_string();
        let value3 = get_random_nonblank_string();

        assert_ne!(value1, value2);
        assert_ne!(value2, value3);
        assert_ne!(value3, value1);
    }
}
