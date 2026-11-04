# The Basic HUD 0.5.0 — Recent pickups

- **See what you collected.** A new movable widget shows the item and quantity picked up from the ground.
- **Counts add up.** Pick up four torches, then two more before the entry disappears: it becomes **Torch x6**, and that item's timer restarts.
- **Choose the timing.** Five seconds by default; configurable from **1 to 60 seconds** in the HUD editor. Entries expire independently.
- **Style it your way.** Drag, resize, adjust colours and opacity, or show just the icon and count. New controls are translated into all ten interface languages.
- **Accurate collection amounts.** A partial pickup counts only the items that fit. `/give`, crafting and rearranging inventory do not by themselves create entries.
- **New icon.** The selected pixel-art totem replaces the old small-text icon.

Existing layouts and the v4 settings format are preserved. Unsaved duration changes use Save / Discard / Stay; resetting the widget asks for confirmation and restores the five-second default.

**Verified:** 75 strict Rust test executions, warning-free pinned native build, real in-game 4+2 aggregation, expiry, icon/count mode, partial pickup, editor persistence and ordinary startup without a controller. Full details and remaining verification limits: [release verification](https://github.com/repositorykreml1n/the-basic-hud/blob/main/docs/release-0.5.0.md).

Install `basic_hud-0.5.0.hnmod` through Horizon's native-mod manager and choose **Performance / Производительная сборка**. Minecraft 26.3, pinned Horizon `68ac6f647e7022b2c696e81863d7cface28c9416`; horizon-mixins `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`. Compatibility with other Nightlies is not claimed.

**SHA-256:** `97011cc7374354c6293678c7ab593240d0512bffde1d7b642dad521e1df3f8ac`

## Русский

Добавлен блок «Недавний подбор»: предмет и собранное количество, 5 секунд по умолчанию с настройкой от 1 до 60. Подберите 4 факела, затем ещё 2 до исчезновения записи — будет ×6, а таймер обновится. Доступны перетаскивание, масштаб, цвета и режим значок + количество. Частичный подбор учитывается точно; выдача командой и перекладывание сами по себе не считаются. Новая иконка — пиксельный тотем. Прежние настройки сохраняются.

## Українською

Новий віджет показує нещодавно підібрані предмети та їхню кількість. Типовий час — 5 секунд, налаштовується від 1 до 60. Підберіть 4 смолоскипи, потім ще 2 до зникнення запису — буде ×6, а таймер оновиться. Є переміщення, масштаб і режим значок + кількість. Попередні налаштування збережено; нова іконка — піксельний тотем.

## Deutsch

Ein neues Widget zeigt kürzlich aufgehobene Gegenstände und ihre Anzahl. Standardmäßig bleibt ein Eintrag 5 Sekunden sichtbar, einstellbar von 1 bis 60 Sekunden. Vier Fackeln und zwei weitere vor Ablauf ergeben ×6; der Timer startet erneut. Position, Größe, Farben und Symbol + Anzahl lassen sich anpassen. Vorhandene Einstellungen bleiben erhalten. Neues Symbol: ein Pixel-Totem.

## Français

Un nouveau widget affiche les objets ramassés récemment et leur quantité. La durée par défaut est de 5 secondes, réglable de 1 à 60. Quatre torches, puis deux autres avant la disparition de l’entrée, donnent ×6 et relancent le délai. Position, taille, couleurs et mode icône + quantité sont personnalisables. Les réglages existants sont conservés. Nouvelle icône : un totem en pixel art.

## Español

Un nuevo widget muestra los objetos recogidos recientemente y su cantidad. Dura 5 segundos por defecto, ajustables de 1 a 60. Recoge cuatro antorchas y luego dos más antes de que desaparezca la entrada: mostrará ×6 y reiniciará el temporizador. Puedes moverlo, cambiar su tamaño y colores, o mostrar solo el icono y la cantidad. Se conservan los ajustes anteriores. Nuevo icono: un tótem de píxeles.
