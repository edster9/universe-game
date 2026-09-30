# What games do with a player's character while they're offline

Surveyed 2026-09-29 for the [time away](../ideas/time-away.md) discussion. Sources are official wikis and developer posts where they could be found; where only player forums were available, that's said. Nothing here was tested first-hand.

## Game by game

**Rust.** A player who leaves becomes a "sleeper": "a physical, defenseless entity" where they logged off, which other players and hostile animals can kill and anyone can loot. The wiki advises logging off inside your base. Sleepers are a server setting (`sleepers.on`). Hunger and thirst appear to keep draining offline; the best evidence is a server plugin, "PauseOfflineMetabolism", written to stop players "mysteriously dying while logged out". No official statement found.
Sources: https://rust.fandom.com/wiki/Sleeper · https://rust.fandom.com/wiki/PvP,_PvE,_Griefing,_Sleepers,_Doorshare · https://codefling.com/carbon/pause-offline-metabolism

**ARK: Survival Evolved.** The character "remains sleeping in the persistent world", and "everything can be looted & stolen" (official wiki). The fan wiki says a player's food and water freeze when they log off, but tamed creatures keep eating and can starve. Players dispute the freeze: they report drain when another player nearby takes the area out of stasis.
Sources: https://ark.wiki.gg/wiki/Gameplay_Mechanics · https://ark.fandom.com/wiki/Game_Persistence · https://steamcommunity.com/app/346110/discussions/0/594820656457628292/

**DayZ.** Players report that however you disconnect, including a crash, the character takes about 30 seconds to disappear and can be killed meanwhile. This answered "combat logging" in the original DayZ mod, where quitting made you vanish instantly. Community source only.
Source: https://steamcommunity.com/app/221100/discussions/2/154645214958021041

**EVE Online.** On a disconnect in space, the ship makes one "emergency warp", then stays in space for a while: about 1 minute, about 2 under NPC aggression, 15 with a player-combat timer (player forums). In 2012 CCP added "Log Off Safely": after a timer, "your ship is immediately removed from space", unless you're targeting or flagged for combat. The stated reason: players killed for logging off "a minute too soon". Skills train "in real time, even when logged off."
Sources: https://www.eveonline.com/news/view/happy-safe-fun-time · https://forums-archive.eveonline.com/topic/46110 · https://wiki.eveuniversity.org/Skills

**Ultima Online.** Logging out is instant in inns, taverns, your own house, or a house where you're a friend or co-owner. Anywhere else the character "will remain in game for a period of five minutes", vulnerable. After combat, you wait for it to wear off first. The Camping skill makes a safe exit in the wild: light kindling, wait about 30 seconds until the camp is "secure", then use a bedroll.
Sources: https://uo.com/wiki/ultima-online-wiki/beginning-the-adventure/concluding-a-play-session/ · https://wiki.ultimacodex.com/wiki/Camping

**World of Warcraft.** Logging out in an inn, a capital city, or another rest area is immediate; elsewhere there's a countdown (commonly 20 seconds), and none in combat. "Rested" experience builds while offline: faster in a rest area, capped. It exists to balance casual and heavy players.
Sources: https://wowprogramming.com/docs/api/Logout.html · https://warcraft.wiki.gg/wiki/Rest

**Haven & Hearth.** A character stays "for like 3-4 seconds" after logging out (player forum). A character who has committed a crime may stay where they logged out (unconfirmed). The Hearth Fire is where you return, and its flame shows whether its owner is online.
Sources: https://www.havenandhearth.com/forum/viewtopic.php?f=42&t=48982 · https://ringofbrodgar.com/wiki/Hearth_Fire

**Eco.** Calories are spent by work, so nothing drains while you're offline and nobody starves away from the game. Skill points keep accruing offline while the server runs, faster for better food and housing. From player threads and search excerpts; the official wiki couldn't be reached.
Sources: https://steamcommunity.com/app/382310/discussions/7/1694914735997778482/ · https://steamcommunity.com/app/382310/discussions/7/1699416432416784916/ · https://wiki.play.eco/en/Skill_Points

**Star Citizen.** Beds are logout points; sleeping in a ship's bed can set where you return. Logging out anywhere and coming back to the same spot is still "planned"; otherwise you return to your home location. Whether a body stays after an ordinary logout wasn't confirmed.
Sources: https://starcitizen.tools/Bed_logging · https://starcitizen.tools/Player_hab

**Albion Online.** In dangerous zones, a character "still remains in the world until the timer is done", about 60 seconds, even after quitting (player report, 2019).
Source: https://steamcommunity.com/app/761890/discussions/0/1679189548076335896/

**Tibia.** After a lost connection, monsters ignore the character for 30 seconds, and it's removed after 40 to 60 seconds; a player who has killed another stays online for 15 minutes (fan Q&A).
Source: https://www.tibiaqa.com/10423/what-happens-when-you-lose-connection-to-the-game-or-force-it-to-exit

**Hollowed Oath (2026).** A patch made every character "linger" in the world "helpless and vulnerable" after logging out, so that "disconnecting is no longer a magic escape hatch" (news excerpt).
Source: https://massivelyop.com/2026/07/19/retro-styled-mmo-hollowed-oath-lets-you-get-beat-up-while-youre-offline-for-a-while/

**Clash of Clans.** After your village is raided, a shield protects it "when you aren't online"; attacking others shortens it; a guard period follows. "Personal Break" forces players offline for a few minutes so their villages can be raided.
Source: https://www.clash.ninja/guides/what-are-shields-and-guard

**Animal Crossing: New Horizons.** Follows the real clock: building finishes "the next day". Stay away a week or more and cockroaches and weeds appear, more the longer you're gone.
Sources: https://shacknews.com/article/117080/how-does-time-work-in-animal-crossing-new-horizons · https://nookipedia.com/wiki/Roach · https://nookipedia.com/wiki/Weed

**7 Days to Die.** The player leaves the world; their land claim protects it, with offline blocks taking a quarter of the damage by default, for 7 days before the claim lapses.
Source: https://7daystodie.wiki.gg/wiki/Land_Claims

**Minecraft.** Hunger drops only from actions, never from time alone. No body is left behind in vanilla; a mod exists to add one.
Sources: https://minecraft.wiki/w/Tutorial:Hunger_management · https://modrinth.com/project/qVpTPVku

Not confirmed: Valheim, Foxhole, Mortal Online 2, Life is Feudal. Conan Exiles players report dying of hunger while offline in their own base, with no reply from the developer: https://forums.funcom.com/t/logging-in-dead-fully-sheltered-inside-base/133461

## Patterns

1. **The body stays, vulnerable, for as long as you're gone**: Rust, ARK, Conan Exiles. Safety is whatever you've built around yourself. Needs keep draining (Rust, Conan) or freeze (ARK, disputed).
2. **The body stays for a while, then leaves**: DayZ, Albion, Ultima Online, Tibia, EVE, Haven & Hearth, Hollowed Oath. The reason given every time is to stop players escaping danger by logging out ("combat logging").
3. **A safe place, or a safe way to leave**: inns and cities (WoW), inns, houses, and campsites (UO), a secure-logout timer (EVE), beds (Star Citizen).
4. **The body leaves, and what you own is protected**: Minecraft, 7 Days to Die, Clash of Clans.
5. **Progress while offline**: EVE's skills, WoW's rested experience, Eco's skill points (depending on food and housing), Animal Crossing's clock (growth, weeds, pests).
6. **Needs while offline**: drain (Rust), frozen (ARK, disputed), spent only by work (Eco, Minecraft).

## What designers said, and what players complained about

- Lingering bodies and sleepers exist to stop combat logging.
- CCP added a safe exit because players were killed for logging off "a minute too soon".
- WoW's rested experience balances casual and heavy players.
- Players complain about dying of hunger while offline (ARK, Conan), about being raided while offline (hence Rust's anti-offline-raid plugins and 7 Days to Die's protection), and about being forced into vulnerability (Clash's Personal Break).
