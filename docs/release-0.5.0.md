# The Basic HUD 0.5.0 — Recent pickups

A fourteenth widget shows recently collected ground items. Collect four torches and then two more before the entry expires: the count becomes six and its timer starts again. Entries expire independently. The default is 5 seconds; the editor supports 1–60 seconds, dragging, scale, colours, opacity and icon + count mode. Reset asks for confirmation; unsaved duration changes trigger the existing Save / Discard / Stay dialog. The ten languages include the new controls.

The repository icon is now the selected pixel-art totem. Existing settings, widget indices, the v4 configuration format and previous release artifacts are preserved.

## Verification

- 75 strict Rust test executions passed: `-D warnings --locked --all-targets -j6`.
- The product mixin bundle applied and compiled against the pinned engine with warnings denied.
- An isolated world verified actual torch pickups, 4+2=6 aggregation, timer refresh and default expiry, no entry from give, icon + count mode, drag/scale persistence, a 1-second expiry and partial collection of only one torch from eight when the inventory had room for one.
- Duration persistence, the unsaved-changes dialog and confirmed reset to 5 seconds were verified in the editor.
- A separate ordinary startup without a controller reached the first frame and 60 ready-loop callbacks. The debugger then deliberately stopped only that test process. The controlled gameplay test exited normally.
- Archive CRC, all module/helper references and byte parity with the successfully compiled source input were checked. The official native-mod checker accepted the package.

External multiplayer with another collector and the deprecated in-frame engine path were not separately exercised. The integrated server's client packet path was exercised. All ten translation catalogs were structurally tested; the new controls were visually checked in English.

## Package

- File: `basic_hud-0.5.0.hnmod`
- Size: 82,091 bytes
- SHA-256: `97011cc7374354c6293678c7ab593240d0512bffde1d7b642dad521e1df3f8ac`
- 37 files; 12 modules; 23 injections; 2 bridges.
- Minecraft 26.3; Horizon `68ac6f647e7022b2c696e81863d7cface28c9416`; horizon-mixins `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`.

Install through Horizon's native-mod manager and choose **Performance** on this Windows baseline. This is a client-side visual mod. It cannot infer unobserved totem uses, unknown health or server-plugin-only cooldowns. No FPS improvement is claimed.
