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

---

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


## Українською

Велике оновлення після 0.1.0: редактор просто поверх гри та 13 незалежних віджетів. Перетягуйте блоки, змінюйте їхній розмір, кольори й прозорість. Додано FPS, координати, час відновлення предметів, доступні клієнту дані здоров’я гравців, мініброню біля панелі швидкого доступу та налаштування списку Tab із числовим пінгом і сортуванням. Режим «Значки + значення» залишає ×16 і 1:45 без довгих назв. Швидкість згладжено та вимкнено за замовчуванням; пінг в одиночній грі позначено як локальний.

Десять мов інтерфейсу. Навчання відкривається під час першого входу в налаштування, а потім — через «?». Скидання потребує підтвердження. Вихід зі змінами пропонує зберегти їх, відкинути або залишитися в редакторі. Помилка запису не видаляє чернетку.

Minecraft 26.3, закріплений Horizon 68ac6f6; обирайте профіль Performance. Сумісність з іншими Nightly не заявлена. Мод працює лише на клієнті: невідоме здоров’я залишається невідомим, а таймери серверних плагінів недоступні. Приріст FPS не обіцяється.

## Deutsch

Ein großes Update gegenüber 0.1.0: ein Editor direkt über dem Spiel und 13 unabhängige Widgets. Verschiebe die Anzeigen und passe Größe, Farben und Transparenz einzeln an. Neu sind FPS, Koordinaten, Gegenstands-Abklingzeiten, dem Client bekannte Gesundheitswerte anderer Spieler, eine kleine Rüstungsanzeige neben der Schnellzugriffsleiste und eine anpassbare Tab-Liste mit numerischem Ping und Sortierung. „Symbole + Werte“ zeigt ×16 und 1:45 ohne lange Namen. Die Geschwindigkeitsanzeige ist geglättet und standardmäßig aus; im Einzelspieler wird der Ping als lokal gekennzeichnet.

Zehn Oberflächensprachen. Die Einführung erscheint beim ersten Besuch der Einstellungen, danach über „?“. Zurücksetzen erfordert eine Bestätigung. Beim Verlassen mit Änderungen stehen Speichern, Verwerfen und Im Editor bleiben zur Wahl. Ein Schreibfehler lässt den Entwurf erhalten.

Minecraft 26.3, festgelegter Horizon-Stand 68ac6f6; verwende das Profil Performance. Andere Nightlies werden nicht als kompatibel angegeben. Das Mod arbeitet nur clientseitig: unbekannte Gesundheit bleibt unbekannt, und reine Server-Plugin-Timer sind nicht verfügbar. Ein FPS-Zuwachs wird nicht versprochen.

## Français

Une mise à jour majeure depuis la version 0.1.0 : un éditeur directement superposé au jeu et 13 widgets indépendants. Déplacez chaque bloc et réglez sa taille, ses couleurs et sa transparence. Nouveautés : FPS, coordonnées, délais de récupération des objets, santé des joueurs connue du client, petite vue de l’armure près de la barre d’accès rapide et liste Tab personnalisable avec ping numérique et tri. Le mode « Icônes + valeurs » conserve ×16 et 1:45 sans les noms longs. La vitesse est lissée et désactivée par défaut ; en solo, le ping est indiqué comme local.

Dix langues d’interface. Le tutoriel apparaît à la première visite des paramètres, puis reste accessible avec « ? ». La réinitialisation demande confirmation. En quittant avec des modifications, vous pouvez enregistrer, abandonner les changements ou rester dans l’éditeur. Une erreur d’écriture conserve le brouillon.

Minecraft 26.3, version précise de Horizon 68ac6f6 ; choisissez le profil Performance. La compatibilité avec d’autres Nightly n’est pas annoncée. Le mod fonctionne uniquement côté client : une santé inconnue reste inconnue et les délais propres aux plugins serveur ne sont pas disponibles. Aucun gain de FPS n’est promis.

## Español

Una gran actualización desde la versión 0.1.0: un editor superpuesto al juego y 13 widgets independientes. Arrastra cada bloque y ajusta su tamaño, colores y transparencia. Se añaden FPS, coordenadas, tiempos de recarga de objetos, la salud de jugadores que conoce el cliente, una vista pequeña de la armadura junto a la barra de acceso rápido y una lista Tab personalizable con ping numérico y ordenación. «Iconos + valores» conserva ×16 y 1:45 sin nombres largos. La velocidad se muestra de forma más estable y está desactivada por defecto; en una partida individual, el ping se indica como local.

Diez idiomas de interfaz. El tutorial aparece en la primera visita a los ajustes y después se abre con «?». Restablecer requiere confirmación. Al salir con cambios puedes guardar, descartar los cambios o permanecer en el editor. Un error de escritura conserva el borrador.

Minecraft 26.3, versión concreta de Horizon 68ac6f6; elige el perfil Performance. No se afirma compatibilidad con otras Nightly. El mod funciona solo en el cliente: la salud desconocida sigue siendo desconocida y no están disponibles los temporizadores exclusivos de plugins del servidor. No se promete un aumento de FPS.
