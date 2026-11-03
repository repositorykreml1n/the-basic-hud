#[path = "../mixins/basic_hud/data_logic.rs"]
#[allow(dead_code)] // This integration test imports the full production helper module.
mod logic;

#[test]
fn fps_unit_and_small_scale_independently_remove_clutter() {
    assert_eq!(logic::format_fps(400, 147, 138, true, false, true, 1.0, false), "400 FPS");
    assert_eq!(logic::format_fps(400, 147, 138, false, false, true, 1.0, false), "400");
    let full = logic::format_fps(400, 147, 138, true, true, true, 1.0, false);
    assert!(full.contains("147") && full.contains("138"));
    assert_eq!(logic::format_fps(400, 147, 138, true, true, true, 0.6, false), "400 FPS");
    assert_eq!(logic::format_fps(400, 147, 138, false, true, true, 0.6, false), "400");
    assert!(logic::format_fps(400, 147, 138, true, true, false, 0.6, false).contains("147"));
}

fn tick_motion(fps: usize) -> Vec<f64> {
    let mut meter = logic::SpeedMeter::default();
    let dt = 1.0 / fps as f64;
    meter.sample(0.0, 0.0, dt);
    let mut steady = Vec::new();
    for frame in 1..=fps * 5 {
        let time = frame as f64 * dt;
        // Authoritative movement arrives at20ticks/s, independently of render FPS.
        let x = (time * 20.0 + 1e-8).floor() * 0.25;
        let value = meter.sample(x, 0.0, dt);
        if time >= 3.0 { steady.push(value); }
    }
    steady
}

#[test]
fn quantized_ticks_do_not_flicker_with_render_frames() {
    let mut means = Vec::new();
    for fps in [30, 60, 144, 240, 400, 1000] {
        let values = tick_motion(fps);
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = values.iter().copied().fold(0.0, f64::max);
        assert!((mean - 5.0).abs() < 0.4, "{fps}fps: mean={mean}");
        assert!(max - min < 0.8, "{fps}fps: visible speed swings {min}..{max}");
        means.push(mean);
    }
    assert!(means.iter().copied().fold(0.0, f64::max) - means.iter().copied().fold(f64::INFINITY, f64::min) < 0.3);
}

#[test]
fn speed_stops_and_never_turns_a_teleport_into_motion() {
    let mut meter = logic::SpeedMeter::default();
    for frame in 0..300 { meter.sample(frame as f64 / 12.0, 0.0, 1.0 / 60.0); }
    let x = 299.0 / 12.0;
    for _ in 0..120 { meter.sample(x, 0.0, 1.0 / 60.0); }
    assert!(meter.value() < 0.2, "stationary reading {}", meter.value());
    assert_eq!(meter.sample(1000.0, 1000.0, 1.0 / 60.0), 0.0);
    meter.reset();
    assert_eq!(meter.value(), 0.0);
}

#[test]
fn integrated_server_latency_is_always_local() {
    assert_eq!(logic::classify_ping(true, Some(67)), logic::PingStatus::Local);
    assert_eq!(logic::classify_ping(false, Some(67)), logic::PingStatus::Latency(67));
    assert_eq!(logic::classify_ping(false, Some(-1)), logic::PingStatus::Unknown);
    assert_eq!(logic::classify_ping(false, None), logic::PingStatus::Unknown);
}
