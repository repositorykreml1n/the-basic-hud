use horizon_mod_basic_hud::{Config,ResetTarget,Widget,icon_count,icon_value_layout,should_show_guide,mark_guide_seen,should_confirm_exit,GUIDE_SEEN_KEY};
use std::cell::RefCell;

#[test]
fn icon_readouts_keep_counts_and_exact_effect_time() {
    assert_eq!(icon_count(16),"×16");
    assert_eq!(icon_count(64),"×64");
    assert_eq!(horizon_mod_basic_hud::duration(105*20),"1:45");
    assert_eq!(horizon_mod_basic_hud::duration(-1),"**:**");
    assert_eq!(horizon_mod_basic_hud::durability_text(363,400),"363/400");
}

#[test]
fn value_cells_wrap_without_hiding_or_overlapping_numbers() {
    let (at,size)=icon_value_layout(&[37.0,55.0,31.0],10.0,3.0,102.0);
    assert_eq!(at,vec![[3.0,3.0],[43.0,3.0],[3.0,16.0]]);
    assert_eq!(size,[101.0,29.0]);
    for (i,pos) in at.iter().enumerate() {
        let width=[37.0,55.0,31.0][i];
        assert!(pos[0]+width <= size[0]-3.0);
        assert!(pos[1]+10.0 <= size[1]-3.0);
    }
    assert_eq!(icon_value_layout(&[],10.0,3.0,120.0),(vec![],[0.0,0.0]));
}

#[test]
fn guide_seen_is_independent_of_visual_save_cancel_and_reset() {
    assert_eq!(GUIDE_SEEN_KEY,"guide_seen");
    let storage=RefCell::new(None::<String>);
    let config=Config::default();
    assert!(should_show_guide(storage.borrow().as_deref(),config.tutorial_seen));
    assert!(mark_guide_seen(||storage.borrow().clone(), |v|*storage.borrow_mut()=Some(v.to_owned())));
    assert!(!should_show_guide(storage.borrow().as_deref(),config.tutorial_seen));
    let mut draft=config.clone();
    draft.reset_target(ResetTarget::All);
    assert!(!should_show_guide(storage.borrow().as_deref(),draft.tutorial_seen));
    assert_eq!(config,Config::default());
    assert!(!should_show_guide(None,true),"existing completed guide must migrate quietly");
    assert!(!mark_guide_seen(||None, |_|{}),"failed persistence must be reported");
}

#[test]
fn exit_confirmation_tracks_preferences_but_not_onboarding() {
    let initial=Config::default();
    let mut draft=initial.clone();
    assert!(!should_confirm_exit(&draft,&initial));
    draft.tutorial_seen=true;
    assert!(!should_confirm_exit(&draft,&initial),"only seeing help is not an unsaved HUD edit");
    draft.widget_mut(Widget::Items).icons_only=true;
    assert!(should_confirm_exit(&draft,&initial));
    draft.widget_mut(Widget::Items).icons_only=false;
    assert!(!should_confirm_exit(&draft,&initial),"reverted edits must not prompt");
    draft.language="de_de".into();
    assert!(should_confirm_exit(&draft,&initial));
}
