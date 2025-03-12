#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fluency {
    Limited,
    Fluent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Language {
    pub name: String,
}

impl Language {
    pub fn new(name: impl Into<String>) -> Self {
        Language { name: name.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageFluency(pub Language, pub Fluency);

impl LanguageFluency {
    pub fn common() -> Self {
        LanguageFluency(Language::new("Common"), Fluency::Fluent)
    }

    pub fn limited_in(language: Language) -> Self {
        LanguageFluency(language, Fluency::Limited)
    }

    pub fn fluent_in(language: Language) -> Self {
        LanguageFluency(language, Fluency::Fluent)
    }
}
