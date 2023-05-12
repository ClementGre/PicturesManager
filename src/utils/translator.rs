use std::{rc::Rc, io, env, fs};

use fluent_fallback::{Bundles, Localization};
use fluent_resmgr::ResourceManager;
use unic_langid::{langid, LanguageIdentifier};

pub struct Translator {
    lang: LanguageIdentifier,
    bundles: Rc<Bundles<ResourceManager>>,
}

impl Translator {
    pub fn new(lang: LanguageIdentifier) -> Self {
        let res_mgr = ResourceManager::new("../translations/{locale}/{res_id}".to_string());

        let localization = Localization::with_env(
            vec!["front.ftl".into()],
            true,
            vec![langid!("fr-FR")],
            res_mgr,
        );
        let bundles = localization.bundles();

        Self {
            lang,
            bundles: bundles.clone(),
        }
    }

    fn get_available_locales() -> io::Result<Vec<LanguageIdentifier>> {
        let mut dir = env::current_dir()?;
        dir.push("../translations");
        let res_dir = fs::read_dir(dir)?;

        let locales = res_dir
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .filter_map(|dir| {
                let file_name = dir.file_name();
                let name = file_name.to_str()?;
                Some(name.parse().expect("Parsing failed."))
            })
            .collect();
        Ok(locales)
    }
}