# The village

Claude's take, 2026-09-30. **Not decided.** This reshapes the old slice 3 plan ("a town with a budget" in [slices.md](../slices.md): a mine, a smith, a shop, a guard, a tax, and coins) in light of everything decided since: NPCs live by the realistic rules and players don't ([game-interface.md](game-interface.md)), names live in minds ([vocabulary.md](vocabulary.md)), the clock runs for everyone, a player who's away becomes an NPC under standing orders ([time-away.md](time-away.md)), and the engine has no idea what money is ([money.md](money.md)).

**The owner asked (2026-09-30) to analyse readiness first and reach the village in smaller steps.** The road there, and the measurements behind it, are in [the-road-ahead.md](the-road-ahead.md); this file stays as the design the later steps build toward.

## What the old plan still gets right

Its tests are still the point of the village:

- stop the tax, and the guard leaves after going unpaid;
- rob the shop, and it has nothing to sell until the supply chain restocks it;
- run the village for a long time with nobody playing, and it neither collapses nor prints money;
- after a raid, it recovers only through named sources: a migrant arrives, or a trader brings goods from beyond the map, at a price.

And its **fun check**: is watching and poking this village interesting? If it isn't, graphics won't save it.

## The one big new thing: how villagers decide

Boars already decide for themselves (their instinct: eat, drink, sleep, flee, charge). Villagers need more: a trade, a day's work, a wage. **Recommendation: a villager's mind is instinct plus standing orders.**

- **Instinct** covers the body, as it does for boars: eat when hungry, drink when thirsty, sleep at night, flee or fight when hurt.
- **Standing orders** cover everything else. They're lines in data: *when* something is true that the villager can perceive or knows, *do* a command, written in the same words a player types. For example: the fisher's orders say *at dawn, go to the shore; when there, fish; when carrying fish, go to the market; when there, sell fish*. The smith's say *when holding ore and charcoal, smelt*.

Why this shape:

- **It's the mechanism we already decided on for time away.** When you log off, you become an NPC under standing orders you set. Building villagers this way builds that too.
- **AI proposes, the engine decides.** Orders only produce commands, exactly as a player does. Every law still applies: a villager can't sell fish they didn't catch.
- **Jobs, taxes, and guards stay out of the engine.** "Guard", "tax", and "shop" are social arrangements, so, like names of things, they live in data: a guard is someone whose orders say *watch the market; if someone steals, go after them; if unpaid for a week, leave*. The engine knows only laws. The no-names scan keeps it honest.
- **Later, an AI can write or change orders** (a villager deciding to become a smith) without ever touching the world directly.

## Trade, and whose things are whose

Two new laws, both small:

- **Exchange.** A trade is two people, both present, swapping things they hold, in one step through the gate, only if both agree. A player agrees by typing it (`sell the meat to the smith for 3 coins`, `buy the knife`). A villager agrees if the deal meets the prices in their orders. Prices are numbers in data, not in the engine; the first version might nudge a price up when stock is low and down when it's high.
- **Ownership lives in minds, like names.** Holding something is physical; owning it is what people believe. You believe a thing is yours because you made it, bought it, found it, or were given it. A **theft** is taking what someone believes is theirs without a trade. The engine doesn't flag it: witnesses see it, remember it, and act on their orders. A theft nobody sees goes unpunished. No oracles.

## Money

The engine still knows nothing about money. The village starts with the rare metal in its data (not called gold in the engine) and coins: stamped discs of a fixed weight, made by whoever runs the mint. **Barter works from day one**; coins are just the thing everyone's orders accept. The slice 0 credits counter can retire once coins exist.

**Testing coins by density** (and fakes that are caught or missed) is the step where money meets physics. It needs weighing and measuring volume as actions. I'd make it a late, optional stage, or leave it to slice 5's coin validator, which does the same test with a computer.

## Beyond the map

A village isn't sealed. People and goods arrive and leave. Recommendation: **"beyond the map" becomes a named source and sink, like sunlight and vitality.** A migrant or trader who arrives brings mass (and coins) that the gate records as coming from beyond. Someone who leaves takes theirs away, recorded the same way. Conservation still holds: the village's own mass, minus what came from beyond, plus what left, never changes.

## Food is the real stability test

NPCs eat, so **a village that doesn't grow or catch its own food starves**, and then no tax or guard matters. Plants and shellfish already grow back from sunlight (the living-with-the-island world). The village needs a fisher, and something grown or gathered, feeding everyone, month after month. If a 30-day run with no player keeps everyone fed, the village is alive.

## The first browser page

Recommendation: **early, right after the village first lives on its own**, so you can watch it, which is the fun check. It's a plain page drawn from the live channel we already have: the time; where you are and who's here, each with what they're doing; what you carry; the places you know; a command box; and buttons for your saved skills. It's text and panels, not a picture. Playwright takes screenshots of it so Claude can check it and show you. Local only for now; hosting on AWS comes with the server.

## Proposed stages

A challenge like the others (`docs/challenges/the-village.md`, its own world), each stage with its scripted proofs:

| Stage | Name | What it proves |
| --- | --- | --- |
| 1 | The village lives | Around six villagers (fisher, gatherer or farmer, miner, smith, trader, headman) eat, sleep at night, work by their orders, and stay fed for 30 days with nobody playing |
| 2 | Watching it | The first browser page, with Playwright screenshots; the player walks in and sees who's doing what |
| 3 | Trade | Exchange and ownership in minds: the player sells boar meat, buys a knife, works for a wage; barter first |
| 4 | Coins | The mint, prices in coins; the trader buys from the smith, the smith buys ore from the miner; money goes round without being printed |
| 5 | Guard and tax | The headman collects tax and pays the guard; stop the tax and the guard leaves; rob the trader and, if seen, the guard comes after you; if unseen, you get away with it; the shelf stays empty until restocked |
| 6 | Beyond the map | Traders and migrants as a named source and sink; the village recovers after a raid only through them |
| 7 | (optional) Testing coins | Weighing and measuring: a fake is caught or missed depending on the tools |

Real-time follow-ups get fixed along the way: villagers act through the same start-and-finish path as players (so creatures come along too), and standing orders give us the command queue skills-as-buttons need. Walkers still aren't "on the path"; that waits for 3D and real space.

## Questions for the owner

1. **Standing orders as the villager's mind:** agree?
2. **Whose village is it?** The archipelago from the summit already has a village's smoke. I'd make it the far-folk's, the stranger's people, so their words are already in the world and an islander arriving has to learn them. Or keep the village speaking the islanders' words, to keep the first version plain.
3. **Where does the player start?** I'd give the village its own world file, with the player arriving at its edge carrying little; linking it to the island by raft can come later.
4. **Age of the village:** iron and the rare metal, as the old plan had, since the climb goes from here toward electricity?
5. **The browser page at stage 2**, before trade: agree, or would you rather have it last?
