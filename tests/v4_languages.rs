#[path = "../mixins/basic_hud/i18n.rs"]
mod i18n;

#[test]
fn all_ten_languages_translate_settings_and_confirmation() {
    assert_eq!(i18n::LANGUAGES.len(),11);
    for &(code,name) in i18n::LANGUAGES.iter().skip(1) {
        assert!(!name.is_empty());
        assert!(!i18n::tr(code,"Save").is_empty());
        if code!="en_us" {
            assert_ne!(i18n::tr(code,"Save"),"Save", "missing Save in {code}");
            assert_ne!(i18n::tr(code,"Are you sure you want to reset?"),"Are you sure you want to reset?", "missing confirmation in {code}");
            assert_ne!(i18n::tr(code,"Icons + values"),"Icons + values", "missing icons control in {code}");
        }
        let rendered=i18n::fmt(code,"Players: {0}",&["27"]);
        assert!(rendered.contains("27"));
        assert!(!rendered.contains("{0}"));
    }
}

#[test]
fn language_variants_and_fallback_are_safe() {
    assert_eq!(i18n::tr("ru_RU","Save"),i18n::tr("ru_ru","Save"));
    assert_eq!(i18n::tr("pt_PT","Cancel"),i18n::tr("pt_br","Cancel"));
    assert_eq!(i18n::tr("en_GB","Save"),"Save");
    assert_eq!(i18n::tr("unknown","Save"),"Save");
    assert_eq!(i18n::tr("de_de","Custom Player: 27"),"Custom Player: 27");
    assert_eq!(i18n::next_language("auto",-1),"it_it");
    assert_eq!(i18n::next_language("it_it",1),"auto");
    assert_eq!(i18n::language_name("ru_ru"),"Русский");
}

#[test]
fn recent_pickup_settings_exist_in_all_translated_languages() {
    for &(code, _) in i18n::LANGUAGES.iter().skip(2) {
        for key in ["Recent pickups", "Display time:"] {
            assert_ne!(i18n::tr(code, key), key, "missing {key} in {code}");
        }
        assert!(i18n::fmt(code, "{0} s", &["5"]).contains('5'));
    }
}
