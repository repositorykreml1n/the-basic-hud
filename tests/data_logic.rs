// Exercise the actual dependency-free data helpers that the mixin uses.
// Kept outside the bundle so the game's own test-module path resolution is unchanged.
#[path = "../mixins/basic_hud/data_logic.rs"]
pub mod logic;

#[test]
fn selected_text_color_overrides_status_colors_and_default_keeps_them() {
    assert_eq!(logic::widget_text_color(0x123456, logic::GOOD), 0x123456);
    assert_eq!(logic::widget_text_color(logic::AUTO_TEXT, logic::BAD), logic::BAD);
    assert_eq!(logic::widget_text_color(logic::INK, logic::BAD), logic::INK);
    assert_eq!(logic::AUTO_TEXT, horizon_mod_basic_hud::WidgetConfig::DEFAULT_TEXT);
}
