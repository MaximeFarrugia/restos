use async_graphql::{CustomValidator, InputValueError};

pub struct PasswordValidator;

impl CustomValidator<String> for PasswordValidator {
    fn check(&self, value: &String) -> Result<(), InputValueError<String>> {
        let mut issues = Vec::<String>::new();

        if value.len() < 8 {
            issues.push(String::from("8 characters"));
        }
        if !value.chars().any(|x| x.is_ascii_lowercase()) {
            issues.push(String::from("1 lowercase"));
        }
        if !value.chars().any(|x| x.is_ascii_uppercase()) {
            issues.push(String::from("1 uppercase"));
        }
        if !value.chars().any(|x| x.is_ascii_punctuation()) {
            issues.push(String::from("1 special character"));
        }

        if issues.len() == 0 {
            Ok(())
        } else {
            Err(InputValueError::custom(format!(
                "Expected at least {}",
                issues.join(", ")
            )))
        }
    }
}
