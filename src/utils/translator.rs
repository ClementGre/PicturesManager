use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use log::warn;
use serde::{Deserialize, Serialize};
use unic_langid::{langid, LanguageIdentifier};
use yewdux::store::Store;

use super::utils::{cmd_async, cmd_async_get};

const RES_IDS: [&str; 2] = ["menu-bar", "common"];

#[derive(Store)]
pub struct Translator {
    locales: Vec<LanguageIdentifier>,
    bundles: Vec<TrBundle>,
}
type TrBundle = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

impl PartialEq for Translator {
    fn eq(&self, other: &Self) -> bool {
        self.locales == other.locales
    }
}
impl Default for Translator {
    fn default() -> Self {
        Self {
            locales: vec![],
            bundles: vec![],
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

        let mut bundles = vec![];
        for locale in &locales {
            let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
            for res_id in RES_IDS {
                Self::load_ressource_to_bundle(&mut bundle, locale, res_id).await;
            }
            bundles.push(bundle);
        }
        Self { locales, bundles }
    }

    async fn load_ressource_to_bundle(bundle: &mut TrBundle, locale: &LanguageIdentifier, res_id: &str) {
        let content = cmd_async::<_, String>(
            "get_translation_file",
            &GetTranslationFileArgs {
                locale: locale.to_string().as_str(),
                resid: res_id,
            },
        )
        .await;

        bundle
            .add_resource(FluentResource::try_new(content).expect("failed to parse translation file"))
            .unwrap();
    }

    pub fn get_bundle_for_key(&self, key: &str) -> Option<&TrBundle> {
        self.bundles.iter().find(|bundle| bundle.has_message(&key))
    }
    #[allow(dead_code)]
    pub fn tr(&self, key: &str) -> String {
        self.translate(key, None)
    }
    #[allow(dead_code)]
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
            String::from(key)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetTranslationFileArgs<'a> {
    pub locale: &'a str,
    pub resid: &'a str,
}
