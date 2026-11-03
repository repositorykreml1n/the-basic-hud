use horizon_mod_basic_hud::{Config, Widget};

#[test]
fn default_fps_coordinates_and_effects_do_not_overlap_on_small_gui() {
    let c = Config::default();
    let viewport = [320.0, 240.0];
    let fps = c.widget_rect(Widget::Fps, [70.0, 17.0], viewport);
    let xyz = c.widget_rect(Widget::Coordinates, [180.0, 28.0], viewport);
    let effects = c.widget_rect(Widget::Effects, [140.0, 17.0], viewport);
    assert!(fps.y + fps.h <= xyz.y, "FPS overlaps coordinates");
    assert!(xyz.y + xyz.h <= effects.y, "Coordinates overlap effects");
}

#[test]
fn new_preferences_and_detached_mini_position_survive_restart() {
    let mut c = Config::default();
    c.fps_label = false;
    c.fps_lows = true;
    c.fps_auto = false;
    c.tutorial_seen = true;
    c.mini_numbers = true;
    c.tab.opacity = 25;
    c.tab.sort = 2;
    c.tab.numeric_ping = false;
    c.tab.scale = 1.3;
    c.move_widget(Widget::ArmorMini, [30.0, 80.0], [62.0, 22.0], [640.0, 360.0], false);
    assert!(!c.mini_anchor);
    assert_eq!(Config::parse(Some(&c.line())), c);
}

#[test]
fn upgrade_moves_only_untouched_old_defaults() {
    let c = Config::parse(Some("v=2;corner=0;fps_lows=1;w.fps=1,0.99,0.15,1,auto,70C9FF,10151D,60,1,3,0,1;w.speed=1,0.99,0.31,1,auto,70C9FF,10151D,60,1,3,0,1"));
    assert!(c.widget(Widget::Fps).x < 0.1);
    assert!(!c.fps_lows);
    assert!(!c.enabled(Widget::Speed));
    let custom = Config::parse(Some("v=2;w.fps=1,0.4,0.6,1,auto,70C9FF,10151D,60,1,3,0,1"));
    assert_eq!(custom.widget(Widget::Fps).x, 0.4);
    assert_eq!(custom.widget(Widget::Fps).y, 0.6);
}

#[test]
fn fresh_hud_is_quiet_and_telemetry_starts_top_left() {
    let c = Config::default();
    assert!(!c.fps_lows, "min/1% should be an explicit option");
    assert!(!c.enabled(Widget::Speed), "speed must be opt-in");
    let fps = c.widget(Widget::Fps);
    let xyz = c.widget(Widget::Coordinates);
    assert!(fps.x < 0.1 && xyz.x < 0.1);
    assert!(fps.y < xyz.y && xyz.y < 0.2);
}
