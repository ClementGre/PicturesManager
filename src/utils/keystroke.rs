use web_sys::KeyboardEvent;

use crate::app::MACOS;

#[derive(Clone, Debug, PartialEq)]
pub struct KeyStroke {
    key: String,
    shortcut: bool,
    ctrl: bool,
    alt: bool,
    shift: bool,
    meta: bool,
}
impl KeyStroke {
    pub fn from(s: &str) -> Self {
        let mut shortcut = false;
        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut meta = false;
        let mut key = String::new();
        for c in s.split('+') {
            match c {
                "shortcut" => shortcut = true,
                "ctrl" => ctrl = true,
                "alt" => alt = true,
                "shift" => shift = true,
                "meta" => meta = true,
                _ => key.push_str(c),
            }
        }
        Self {
            key: key.to_string().to_uppercase(),
            shortcut,
            ctrl,
            alt,
            shift,
            meta,
        }
    }
    pub fn to_string(&self) -> String {
        let is_macos = *MACOS.get().unwrap();
        let mut s = String::new();
        if self.ctrl {
            if is_macos {
                s.push('⌃');
            } else {
                s.push_str("Ctrl+");
            }
        }
        if self.alt {
            if is_macos {
                s.push('⌥');
            } else {
                s.push_str("Alt+");
            }
        }
        if self.shift {
            if is_macos {
                s.push('⇧');
            } else {
                s.push_str("Shift+");
            }
        }
        if self.shortcut {
            if is_macos {
                s.push('⌘');
            } else {
                s.push_str("Ctrl+");
            }
        }
        if self.meta {
            if is_macos {
                s.push('⌘');
            } else {
                s.push_str("Win+");
            }
        }
        s.push_str(&self.key.to_uppercase());
        s
    }
    pub fn matches(&self, e: &KeyboardEvent) -> bool {
        let key = e.key();
        let ctrl = e.ctrl_key();
        let alt = e.alt_key();
        let shift = e.shift_key();
        let meta = e.meta_key();

        if *MACOS.get().unwrap() {
            return self.key.eq_ignore_ascii_case(key.as_str())
                && self.ctrl == ctrl
                && self.alt == alt
                && self.shift == shift
                && self.shortcut == meta;
        }else{
            return self.key.eq_ignore_ascii_case(key.as_str())
                && (self.ctrl == false || self.ctrl == ctrl)
                && self.alt == alt
                && self.shift == shift
                && self.shortcut == ctrl;
        }
    }
}
