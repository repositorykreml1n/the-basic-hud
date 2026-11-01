# The Basic HUD

A native mod for [Horizon](https://playhorizon.dev). In game and in the files it is still called Potion HUD (id `potion_hud`).

## What it shows

- Active effects with their time left. Potion effects are pinned by default; any other effect can be pinned on the mod's settings page, or all of them shown at once.
- Potions of each kind, golden apples, enchanted golden apples and totems in your inventory.
- Totems each player has used, above that player's name. Your own count is in the box.
- Durability of worn armour, elytra and a held shield.

The count of a player's totems resets when they die or when you leave the world.

## What it does not do

- It is not a content mod: a `.hmod` has no code, so a HUD needs a native `.hnmod`, which the launcher compiles into the game.
- It builds on one Horizon commit only, `68ac6f6` (Minecraft 26.3). Counting totems and writing above a player have no hook point, so it hooks two of the game's own functions. The rest uses hook points but reads the game's internal data.
- It knows only what the client is told: a totem used out of your sight is not counted.
- Texts of its own are in English. Effect and item names follow the game's language.
- Tested on Windows in single player and over LAN with two clients, not on a dedicated server.

## Build

Unit tests:

```
cargo test
```

Build the bundle into the game:

```
horizon-launcher native-build mixins/potion_hud --trust-native-code
```

Native mods run with your rights and are not sandboxed. Read the source before you build it.

## License

MIT. Not made or endorsed by the Horizon project, Mojang or Microsoft. It contains no Minecraft assets.
