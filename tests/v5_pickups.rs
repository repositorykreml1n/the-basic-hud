#[path = "../mixins/basic_hud/recent_pickups.rs"]
mod feed;
use std::time::Duration;
use horizon_mod_basic_hud::{Config, Widget, WIDGETS};
use feed::PickupFeed;

fn at(ms: u64) -> Duration { Duration::from_millis(ms) }

#[test]
fn accepted_amount_uses_only_the_confirmed_pickup_increase() {
    assert_eq!(feed::accepted_amount(60, 64, 16), 4);
    assert_eq!(feed::accepted_amount(60, 60, 16), 0);
    assert_eq!(feed::accepted_amount(64, 60, 16), 0);
    assert_eq!(feed::accepted_amount(0, 100, 16), 16);
}

#[test]
fn repeated_pickup_adds_and_refreshes_expiry() {
    let mut feed = PickupFeed::new();
    feed.add("torch", 4, at(0), at(5000));
    feed.add("torch", 2, at(4900), at(5000));
    feed.expire(at(5000), at(5000));
    assert_eq!(feed.entries().len(), 1);
    assert_eq!(feed.entries()[0].count, 6);
    feed.expire(at(9900), at(5000));
    assert!(feed.entries().is_empty());
}

#[test]
fn entries_expire_independently_and_old_totals_do_not_return() {
    let mut feed = PickupFeed::new();
    feed.add("torch", 4, at(0), at(5000));
    feed.add("dirt", 2, at(1000), at(5000));
    feed.expire(at(5000), at(5000));
    assert_eq!(feed.entries()[0].item, "dirt");
    feed.add("torch", 1, at(5001), at(5000));
    assert_eq!(feed.entries()[0].count, 1);
    feed.expire(at(6000), at(5000));
    assert_eq!(feed.entries().len(), 1);
}

#[test]
fn identities_include_components_and_feed_is_bounded() {
    let mut feed = PickupFeed::new();
    feed.add(("potion", "speed"), 1, at(0), at(5000));
    feed.add(("potion", "strength"), 1, at(1), at(5000));
    assert_eq!(feed.entries().len(), 2);
    let mut feed = PickupFeed::new();
    for i in 0..100 { feed.add(i, 1, at(i), at(5000)); }
    assert_eq!(feed.entries().len(), 64);
    assert_eq!(feed.entries()[0].item, 99);
    feed.clear();
    assert!(feed.entries().is_empty());
}

#[test]
fn zero_is_ignored_and_totals_saturate() {
    let mut feed = PickupFeed::new();
    feed.add("torch", 0, at(0), at(5000));
    assert!(feed.entries().is_empty());
    feed.add("torch", u64::MAX, at(1), at(5000));
    feed.add("torch", 1, at(2), at(5000));
    assert_eq!(feed.entries()[0].count, u64::MAX);
}

#[test]
fn configurable_duration_is_clamped_saved_and_reset() {
    let config = Config::default();
    assert_eq!(config.pickup_seconds, 5);
    assert_eq!(Widget::Pickups.index(), 13);
    assert_eq!(WIDGETS.len(), 14);
    assert!(config.enabled(Widget::Pickups));
    assert!(Widget::Pickups.supports_icons_only());
    let mut config = Config::parse(Some("v=4;pickup_seconds=17;w.fps=0,0.2,0.3,1,auto,70C9FF,10151D,60,1,3,0,1,0"));
    assert_eq!(config.pickup_seconds, 17);
    assert!(!config.enabled(Widget::Fps));
    assert_eq!(Config::parse(Some(&config.line())), config);
    config.reset_widget(Widget::Pickups);
    assert_eq!(config.pickup_seconds, 5);
    assert_eq!(Config::parse(Some("pickup_seconds=0")).pickup_seconds, 1);
    assert_eq!(Config::parse(Some("pickup_seconds=999")).pickup_seconds, 60);
    assert_eq!(Config::parse(Some("pickup_seconds=oops")).pickup_seconds, 5);
    assert_eq!(Config::parse(Some("v=4;fps_label=0")).pickup_seconds, 5);
}

#[test]
fn custom_expiry_and_backwards_clock_are_deterministic() {
    let mut feed = PickupFeed::new();
    feed.add("torch", 1, at(2000), at(1000));
    feed.expire(at(1000), at(1000));
    assert_eq!(feed.entries().len(), 1);
    feed.expire(at(2999), at(1000));
    assert_eq!(feed.entries().len(), 1);
    feed.expire(at(3000), at(1000));
    assert!(feed.entries().is_empty());
}
