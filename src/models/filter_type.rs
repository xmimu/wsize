use std::any::Any;

pub trait MatchBank: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn matches(&self) -> bool;
}

pub struct NameFilter {
    pattern: String,
    case_sensitive: bool,
    use_regex: bool,
    filter_banks: bool,
    filter_media: bool,
}

impl NameFilter {
    pub fn new() -> Self {
        Self {
            pattern: "*".to_string(),
            case_sensitive: false,
            use_regex: false,
            filter_banks: true,
            filter_media: true,
        }
    }
}

impl MatchBank for NameFilter {
    fn matches(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct LanguageFilter {
    pub languages: Vec<String>,
}

impl LanguageFilter {
    pub fn new() -> Self {
        Self { languages: vec![] }
    }
}

impl MatchBank for LanguageFilter {
    fn matches(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
