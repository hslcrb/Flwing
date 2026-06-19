pub mod locale;

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n::locale::get_string($key)
    };
}
