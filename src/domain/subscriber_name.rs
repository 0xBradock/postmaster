use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 32;
        let forbiden_chars = ['/', '(', ')', '[', ']', '{', '}', '<', '>', '\\'];
        let has_forbiden_chars = s.chars().any(|c| forbiden_chars.contains(&c));

        if is_empty || is_too_long || has_forbiden_chars {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn grapheme_long_name_valid() {
        let name = "ë".repeat(30);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn grapheme_too_long_name_invalid() {
        let name = "a".repeat(300);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn white_space_names_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_names_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn invlaid_chars_names_rejected() {
        for name in &['/', '(', ')', '[', ']', '{', '}', '<', '>', '\\'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn vlaid_names_rejected_successful() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
