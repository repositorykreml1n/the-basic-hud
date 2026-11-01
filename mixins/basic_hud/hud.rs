// The box: effects, then what the player carries, then what is wearing out.
fn basic_hud_draw(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::basic_hud as m;
    use horizon_client::{extras::Corner, items::ItemStack, lang};

    enum Icon {
        Effect(String),
        Item(ItemStack),
    }
    const ROW: f32 = 11.0;
    const ICON: f32 = 10.0;
    const CORNERS: [Corner; 4] =
        [Corner::TopLeft, Corner::TopRight, Corner::BottomLeft, Corner::BottomRight];

    if !game.native_on(m::ID) || game.player.is_none() {
        return;
    }
    let config = m::config(|| game.native_setting(m::ID, "config"), m::Config::clone);
    let mut rows: Vec<(Icon, String, u32)> = Vec::new();

    if config.effects {
        for e in &game.potions.effects.list {
            if !config.shows_effect(e.id, e.ambient) {
                continue;
            }
            let mut name = lang::get(&format!("effect.minecraft.{}", e.id));
            if e.amplifier > 0 {
                name = format!("{name} {}", lang::get(&format!("potion.potency.{}", e.amplifier)));
            }
            let text = format!("{name} {}", m::duration(e.duration));
            let sprite = format!("mob_effect/{}", e.id.rsplit(':').next().unwrap_or(e.id));
            rows.push((Icon::Effect(sprite), text, m::effect_colour(e.id, e.duration)));
        }
    }
    if config.items {
        let mut tally = m::Tally { rows: Vec::new() };
        let carried = game.inventory.iter().take(36).chain(std::iter::once(&game.offhand));
        for s in carried.flatten() {
            let potion = s.item.ends_with("potion");
            if potion || [m::GOLDEN_APPLE, m::ENCHANTED_APPLE, m::TOTEM_ITEM].contains(&&*s.item) {
                let mut one = s.clone();
                one.count = 1;
                tally.add((one.item.clone(), one.potion().map(str::to_owned), one), s.count);
            }
        }
        for ((_, _, stack), count) in tally.rows {
            let name = game.stack_display_name(&stack).plain();
            let text = format!("{} x{count}", name.trim_matches(['[', ']']));
            rows.push((Icon::Item(stack), text, m::INK));
        }
    }
    if config.totems {
        let name = game.own_profile_name();
        let count = m::pops(|p| p.get(&name));
        if count > 0 {
            let totem = ItemStack::new(m::TOTEM_ITEM, 1);
            rows.push((Icon::Item(totem), format!("{}: {count}", m::POPPED), m::TOTEM));
        }
    }
    if config.durability {
        let shield = |s: &&ItemStack| &*s.item == "minecraft:shield";
        let worn = game.armor.iter().flatten();
        let held = game.held().filter(shield).or(game.offhand.as_ref().filter(shield));
        for s in worn.chain(held) {
            let max = game.res.items().get(&s.item).map_or(0, |i| i.max_damage);
            if max > 0 {
                let left = max.saturating_sub(s.damage);
                rows.push((
                    Icon::Item(s.clone()),
                    m::durability_text(left, max),
                    m::durability_colour(left, max),
                ));
            }
        }
    }
    if rows.is_empty() {
        return;
    }
    let Some(corner) = (0..4)
        .map(|i| CORNERS[(config.corner + i) % 4])
        .find(|c| game.native_corner_shown(*c))
    else {
        return;
    };
    let text = rows.iter().map(|r| game.native_text_width(&r.1)).fold(0.0, f32::max);
    let w = ICON + text + 5.0;
    let h = ROW * rows.len() as f32 + 2.0;
    let (x, y) = corners.place(corner, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, (icon, line, rgb)) in rows.iter().enumerate() {
        let top = y + 1.0 + ROW * i as f32;
        match icon {
            Icon::Effect(sprite) => {
                if let Some([sw, sh]) = game.native_sprite_size(sprite) {
                    game.native_sprite(batch, sprite, [0.0, 0.0, sw, sh], [x + 1.0, top, ICON, ICON]);
                }
            }
            Icon::Item(stack) => {
                let start = batch.vertices.len();
                game.draw_item_model(batch, stack, x + 1.0, top);
                batch.scale_since(start, x + 1.0, top, ICON / 16.0, ICON / 16.0);
            }
        }
        game.native_text(batch, line, [x + ICON + 3.0, top + 1.5], *rgb, true);
    }
}
