use std::collections::HashMap;
use fluent::{FluentBundle, FluentResource, FluentArgs, FluentValue};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use sys_locale::get_locales;
use unic_langid::LanguageIdentifier;

pub struct Localizer {
    bundle: FluentBundle<FluentResource>,
}

impl Localizer {
    pub fn new() -> Self {
        let translations = HashMap::from([
            ("en".to_string(), include_str!("../locales/en/main.ftl").to_string()),
            ("ru".to_string(), include_str!("../locales/ru/main.ftl").to_string()),
        ]);

        let locales: Vec<LanguageIdentifier> = get_locales()
            .filter_map(|l| {
                let s = l.to_string();
                let lang = s.split('-').next().unwrap_or("en");
                lang.parse().ok()
            })
            .collect();

        let available: Vec<LanguageIdentifier> = translations.keys()
            .filter_map(|s| s.parse().ok())
            .collect();

        let negotiated = negotiate_languages(
            &locales,
            &available,
            None,
            NegotiationStrategy::Filtering,
        );

        let lang = negotiated.first()
            .map(|l| l.to_string().split('-').next().unwrap_or("en").to_string())
            .unwrap_or_else(|| "en".to_string());

        let ftl_content = translations.get(&lang).unwrap_or(translations.get("en").unwrap());

        let res = FluentResource::try_new(ftl_content.clone())
            .expect("Failed to parse FTL content");

        let mut bundle = FluentBundle::default();
        bundle.add_resource(res).expect("Failed to add resource to bundle");

        Self { bundle }
    }

    pub fn translate(&self, key: &str) -> String {
        self.bundle.get_message(key)
            .and_then(|m| m.value())
            .map(|v| self.bundle.format_pattern(v, None, &mut Vec::new()).to_string())
            .unwrap_or_else(|| key.to_string())
    }

    pub fn translate_with_args(&self, key: &str, args: &FluentArgs) -> String {
        self.bundle.get_message(key)
            .and_then(|m| m.value())
            .map(|v| self.bundle.format_pattern(v, Some(args), &mut Vec::new()).to_string())
            .unwrap_or_else(|| key.to_string())
    }
}

impl Localizer {
    pub fn translate_with_map(&self, key: &str, args: &HashMap<String, String>) -> String {
        let mut fluent_args = FluentArgs::new();
        for (k, v) in args {
            let val: FluentValue = v.as_str().into();
            fluent_args.set(k.as_str(), val);
        }
        self.translate_with_args(key, &fluent_args)
    }
}