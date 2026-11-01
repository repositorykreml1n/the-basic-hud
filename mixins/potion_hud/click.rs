// A click on the mod's settings page, kept at once.
fn potion_hud_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::potion_hud as m;
    if mod_id != m::ID {
        return;
    }
    let line = m::change(|| game.native_setting(m::ID, "config"), |c| c.click(button));
    game.native_set_setting(m::ID, "config", &line);
}
