use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Option<Self> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 32;
        let forbiden_chars = ['/', '(', ')', '[', ']', '{', '}', '<', '>', '\\'];
        let has_forbiden_chars = s.chars().any(|c| forbiden_chars.contains(&c));

        if is_empty || is_too_long || has_forbiden_chars {
            None
        } else {
            Some(Self(s))
        }
    }

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.0
    }

    pub fn inner_ref(&self) -> &String {
        &self.0
    }
}
