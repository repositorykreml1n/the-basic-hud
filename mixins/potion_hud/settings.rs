// The mod's settings page: its parts, then a pin for each effect.
fn potion_hud_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::potion_hud as m;
    if mod_id != m::ID {
        return;
    }
    let name = |id: &str| horizon_client::lang::get(&format!("effect.minecraft.{id}"));
    buttons.extend(m::config(|| game.native_setting(m::ID, "config"), |c| m::labels(c, name)));
}
