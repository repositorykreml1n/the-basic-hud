// A line above the name of each player who has used a totem, drawn as the name
// is and only on a tag the game decided to show (`name_tags` holds a macro, so
// horizon-mixins cannot add the line there).
fn basic_hud_name_tags(
    game: &mut crate::game::Game<'_>,
    camera: glam::DVec3,
    alpha: f64,
    out: &mut Vec<Vertex>,
    see_through: &mut Vec<Vertex>,
) {
    use crate::game::basic_hud as m;
    const FADED: u32 = 0x7F;
    if !game.native_on(m::ID) || !m::config(|| game.native_setting(m::ID, "config"), |c| c.totems) {
        return;
    }
    let counted: Vec<_> = game
        .other_players()
        .into_iter()
        .chain(game.own_name_tag())
        .filter_map(|(mob, name)| {
            let count = m::pops(|p| p.get(&name));
            (count > 0).then(|| (name_tag_at(mob, alpha), count))
        })
        .collect();
    if counted.is_empty() {
        return;
    }
    let tags = game.name_tags(camera, alpha);
    let mut patches = Vec::new();
    for (at, count) in counted {
        let Some(tag) = tags.iter().find(|t| t.at.distance_squared(at) < 1e-6) else {
            continue;
        };
        let o = tag.at - camera;
        let lift = tag.lines.last().map_or(0.0, |l| l.1) + f64::from(9.0f32 * 1.15 * 0.025);
        let lines = [(Text::colored(format!("{}: {count}", m::POPPED), m::TOTEM), lift)];
        if tag.discrete {
            game.mark_translucent_entity(out.len(), "minecraft:name_tag");
            let light = vertex_light(tag.light);
            patches.extend(game.name_tag_quads(o, &lines, FADED, true, 0, light, NAME_TAG_UNIT, out));
            continue;
        }
        let lit = (tag.light >> 4).max(2) << 4 | (tag.light & 15).max(2);
        game.mark_entity(out.len(), "minecraft:name_tag");
        let light = vertex_light(lit);
        patches.extend(game.name_tag_quads(o, &lines, 0xFF, false, 0, light, NAME_TAG_UNIT, out));
        patches.extend(game.name_tag_quads(o, &lines, FADED, true, 2, 0, NAME_TAG_UNIT, see_through));
    }
    game.map_patches.extend(patches);
}
