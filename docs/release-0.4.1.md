# The Basic HUD 0.4.1 — Your HUD, your layout

A major upgrade from the registry's 0.1.0: a fully visual, client-side HUD editor with 13 independent widgets.

- **Arrange it in-game.** Drag widgets over the world; resize each one with the wheel or controls. Tune text/accent/background colours, opacity, padding and shadows, with presets and snap alignment.
- **Keep the essentials.** Icons + values removes names while retaining counts and timers: pearls ×16, effects 1:45, item cooldowns, and durability. Each widget has its own presentation.
- **FPS and coordinates.** FPS starts at the top left with coordinates below it. Hide the FPS label, enable min/1% low details, or automatically hide details at small scales.
- **Equipment by the hotbar.** Mini armour shows equipped pieces, including elytra, with durability bars or compact icon/value readouts. Keep the separate numeric durability widget too.
- **A more useful Tab list.** Coloured numeric ping, optional bars, player counts, your-player marker, sorting, scale and opacity. Server headers, footers and scoreboard data are preserved.
- **More client information.** Item cooldowns, available player-health data, consumable counts, effects and observed totem uses; optional speed, CPS and world clock.
- **Calmer speed readings.** Speed uses a measurement window; it is disabled by default. Single-player ping says Local instead of presenting integrated-server latency as network ping.
- **Ten languages.** English, Russian, Ukrainian, German, French, Spanish, Portuguese, Polish, Turkish and Italian, plus automatic selection.
- **Help when needed.** A short guide appears on the first settings visit, then only through ?. Resetting or discarding visual edits does not restart it.
- **Safer editing.** Reset asks for confirmation. Leaving with edits offers Save / Discard changes / Stay. Clean and reverted drafts close directly; a failed save retains the draft and shows an error.

## Compatibility and installation

Minecraft 26.3 / Horizon **68ac6f647e7022b2c696e81863d7cface28c9416**. This native mod is pinned to that engine commit; compatibility with newer Nightlies is not claimed. Install the .hnmod through Horizon and choose **Performance / Производительная сборка**. On this Windows baseline, the Dev profile can overflow the startup stack; Performance passed the ordinary startup check. This is a build-profile workaround, not a bundled engine patch.

Client-side only: player health and totem uses depend on data received by the client; unknown health stays unknown. Plugin-only cooldowns are unavailable. No server mod is required. This release does not promise an FPS improvement.

Validation: 67 Rust test executions, warning-free native build, isolated in-game editor checks, restart persistence, failed-save recovery and ordinary asynchronous startup. Windows was tested; other platforms were not. Language coverage is structurally tested; not every translation has had a full human language review.

SHA-256: `36431d3a2d810e2aefbd8611767a80f9184e3df032e4cc2b9b9fd9398f794560`

---

## По-русски

Главное обновление после 0.1.0: редактор прямо поверх игры, 13 отдельных блоков, перетаскивание и размер каждого, цвета и прозрачность. Добавлены FPS и координаты, реальные кулдауны, доступные клиенту данные здоровья игроков, мини-броня рядом с хотбаром и настройка Tab с числовым пингом и сортировкой. Режим «Значки + значения» оставляет ×16 и 1:45 без длинных названий. Скорость сглажена и выключена по умолчанию, пинг в одиночной игре обозначен как локальный.

Десять языков, обучение только при первом входе и затем через «?». Сброс требует подтверждения, а выход с правками предлагает «Сохранить / Выйти без сохранения / Остаться». При ошибке записи черновик сохраняется в редакторе для повторной попытки.

Для установки выбирайте «Производительная сборка». Поддерживается закреплённый Horizon68ac6f6; поддержка других Nightly требует отдельной проверки.
