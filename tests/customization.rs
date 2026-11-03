use horizon_mod_basic_hud::Config;

#[test]
fn independent_widget_transform_and_style_survive_restart_serialization() {
    let row = "w.coordinates=1,0.25,0.75,1.5,FFFFFF,70C9FF,10151D,55,1,3,0,1";
    let stored = format!("v=2;effects=0;all=1;pinned=haste;{row}");
    let first = Config::parse(Some(&stored));
    assert!(!first.effects);
    assert!(first.shows_effect("haste", true));
    let saved = first.line();
    assert!(saved.split(';').any(|part| part == format!("{row},0")), "lost widget settings: {saved}");
    assert_eq!(Config::parse(Some(&saved)), first);
}

#[test]
fn independent_widgets_keep_different_positions_and_styles() {
    let a = "w.coordinates=1,0.25,0.75,1.5,FFFFFF,70C9FF,10151D,55,1,3,0,1";
    let b = "w.fps=0,0.9,0.1,2,FFAA00,00FF00,000000,0,0,0,1,0";
    let saved = Config::parse(Some(&format!("v=2;{a};{b}"))).line();
    assert!(saved.split(';').any(|part| part == format!("{a},0")), "coordinates lost: {saved}");
    assert!(saved.split(';').any(|part| part == format!("{b},0")), "FPS lost: {saved}");
}

#[test]
fn future_key_survives_editing_existing_preferences() {
    let mut config = Config::parse(Some("future_widget_setting=keep_this;effects=1"));
    config.effects = false;
    let saved = config.line();
    assert!(saved.split(';').any(|part| part == "future_widget_setting=keep_this"));
    assert!(!Config::parse(Some(&saved)).effects);
}

#[test]
fn snapping_chooses_the_nearest_edge_when_a_widget_almost_fills_the_screen() {
    use horizon_mod_basic_hud::{WidgetConfig, move_to, place};
    let mut widget = WidgetConfig::default();
    move_to(&mut widget, [10.0, 0.0], [100.0, 10.0], [110.0, 30.0], true);
    assert_eq!(place(&widget, [100.0, 10.0], [110.0, 30.0]).x, 10.0);
}

#[test]
fn settings_labels_tolerate_an_out_of_range_corner() {
    let mut config = Config::default();
    config.corner = usize::MAX;
    let labels = horizon_mod_basic_hud::labels(&config, str::to_owned);
    assert!(labels[5].ends_with("bottom right"));
}

#[test]
fn an_oversized_widget_keeps_its_alignment_when_dragged() {
    use horizon_mod_basic_hud::{WidgetConfig, move_to};
    let mut widget = WidgetConfig::new(true, 0.7, 0.3);
    move_to(&mut widget, [0.0, 0.0], [1000.0, 1000.0], [320.0, 180.0], false);
    assert_eq!([widget.x, widget.y], [0.7, 0.3]);
}

#[test]
fn the_longest_server_effect_duration_does_not_overflow() {
    assert_eq!(horizon_mod_basic_hud::duration(i32::MAX), "29826:09:43");
}
