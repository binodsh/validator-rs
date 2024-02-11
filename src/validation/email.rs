use regex::Regex;

#[allow(dead_code)]
#[must_use]
pub fn validate_email(val: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    email_regex.is_match(val)
}

#[cfg(test)]
mod tests {
    use crate::validation::email::validate_email;


    #[test]
    fn test_email() {
        let tests = vec![
            ("email@here.com", true),
            ("weirder-email@here.and.there.com", true),
            ("example@valid-----hyphens.com", true),
            ("example@valid-with-hyphens.com", true),
            (r#""test@test"@example.com"#, false),
            (
                "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
                true,
            ),
            ("", false),
            ("abc", false),
            ("abc@", false),
            ("a @x.cz", false),
            ("abc@.com", false),
            ("something@@somewhere.com", false),
            ("example@invalid.com-", false),
            (r#"test@example.com\n\n<script src="x.js">"#, false),
            (r#""\\\011"@here.com"#, false),
            (r#""\\\012"@here.com"#, false),
            ("trailingdot@shouldfail.com.", false),
            // Trailing newlines in username or domain not allowed
            ("a@b.com\n", false),
            ("a\n@b.com", false),
            (r#""test@test"\n@example.com"#, false),
            ("a@[127.0.0.1]\n", false),
            // underscores are not allowed
            ("John.Doe@exam_ple.com", false),
        ];

        for (input, expected) in tests {
            // println!("{} - {}", input, expected);
            assert_eq!(
                validate_email(input),
                expected,
                "Email `{}` was not classified correctly",
                input
            );
        }
    }
}
