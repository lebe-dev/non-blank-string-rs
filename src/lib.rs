use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::StringValueError;

pub mod error;

#[cfg(feature = "utils")]
pub mod utils;

pub type RequestId = NonBlankString;

#[derive(Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash, Clone, Debug)]
#[serde(try_from = "String", into = "String")]
pub struct NonBlankString(String);

impl FromStr for NonBlankString {
    type Err = StringValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() > 0 {
            Ok(Self(value.to_string()))
        } else {
            Err(StringValueError::ParseError)
        }
    }
}

impl AsRef<str> for NonBlankString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for NonBlankString {
    type Error = StringValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NonBlankString::from_str(&value)
    }
}

impl From<NonBlankString> for String {
    fn from(value: NonBlankString) -> Self {
        value.0
    }
}

impl Deref for NonBlankString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for NonBlankString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use fake::{Fake, Faker};
    use serde::{Deserialize, Serialize};

    use crate::NonBlankString;

    #[test]
    fn return_string_value_for_non_blank_string() {
        let value = get_random_string();
        assert_eq!(value, NonBlankString::from_str(&value).unwrap().as_ref());
    }

    #[test]
    fn return_error_for_blank_string() {
        assert!(NonBlankString::from_str("").is_err());

        let value = "".parse::<NonBlankString>();
        assert!(value.is_err())
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Demo {
        pub login: NonBlankString,
    }

    #[test]
    fn serialization_deserialization_test() {
        let value = get_random_string();

        let entity = Demo {
            login: NonBlankString::from_str(&value).unwrap(),
        };

        let json = serde_json::to_string(&entity).unwrap();

        let expected_json = format!("{{\"login\":\"{}\"}}", value);

        assert_eq!(json, expected_json);

        let result_entity = serde_json::from_str::<Demo>(&expected_json).unwrap();

        assert_eq!(result_entity, entity);
    }

    #[test]
    fn serialization_deserialization_test_for_string_and_blank_value() {
        let json = "{\"login\":\"\"}".to_string();
        match serde_json::from_str::<Demo>(&json) {
            Ok(_) => panic!("error expected"),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn use_as_str_test() {
        let non_blank_string = NonBlankString::from_str(&get_random_string()).unwrap();
        assert_str_func(&non_blank_string);
    }

    fn assert_str_func(_: &str) {
        assert!(true)
    }

    fn get_random_string() -> String {
        Faker.fake::<String>()
    }
}
