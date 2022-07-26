pub mod error;

use crate::error::StringValueError;

use serde::{Serialize, Deserialize};

pub type RequestId = NonBlankString;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NonBlankString(String);

impl NonBlankString {
    pub fn parse(s: &str) -> Result<NonBlankString, StringValueError> {
        if s.len() > 0 {
            Ok(Self(s.to_string()))

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

impl From<&str> for NonBlankString {
    fn from(value: &str) -> Self {
        NonBlankString::parse(value).unwrap()
    }
}

#[cfg(test)]
mod non_blank_string_tests {
    use crate::NonBlankString;

    use serde::{Serialize, Deserialize};

    #[test]
    fn return_string_value_for_non_blank_string() {
        let value = "KREATOR";
        assert_eq!(value, NonBlankString::parse(value).unwrap().as_ref());
    }

    #[test]
    fn return_error_for_blank_string() {
        assert!(NonBlankString::parse("").is_err())
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Demo {
        pub login: NonBlankString
    }

    #[test]
    fn serialization_deserialization_test() {
        let entity = Demo {
          login: NonBlankString::parse("value").unwrap()
        };

        let json = serde_json::to_string(&entity).unwrap();

        let expected_json = "{\"login\":\"value\"}";

        assert_eq!(json, expected_json);

        let result_entity = serde_json::from_str::<Demo>(expected_json).unwrap();

        assert_eq!(result_entity, entity);
    }
}
