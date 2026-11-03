use horizon_mod_basic_hud::{Config, Widget, WidgetConfig, ResetTarget, icon_grid};

#[test]
fn icon_only_roundtrip_preserves_other_widget_choices_and_legacy_styles() {
    let legacy="1,0.3,0.4,1.4,ABCDEF,123456,654321,25,0,5,1,0";
    let old=WidgetConfig::parse(legacy,&WidgetConfig::new(false,0.0,0.0));
    assert_eq!(old.scale,1.4);
    assert_eq!(old.text,0xABCDEF);
    assert!(!old.icons_only);
    let mut c=Config::parse(Some(&format!("v=3;future=retained;w.items={legacy}")));
    c.language="pl_pl".to_owned();
    c.widget_mut(Widget::Items).icons_only=true;
    c.widget_mut(Widget::Effects).icons_only=true;
    assert!(!c.widget(Widget::Totems).icons_only);
    assert_eq!(Config::parse(Some(&c.line())),c);
    assert_eq!(c.language("ru_ru"),"pl_pl");
    assert_eq!(Config::default().language("de_de"),"de_de");
}

#[test]
fn icon_strip_wraps_and_empty_data_does_not_allocate_a_fake_cell() {
    assert_eq!(icon_grid(0,10.0,3.0,70.0),(0,[0.0,0.0]));
    let (cols,one)=icon_grid(1,10.0,3.0,70.0);
    assert_eq!(cols,1);
    assert_eq!(one,[16.0,16.0]);
    let (cols,size)=icon_grid(9,10.0,3.0,70.0);
    assert_eq!(cols,5);
    assert!(size[0]<=70.0 && size[1]>16.0 && size[1]<40.0);
}

#[test]
fn reset_scope_keeps_independent_settings_and_language() {
    let mut c=Config::parse(Some("v=3;future=retained"));
    c.language="uk_ua".to_owned();
    c.widget_mut(Widget::Items).icons_only=true;
    c.widget_mut(Widget::Fps).scale=2.0;
    c.tab.opacity=10;
    c.reset_target(ResetTarget::Widget(Widget::Items));
    assert!(!c.widget(Widget::Items).icons_only);
    assert_eq!(c.widget(Widget::Fps).scale,2.0);
    assert_eq!(c.tab.opacity,10);
    c.reset_target(ResetTarget::Tab);
    assert_eq!(c.tab,Config::default().tab);
    assert_eq!(c.widget(Widget::Fps).scale,2.0);
    c.reset_target(ResetTarget::All);
    assert_eq!(c.widget(Widget::Fps).scale,1.0);
    assert_eq!(c.language,"uk_ua");
    assert!(c.unknown.contains(&("future".into(),"retained".into())));
}
