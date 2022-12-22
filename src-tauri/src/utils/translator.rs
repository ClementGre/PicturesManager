use std::{env, fs, io, sync::Mutex};

use fluent::{bundle::FluentBundle, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use sys_locale::get_locale;
use unic_langid::{langid, LanguageIdentifier};

#[derive(Default)]
pub struct TranslatorState {
    pub translator: Mutex<Option<Translator>>,
}

pub struct Translator {
    pub lang: LanguageIdentifier,
    pub bundles: TranslationType
}

type TranslationType = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

impl Translator {
    pub fn new(app_language: Option<String>) -> Self{
        //let res_mgr = ResourceManager::new("../translations/{locale}/{res_id}".to_string());

        let requested: Vec<LanguageIdentifier> = if app_language.is_some() {
            vec![app_language
                .unwrap()
                .parse()
                .unwrap_or(String::from("en-US").parse::<LanguageIdentifier>().unwrap())]
        } else {
            vec![get_locale()
                .unwrap_or("en-US".into())
                .parse()
                .unwrap_or(String::from("en-US").parse::<LanguageIdentifier>().unwrap())]
        };

        let available = Self::get_available_locales().unwrap();

        let locales: Vec<LanguageIdentifier> = negotiate_languages(
            &requested,
            &available,
            Some(&langid!("en-US")),
            NegotiationStrategy::Filtering,
        )
        .into_iter()
        .cloned()
        .collect();

        // let bundles = Localization::with_env(
        //     vec!["back.ftl".into(), "common.ftl".into()],
        //     true,
        //     locales.clone(),
        //     res_mgr,
        // )
        // .bundles()
        // .to_owned();


        let mut bundles = FluentBundle::new_concurrent(locales.clone());
        bundles
            .add_resource(
                FluentResource::try_new(String::from(include_str!(
                    "../../../translations/en-US/back.ftl"
                )))
                .unwrap(),
            )
            .unwrap();

        Self { lang: locales[0].clone(), bundles }
    }

    fn get_available_locales() -> io::Result<Vec<LanguageIdentifier>> {
        let mut dir = env::current_dir()?;
        dir.push("../translations");
        let res_dir = fs::read_dir(dir)?;

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
    state
        .translator
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .lang
        .to_string()
}
