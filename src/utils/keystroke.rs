use web_sys::KeyboardEvent;

#[derive(Clone, Debug, PartialEq)]
pub struct KeyStroke {
    key: String,
    ctrl: bool,
    alt: bool,
    shift: bool,
    meta: bool,
}
impl KeyStroke {
    pub fn from(s: &str) -> Self {
        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut meta = false;
        let mut key = String::new();
        for c in s.split('+') {
            match c {
                "Ctrl" => ctrl = true,
                "Alt" => alt = true,
                "Shift" => shift = true,
                "Win" => meta = true,
                _ => key.push_str(c),
            }
        }
        Self {
            key: key.to_string().to_uppercase(),
            ctrl,
            alt,
            shift,
            meta,
        }
    }
    pub fn matches(&self, e: &KeyboardEvent) -> bool {
        self.key.eq_ignore_ascii_case(e.key().as_str())
            && self.ctrl == e.ctrl_key()
            && self.alt == e.alt_key()
            && self.shift == e.shift_key()
            && self.meta == e.meta_key()
    }
}
