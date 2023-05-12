use std::{
    env,
    fs::{self},
    io,
    sync::{Mutex, MutexGuard}, ops::Deref,
};

use fluent::{bundle::{FluentBundle, self}, FluentResource, FluentArgs};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use log::warn;
use sys_locale::get_locale;
use unic_langid::{langid, LanguageIdentifier};

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
        }else{
            String::from("No translation found")
        }
    }
    pub fn translator(&self) -> MutexGuard<'_, Option<Translator>> {
        self.translator.lock().unwrap()
    }
}

pub struct Translator {
    lang: Vec<LanguageIdentifier>,
    bundles: Vec<TrBundle>,
}

type TrBundle = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

impl Translator {
    pub fn new(app: &tauri::App, app_language: Option<String>) -> Self {
        // let res_mgr = ResourceManager::new("../translations/{locale}/{res_id}".to_string());
        // let bundles = Localization::with_env(vec!["back.ftl".into(), "common.ftl".into()], true, locales.clone(), res_mgr)
        //     .bundles()
        //     .to_owned();

        let requested: Vec<LanguageIdentifier> = if app_language.is_some() {
            vec![app_language.unwrap_or("en-US".into()).parse().unwrap()]
        } else {
            vec![get_locale().unwrap_or("en-US".into()).parse().unwrap()]
        };

        let available = Self::get_available_locales(app).unwrap();

        let locales: Vec<LanguageIdentifier> = negotiate_languages(&requested, &available, Some(&langid!("en-US")), NegotiationStrategy::Filtering)
            .into_iter()
            .cloned()
            .collect();

        let bundles = locales
            .iter()
            .map(|locale| {
                let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
                Self::load_ressource_to_bundle(app, &mut bundle, locale, "back");
                Self::load_ressource_to_bundle(app, &mut bundle, locale, "common");
                bundle
            })
            .collect::<Vec<TrBundle>>();

        Self { lang: locales, bundles }
    }

    pub fn get_bundle_for_key(&self, key: &str) -> Option<&TrBundle>{
        self.bundles.iter().find(|bundle| bundle.has_message(&key))
    }

    fn load_ressource_to_bundle(app: &tauri::App, bundle: &mut TrBundle, locale: &LanguageIdentifier, res_id: &str) {
        let resource_path = app
            .path_resolver()
            .resolve_resource(format!("../translations/{}/{}.ftl", locale, res_id))
            .expect("Failed to resolve language resource");

        bundle
            .add_resource(
                FluentResource::try_new(fs::read_to_string(&resource_path).expect("Failed to load translation file"))
                    .expect("Failed to parse translation file"),
            )
            .unwrap();
    }

    fn get_available_locales(app: &tauri::App) -> io::Result<Vec<LanguageIdentifier>> {
        let path = app
            .path_resolver()
            .resolve_resource("../translations/")
            .expect("Failed to resolve translations directory");

        let res_dir = fs::read_dir(path)?;

        let loc = res_dir
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .filter_map(|dir| {
                let file_name = dir.file_name();
                let name = file_name.to_str()?;
                Some(name.parse().expect("Parsing failed."))
            })
            .collect();
        Ok(loc)
    }
}

#[tauri::command]
pub fn get_language(state: tauri::State<TranslatorState>) -> String {
    state.translator.lock().unwrap().as_ref().unwrap().lang[0].to_string()
}
