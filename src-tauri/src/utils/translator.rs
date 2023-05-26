use std::{
    fs,
    sync::{Mutex, MutexGuard},
};

use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use log::warn;
use tauri::{AppHandle, PathResolver, Wry};
use unic_langid::{langid, LanguageIdentifier};

const RES_IDS: [&str; 2] = ["back", "common"];

#[derive(Default)]
pub struct TranslatorState {
    pub translator: Mutex<Option<Translator>>,
}

impl TranslatorState {
    // Some functions are directly accessible here to ease the use of the translator
    pub fn tr(&self, key: &str) -> String {
        self.translate(key, None)
    }
    pub fn tra(&self, key: &str, args: &FluentArgs) -> String {
        self.translate(key, Some(args))
    }
    pub fn translate(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let translator = self.translator();
        let bundle_opt = translator.as_ref().unwrap().get_bundle_for_key(key);
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
    pub fn translator(&self) -> MutexGuard<'_, Option<Translator>> {
        self.translator.lock().unwrap()
    }
}

pub struct Translator {
    // locales: Vec<LanguageIdentifier>,
    bundles: Vec<TrBundle>,
}
type TrBundle = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

impl Translator {
    pub fn new(app: &tauri::App, app_language: Option<String>) -> Self {
        let requested: Vec<LanguageIdentifier> = vec![app_language
            .unwrap_or(sys_locale::get_locale().unwrap_or("en-US".into()))
            .parse()
            .unwrap()];
        let available = get_available_locales_strings(app.path_resolver())
            .iter()
            .map(|s| s.parse().expect("unable to parse locale directory name"))
            .collect::<Vec<LanguageIdentifier>>();

        let locales: Vec<LanguageIdentifier> = negotiate_languages(&requested, &available, Some(&langid!("en-US")), NegotiationStrategy::Filtering)
            .into_iter()
            .cloned()
            .collect();

        let bundles = locales
            .iter()
            .map(|locale| {
                let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
                for res_id in RES_IDS {
                    Self::load_ressource_to_bundle(app, &mut bundle, locale, res_id);
                }
                bundle
            })
            .collect::<Vec<TrBundle>>();

        Self { /* locales, */ bundles }
    }

    pub fn get_bundle_for_key(&self, key: &str) -> Option<&TrBundle> {
        self.bundles.iter().find(|bundle| bundle.has_message(&key))
    }

    fn load_ressource_to_bundle(app: &tauri::App, bundle: &mut TrBundle, locale: &LanguageIdentifier, res_id: &str) {
        let resource_path = app
            .path_resolver()
            .resolve_resource(format!("../translations/{}/{}.ftl", locale, res_id))
            .expect("failed to resolve language resource");

        bundle
            .add_resource(
                FluentResource::try_new(fs::read_to_string(&resource_path).expect("failed to load translation file"))
                    .expect("failed to parse translation file"),
            )
            .unwrap();
    }
}

#[tauri::command]
pub fn get_available_locales(app: AppHandle<Wry>) -> Vec<String> {
    get_available_locales_strings(app.path_resolver())
}
#[tauri::command]
pub fn get_system_locale() -> Option<String> {
    sys_locale::get_locale()
}

#[tauri::command]
pub fn get_translation_file(app: AppHandle<Wry>, locale: &str, resid: &str) -> String {
    let path = app
        .path_resolver()
        .resolve_resource(format!("../translations/{}/{}.ftl", locale, resid))
        .expect("failed to resolve language resource");

    fs::read_to_string(path).expect("failed to load translation file in command get_translation_file")
}

fn get_available_locales_strings(path_resolver: PathResolver) -> Vec<String> {
    let path = path_resolver
        .resolve_resource("../translations/")
        .expect("Failed to resolve translations directory");

    let res_dir = fs::read_dir(path).expect("failed to read translations directory");

    res_dir
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter_map(|dir| {
            let file_name = dir.file_name();
            let file_name = file_name.to_str()?;
            Some(file_name.to_string())
        })
        .collect()
}
