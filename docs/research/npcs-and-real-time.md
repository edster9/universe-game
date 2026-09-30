# NPC minds, unwatched places, and real time

Surveyed 2026-09-30 before the village stage. It answers three worries: how much thinking NPCs cost, what to do with places nobody is watching, and how a rules engine that steps in turns becomes a smooth 3D game where people also type. Sources are official pages, developer talks and papers, wikis, and decompiled code; forum and search-excerpt sources are marked. Nothing was tested first-hand.

## Summary

In simulation-heavy games the **decisions** are rarely what costs most. Dwarf Fortress spends its time on units checking what they can see, then on temperature and items. Victoria 3 spends its time on its economy. Almost every game makes minds cheap in the same two ways: an NPC **re-thinks only when its current job ends** (or on a slow timer), and the **behaviour lives in the things**, not in the mind (The Sims, F.E.A.R.). LLM minds cost several orders of magnitude more. Places nobody watches are either **frozen** (Minecraft, older Bethesda games), **run coarsely** at long steps (RimWorld's world pawns, X4, Elite, Bannerlord's auto-resolve), or **run at a lower level of detail** that must stay consistent with the detailed one. Real-time games are nearly all tick-based underneath: EVE at 1 Hz, RuneScape at 0.6 s, Minecraft at 20 Hz, Factorio at 60 Hz. Smoothness comes from the client drawing slightly in the past and **interpolating** between ticks, or from the server sending "this move starts now and takes this long" and letting the client animate it. Typed and direct control have coexisted since the graphical MUDs; they work when both go through the same commands.

## 1. What NPC minds cost, and how they're built

**Dwarf Fortress.** The wiki's performance page: "The vast majority of processing time in Dwarf Fortress is taken up by units taking their turns, over 60% in larger forts, of which less than 10% is actually pathfinding related." "Line-of-sight calculations, even after optimizations in v50.05, are the slowest part of the game by a wide margin. This is O(n^2) by nature." Units more than 26 tiles apart are skipped. Temperature "place[s] a significant load"; players report "an FPS increase of 100% or better when disabling temperature calculations." Items (10,000 and more), flowing liquids, caverns, and trees also cost. A developer is quoted there saying that trapped units (who keep retrying a path) are "pretty much the only time pathfinding actually causes FPS issues." The advice is a population cap. No official "dwarves before FPS death" number was found.
Sources: https://dwarffortresswiki.org/index.php/Maximizing_framerate · https://dwarffortresswiki.org/index.php/Lag · https://steamcommunity.com/app/975370/discussions/0/3727324491572010578 (players)

**RimWorld.** 60 ticks per real second at normal speed, 2,500 ticks per game hour; a "rare tick" is 250 ticks and a "long tick" 2,000. Plants grow only on the long tick, by `GrowthPerTick * 2000` at once. In decompiled code (an older version; current values may differ), a pawn's main think tree is consulted **when its current job ends**, and a small "constant" think tree (emergencies) is checked **every 30 ticks** and can interrupt it. A community guide's summary: pawns never really "decide"; their next job follows from needs, schedule, and work priorities.
Sources: https://rimworldwiki.com/wiki/Time · https://github.com/josh-m/RW-Decompile/blob/master/Verse.AI/Pawn_JobTracker.cs · https://github.com/josh-m/RW-Decompile/blob/master/RimWorld/Plant.cs · https://steamcommunity.com/app/294100/discussions/0/1291817837618258616 (players)

**The Sims.** From notes by Kenneth Forbus and Will Wright (2001): each object has behaviours, each with "a procedure that checks to see whether or not it is possible, and a set of advertisements that describe its properties in terms of what need(s) of a Sim it will satisfy". Sims "not under direct player control" pick "the behavior that maximizes their current happiness", and the chosen procedure (which belongs to the object) runs in the Sim's thread. "Sims themselves are just a somewhat more elaborate object." A player-directed Sim and an autonomous one use the same object behaviours; only who picks differs. Wright has said the idea came from SimAnt's pheromones, so new objects could be added "without the Sims having any foreknowledge" of them.
Sources: https://qrg.northwestern.edu/papers/Files/Programming_Objects_in_The_Sims.pdf · https://amara.org/videos/cVoJS4OdmVql/en/4330642 (video transcript)

**Oblivion's Radiant AI.** Behaviour is a list of "AI packages", each "a bundle of hidden AI instructions with conditions for when and how to execute it". "When an actor needs to pick a new package, the list is examined from the top. The first package that is found valid based on its time schedule and logical conditions is selected." Schedules come in whole hours.
Source: https://cs.uesp.net/wiki/Package

**F.E.A.R. (GOAP).** Jeff Orkin, GDC 2006: the state machine has three states, "Goto, Animate, and UseSmartObject", and a planner uses A* over actions with preconditions and effects. Pathfinding is kept out of the planner's world state "because pathfinding is expensive"; it's checked "on-demand only when necessary".
Source: https://www.gamedevs.org/uploads/three-states-plan-ai-of-fear.pdf

**Utility AI and behaviour trees.** Utility AI scores every option and picks the best (Dave Mark, GDC 2010). A 2016 comparison by a middleware vendor: "For very large behavior trees, the costs of evaluating the whole tree can be prohibitive," and behaviour trees organise behaviour but give no model for deciding. No neutral benchmark of the three was found; the costs quoted are opinions.
Sources: https://gdcvault.com/play/1012410/Improving-AI-Decision-Modeling-Through · https://www.gamedeveloper.com/programming/are-behavior-trees-a-thing-of-the-past-

**Mount & Blade II: Bannerlord.** Lords choose among a few campaign behaviours (go to a settlement, patrol, defend, raid, besiege, chase), weighed by travel distance "and several other factors". The campaign map has its own navigation mesh "so that path-finding is fast". Kenshi: no primary source found on how its NPCs decide.
Source: https://www.gamebanshee.com/3wfc8 (dev blog, reprinted)

**Victoria 3.** A tick is six hours of game time, split into yearly, monthly, weekly, daily and regular tasks. "One of the most expensive things in the game is the employment update, followed by the pop need cache update and the modifier update." The world simulation costs most, not the AI.
Source: https://www.paradoxinteractive.com/games/victoria-3/news/dev-diary-76-performance

**Generative Agents (Stanford, 2023).** 25 LLM-driven agents in a small town, with memory, reflection, and plans. The paper reports that two game days "cost[] thousands of dollars in token credits and tak[e] multiple days to complete" (quoted in search results; not read in the full text here). Follow-ups claim big savings: "Lyfe Agents" at "10-100 times lower" cost; "Affordable Generative Agents" by replacing repeated LLM calls with learned policies. Even so, an LLM mind costs very roughly thousands of times more than an instinct rule (an estimate, not a measured figure).
Sources: https://arxiv.org/abs/2304.03442 · https://arxiv.org/abs/2310.02172 · https://arxiv.org/abs/2402.02053

**Patterns.** (1) Minds are cheap when they decide rarely: at the end of a job, on a slow timer, or when interrupted. (2) Behaviour kept in things (advertisements, smart objects, packages) keeps minds small and lets new things arrive without new mind code. (3) What costs most is perception (DF's line of sight), world physics (temperature, fluids, items), and economies. Pathfinding costs little except when it fails and retries.

## 2. Simulating what nobody is watching

**Minecraft.** Freeze. "Unloaded chunks are unprocessed by the game and do not process any of the game aspects." Only chunks within the "simulation distance" of a player are fully ticked.
Source: https://minecraft.wiki/w/Chunk

**Oblivion and Skyrim.** Mostly freeze, sometimes teleport. A 2007 player explanation: NPCs "generally have 'low level processing' set off - this means that their AI won't run if they are not in a loaded cell"; a traveller between two distant cities is "not in any danger during their journey." A forum excerpt says NPCs without it are "moved to a destination upon load where time was passing." A Skyrim mod's description says the engine advances a nearby NPC's schedule by at most one hour when you wait, so NPCs stay at the inn all night (search excerpt).
Sources: https://en.uesp.net/wiki/Oblivion_talk:NPCs (player) · https://forums.nexusmods.com/topic/159088-ai-travel-packages (excerpt) · https://www.nexusmods.com/skyrimspecialedition/articles/2467 (excerpt)

**RimWorld world pawns.** People off the map are "mothballed": decompiled code ticks them every 15,000 ticks (six game hours) with `TickMothballed(15000)`, one big step instead of 15,000 small ones.
Source: https://github.com/josh-m/RW-Decompile/blob/master/RimWorld.Planet/WorldPawns.cs

**X4: Foundations.** "High attention" near the player; everywhere else "low attention", which aims "to approximate the outcome of a high attention fight" at lower precision and update rate. Players report the approximation drifts: out of sight, turrets check only range, not line of fire, so stations fight better when you're away (player forum).
Sources: https://www.pcgameshardware.de/X4-Foundations-Spiel-61270/Specials/Timelines-DLC-Update-Vulkan-Tech-Test-Release-1449923/galerie/3896065/ (excerpt) · https://steamcommunity.com/app/392160/discussions/0/6664812048260991709 (players)

**Bannerlord.** Battles the player isn't in are simulated. A modder describes auto-resolve as taking a random troop's "power level" and applying it as damage to a defender, hit by hit.
Source: https://www.nexusmods.com/mountandblade2bannerlord/mods/673 (mod description)

**Elite Dangerous.** The "background simulation" runs a daily tick over every populated system, moving faction influence by what players did since the last tick. Per the players' guide, NPCs can't move influence themselves.
Sources: https://forums.frontier.co.uk/goto/post?id=4850407 (players' guide) · https://forums.frontier.co.uk/threads/when-and-what-is-the-tick.400292/

**Dwarf Fortress.** World generation simulates centuries of history in the abstract before play begins; Tarn Adams calls it the game's most striking part. How the world outside the fortress is simulated during play wasn't confirmed from a primary source.
Source: https://gamedeveloper.com/design/interview-the-making-of-dwarf-fortress

**Crowds.** Assassin's Creed Unity (GDC 2015) showed 10,000 crowd NPCs using "40 real AIs and 120 high resolution models", swapping low- and high-detail NPCs "without the player noticing". Kingdom Come: Deliverance II (GDC) needed AI level-of-detail for nearly 2,400 NPCs, about half of them in one city (session listing, excerpt).
Sources: https://gdcvault.com/play/1022141/Massive-Crowd-on-Assassin-s · https://schedule.gdconf.com/session/supporting-thousands-of-npcs-in-kingdom-come-deliverance-kingdom-come-deliverance-ii/915120

**Research: simulation LOD.** Šerý, Poch, Šafrata and Brom (2006) note that in games "Behaviour of the creatures out of the sight of the user is not simulated at all typically. This often causes a storyline inconsistency". Their fix is "gradual simulation simplifying". Places form a tree (region, village, house, room), and detail is an "elastic membrane" through it, pushed down near important objects and up everywhere else. Actions have "atomic" versions (whole result at once, at coarse detail) and "expanded" versions (sub-steps, at fine detail). Detail rises in a "crater" around the player before they arrive, with a wider radius for shrinking so it doesn't flicker at borders.
Source: https://artemis.ms.mff.cuni.cz/main/papers/IVE-LOD-2006.pdf

**How consistency is kept.** In the sources: (1) run the **same rules** with a longer step (RimWorld's growth and mothballing), so results match within rounding; (2) keep only **outcomes** consistent, not process (X4, Bannerlord), which invites drift players notice; (3) **freeze**, which is consistent but lifeless (Minecraft); (4) coarse processes whose single-step result equals the sum of their detailed steps (IVE). No source described checking coarse and detailed results against each other automatically.

## 3. From ticks to smooth real-time 3D, and typing alongside

**EVE Online.** "The tick rate of the physics simulation is 1hz which dramatically reduces the need to send updates"; the client hides it with "input prediction and movement interpolation". An input sent just before a tick feels responsive; just after, "it can feel like it takes a second or more". Ships move mostly by **commands**: approach, orbit, keep at range, warp, or double-click space to head that way. Under load, "time dilation" slows the game clock (as low as "5% of real time or something" in the example) because most load "is tied to the clock"; physics was only "5-10% of the load".
Sources: https://www.eveonline.com/news/view/paint-your-ship-red-and-make-it-faster · https://www.eveonline.com/news/view/introducing-time-dilation-tidi · https://wiki.eveuniversity.org/Manual_Piloting

**RuneScape.** A 0.6-second "game tick"; actions registered in one tick begin at the next. Walking is one square per tick, running two. Purely client-side things (menus, tabs) run at 20 ms. The world is click-to-move on a grid, drawn smoothly.
Source: https://oldschool.runescape.wiki/w/Game_tick

**Minecraft and Factorio.** Minecraft runs 20 ticks per second and slows the game when a tick takes over 50 ms. Factorio runs 60 updates per second in **deterministic lockstep**: every machine runs the whole simulation and only player actions travel, "a few hundred bytes per second". Actions are delayed by a latency buffer, the slowest player sets the speed, and desyncs are the risk.
Sources: https://minecraft.wiki/w/Tick · https://wiki.factorio.com/Time · https://factorio.com/blog/post/fff-76

**Age of Empires.** Sending positions "would limit us to 250 moving units", so each machine ran "the exact same simulation", passing only commands. Turns were "typically 200 msec", and a command issued in turn 1000 ran in turn 1002. Animation carried on between turns, so latency rose "but would stay smooth". Out-of-sync bugs were the hardest: "minutes later a villager would path a tiny bit off".
Source: https://www.gamedeveloper.com/programming/1500-archers-on-a-28-8-network-programming-in-age-of-empires-and-beyond

**Interpolation and prediction.** Gabriel Gambetta: the server updates "at low frequency, for example 10 times per second"; "show the other players in the past relative to the user's player" and interpolate; predict only your own character; clients send inputs, never state. Valve's Source engine runs 66 ticks per second in its shooters and draws others 100 ms in the past, enough to survive one lost packet (Valve wiki via search excerpt; the page refused direct fetches).
Sources: https://www.gabrielgambetta.com/client-server-game-architecture.html · https://www.gabrielgambetta.com/entity-interpolation.html · https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking

**Start-and-duration movement (MMOs).** From TrinityCore, an open-source reimplementation of World of Warcraft's server: a player's client sends movement only when direction or speed changes (plus a heartbeat), and observers extrapolate. NPC movement is sent as a **spline path with a duration**, and the position is computed for any moment along it. One message covers a whole walk.
Sources: https://trinitycore.atlassian.net/wiki/x/AYD9Kg (excerpt) · https://trinitycore.net/d3/d68/classmovement_1_1movespline

**Typed and direct control together.** Meridian 59 (1995) was "commonly conceived of as a graphical MUD". In Ultima Online you say "buy" or "sell" to a vendor, and "bank" to a banker or use the banker's click menu: typed speech and clicks reach the same action (the "bank" detail is from a fan-server wiki). Minecraft's typed `/commands` share the chat box with WASD play. Mantella (a Skyrim mod) lets players speak to NPCs through an LLM, which can trigger a few in-game actions such as follow or trade. Krafton's PUBG Ally (CES 2025) uses an on-device small language model to take spoken requests to find loot, drive, and fight.
Sources: https://en.wikipedia.org/wiki/Meridian_59 · https://mocagh.org/origin/uo-refguide.pdf · https://wiki.uooutlands.com/Banks · https://github.com/art-from-the-machine/Mantella · https://www.digitaltrends.com/gaming/inzoi-pubg-ai-cpc-characters-ces-2025/ · https://www.gamedeveloper.com/press-release/krafton-unveiled-co-playable-character-built-with-nvidia-ace

**Patterns.** (1) Real-time games are tick games; 1 Hz (EVE) to 60 Hz (Factorio) all ship. (2) Smoothness is the client's job: interpolate, or animate a move from its start, path, and duration. (3) Control can be by command (EVE's orbit, RuneScape's click-to-walk, AoE's orders) and still feel real-time. (4) Typing and clicking coexist when both become the same command.

## What this means for us

- **Our 1-second clock is already fast enough for a real-time 3D world.** EVE ships at 1 Hz and RuneScape at 0.6 s. The engine doesn't need a faster tick for the browser or for 3D; the client draws in between.
- **Our actions with a start and an end are exactly the MMO movement message.** The live channel should send "this person began walking this path at this time and arrives at that time" (and the same for chopping or eating). The browser animates it and only corrects on the next event. Send events, not a position every second.
- **Keep NPC minds as instinct rules that re-think only when an action ends or a sense interrupts them.** That's how RimWorld and Oblivion stay cheap, and The Sims shows the same things can serve a player's command and an NPC's choice. Put "what this is good for" in data on things, not in the mind (our laws-not-things rule already points this way). Hundreds of such minds should cost little next to heat, perception, and pathfinding. An LLM, if ever used, only proposes commands for a few characters (rule 6).
- **Watch perception and heat, not minds.** Dwarf Fortress's biggest cost is who-can-see-whom, then temperature and items. When the village grows, measure first, and cap perception by distance as DF does.
- **Simulate unwatched places with the same laws at longer steps, not with separate rules.** RimWorld's `TickMothballed(15000)` is our calm one-minute step taken further. Separate "outcome" rules drift (X4's turrets). Freezing (Minecraft) breaks our rule that time runs for everyone. Our conservation gate lets us test that a long step and many short steps agree, which no surveyed game described doing.
- **Typed commands and clicks should be two doors to one command set.** The console's commands become what a browser button or a 3D click sends, as UO did with "bank". An LLM interpreter produces the same commands.
- **Determinism is worth guarding.** Lockstep (AoE, Factorio) lets clients replay the world from commands alone, but a tiny drift ruins it. We already avoid floats and hash-map order; keep the server authoritative anyway, and keep determinism for tests, replays, and possibly client prediction later.
