use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use log::warn;
use serde::{Deserialize, Serialize};
use unic_langid::{langid, LanguageIdentifier};

use super::{
    logger::info,
    utils::{cmd_async, cmd_async_get},
};

pub struct Translator {
    locales: Vec<LanguageIdentifier>,
    front_sources: Vec<String>,  // Necessary for Clone trait
    common_sources: Vec<String>, // Necessary for Clone trait
    bundles: Vec<TrBundle>,
}
type TrBundle = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

impl PartialEq for Translator {
    fn eq(&self, other: &Self) -> bool {
        self.locales == other.locales
    }
}

impl Clone for Translator {
    fn clone(&self) -> Self {
        info("Translator::clone");
        let bundles = self
            .locales
            .iter()
            .enumerate()
            .map(|(i, locale)| {
                let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
                bundle
                    .add_resource(FluentResource::try_new(self.front_sources[i].clone()).expect("failed to parse translation file"))
                    .unwrap();
                bundle
                    .add_resource(FluentResource::try_new(self.common_sources[i].clone()).expect("failed to parse translation file"))
                    .unwrap();
                bundle
            })
            .collect();
        Self {
            locales: self.locales.clone(),
            front_sources: self.front_sources.clone(),
            common_sources: self.common_sources.clone(),
            bundles,
        }
    }
}

impl Translator {
    pub async fn new(app_language: Option<String>) -> Self {
        let requested: Vec<LanguageIdentifier> = vec![app_language
            .unwrap_or(cmd_async_get::<Option<String>>("get_system_locale").await.unwrap_or("en-US".into()))
            .parse()
            .unwrap()];

        let available = cmd_async_get::<Vec<String>>("get_available_locales")
            .await
            .iter()
            .map(|s| s.parse().expect("unable to parse locale directory name"))
            .collect::<Vec<LanguageIdentifier>>();

        let locales: Vec<LanguageIdentifier> = negotiate_languages(&requested, &available, Some(&langid!("en-US")), NegotiationStrategy::Filtering)
            .into_iter()
            .cloned()
            .collect();

        let mut front_sources = vec![];
        let mut common_sources = vec![];
        let mut bundles = vec![];
        for locale in &locales {
            let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
            Self::load_ressource_to_bundle(&mut bundle, &mut front_sources, locale, "front").await;
            Self::load_ressource_to_bundle(&mut bundle, &mut common_sources, locale, "common").await;
            bundles.push(bundle);
        }
        Self {
            locales,
            front_sources,
            common_sources,
            bundles,
        }
    }

    async fn load_ressource_to_bundle(bundle: &mut TrBundle, sources: &mut Vec<String>, locale: &LanguageIdentifier, res_id: &str) {
        let content = cmd_async::<_, String>(
            "get_translation_file",
            &GetTranslationFileArgs {
                locale: locale.to_string().as_str(),
                resid: res_id,
            },
        )
        .await;
        sources.push(content.clone());

        bundle
            .add_resource(FluentResource::try_new(content).expect("failed to parse translation file"))
            .unwrap();
    }

    pub fn get_bundle_for_key(&self, key: &str) -> Option<&TrBundle> {
        self.bundles.iter().find(|bundle| bundle.has_message(&key))
    }
    pub fn tr(&self, key: &str) -> String {
        self.translate(key, None)
    }
    pub fn tra(&self, key: &str, args: &FluentArgs) -> String {
        self.translate(key, Some(args))
    }
    pub fn translate(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let bundle_opt = self.get_bundle_for_key(key);
        if let Some(bundle) = bundle_opt {
            let pattern = bundle.get_message(key).unwrap().value().unwrap();

            let mut errors = vec![];
            let result = bundle.format_pattern(&pattern, args, &mut errors).to_owned();
            if errors.len() > 0 {
                warn!("Error while formatting pattern: {:?}", errors);
            }
            result.to_string()
        } else {
            String::from("No translation found")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetTranslationFileArgs<'a> {
    pub locale: &'a str,
    pub resid: &'a str,
}
