# Challenge: a companion

Step 2 of the road to the village ([ideas/the-road-ahead.md](../ideas/the-road-ahead.md)), started 2026-09-30. The smallest community: the islander and the stranger, living on the same island. The stranger lives by the realistic rules (eats, drinks, sleeps) and has a mind of their own: scoped, with a temperament and standing orders, as decided in [ideas/npc-minds.md](../ideas/npc-minds.md). Nobody plays them.

The world is `data/companion.toml`: the island of [living with the island](living-with-the-island.md), with the stranger, whose people are the far-folk ([a stranger's words](strangers-words.md)), so everything the two do passes through two peoples' words.

## Stages

| Stage | Name | What it proves | Laws it needs |
| --- | --- | --- | --- |
| 1 | A mind of their own | The stranger looks after themselves for 30 days by their standing orders, with nobody commanding them. Everyone, people and creatures, acts through the same path: checked when they start, carried out when they finish. The shopkeeper test: take away what their orders need, and a *confined* mind stands and waits while a *resident* one finds another way to live | Minds: scope, standing orders in the person's own words, instinct underneath; creatures act at the end of what they do |
| 2 | Temperament | A boar turns on the stranger. Fight, defend, flee, or give in, set in data, decide what they do | Temperament as a setting for people as for creatures |
| 3 | Asking | The islander asks the stranger to do something ("ask the stranger to gather sticks"), in words the stranger knows. Within their scope they take it on; outside it, or in words they don't know, they don't | Asking: a request becomes a one-off order |
| 4 | Barter, and whose things are whose | Fish for meat, agreed by both, in one step. The stranger's orders say what they'll give for what, and a bad offer is refused. What they gathered or made is theirs in their mind: take it unseen and nothing happens; take it while they watch, and they remember | Exchange (both agree, one step through the gate); ownership lives in minds |
| 5 | Watching | The first browser page: the time, where you are, who's here and what they're doing, what you carry, the places you know, a command box. Playwright screenshots | None: a window onto the live channel |

## How minds work (stage 1)

A person with a mind is given, in data:

- **A scope** (`mind = "confined"` or `"resident"`; `"free"` comes later): how far they think.
- **Standing orders** (`orders = [...]`): lines like `"thirsty, at the stream: drink water"`. Before the colon, conditions they can tell for themselves: the time of day, whether they're hungry, thirsty, or tired, where they are, what they carry, what they see. After it, a command in their own words, exactly as a player would type it.

When they're free, awake, and not in danger, they take the **first order whose conditions hold and whose command the laws allow**, and start it. Underneath every scope is the body's **instinct**: sleep at night when tired, eat what they carry when hungry, drink if there's water where they stand. When no order applies:

- **confined**: they wait where they are;
- **resident**: they look after themselves, going for water or food at places they remember seeing it, and otherwise wait.

A mind knows only what its person perceives and remembers: no oracles. It only ever proposes commands, which the laws check like anyone's.

## Stage 1 result, 2026-09-30: passed

**The stranger lives on their own.** In `data/companion.toml`, they're a *resident* mind with seven standing orders in far-folk words: eat pale flesh when hungry; drop the shells once the flesh is gone; drink at the stream when thirsty, going there if they aren't; gather shellfish at the beach when hungry, going there if they aren't; and go back to the beach at night. They remember the beach, stream, forest, and hillside, having lived there a while (`remembers` in data).

- **Proofs** (`crates/engine/tests/companion.rs`): the stranger looks after themselves for 10 days, within a few kilos of where they started, with mass conserved; a **confined** stranger with the same orders lives by them, and is back on the beach by night; and **the shopkeeper test**: their orders send them for water to a spring that isn't there. Confined, they stand on the beach and die of thirst within six days; resident, they remember the stream, go there, drink, and live. Orders that make no sense (no colon, a condition nobody can tell, no command) are refused when the world loads.
- **The story** (`data/scripts/companion-1-a-mind-of-their-own.txt`): the islander watches the stranger's day: gathering on the beach, shells piling up on the sand, coming to the stream for water, back on the beach for the night, and alive ten days on.
- **The trial** (on demand): 10 of 10 strangers alive after 30 days with real chance, each about a kilo heavier.
- **Sabotage checks:** a resident mind that can't look after itself, a mind that ignores its orders, and a mind that never feels thirst each fail a proof.
- **Creatures now act at the end of what they do**, like people: a boar's walk, search, or charge is checked when it starts and carried out when it's due. Two stories changed: in the boar story played by players' rules, the herd, back at the stream for a morning drink, was caught mid-way out, so the check now looks ten minutes later; and the hunt brought down a boar whose hide carried 14 mg more rotting flesh.

**What the attempt found:**

- **Orders can be foolish, and the mind follows them.** The first orders put "carrying shell: drop shell" first; a fresh catch is shell *and* pale flesh, so every catch went straight back on the sand, and the stranger slowly starved. Order matters, as it does for any written routine.
- **The living island's shellfish bed can't feed a person on its own.** 60 kg growing back 5% a day gives a few hundred grams of flesh a day, against about 2.5 kg a person needs. Once stripped, searches mostly found nothing. The companion's island has a long rocky shore instead (400 kg, up to 600 kg), a choice recorded in its data.
- **A needed tool is matched by its design's name, not its look.** Fish need a spear, and only something assembled *as* a spear counts; a barb joined to a shaft by someone who doesn't know the word wouldn't, though it's the same thing. That goes against "things are recognised by their look". Noted for the stages where the stranger makes or trades tools.

**Simplified:** a mind has no temperament yet (stage 2); free minds, which plan their own way, come later; a mind decides only among what its orders, its body, and its memory offer, and remembers places only as it last saw them.

## Stage 2 result, 2026-09-30: passed

**Temperament is a setting on every mind** (`temperament = "fight" | "defend" | "flee" | "give in"`, defend if not given). People remember who has gone for them and whom they've gone for, and when. When someone who went for them is where they are, temperament decides before any standing order: **fight** strikes at them for as long as they're there; **defend** strikes back once for each blow; **flee** walks out by a way the danger isn't on; **give in** does neither. The stranger defends, with the iron barb their people brought ashore.

- **Proofs** (`crates/engine/tests/companion.rs`), with a lone boar cornered on the hillside where the stranger stands: fleeing, they get away to the forest and the boar, keeping to the hillside, stays; giving in, they're gored to death where they stand, never striking; fighting or defending, they wound it with the barb. And the difference between fight and defend: the islander strikes the stranger once and stays. Giving in, they take it; fleeing, they're gone; defending, they strike back exactly once; fighting, they keep at it.
- **The story** (`data/scripts/companion-2-temperament.txt`): the islander makes a spear and strikes the stranger on the beach; a minute later, "the stranger goes for you, and wounds you", and nothing more.
- **Sabotage checks:** walkers who stay within reach, defending that answers every blow within the minute, and a mind that ignores its temperament each fail a proof.

**What the attempt found:**

- **Minds named things by their data ids, which people with words of their own can't use.** "Strike the tusker with the barb" was refused, so the stranger fell through to their next choice and walked off. Stage 1 had worked by luck of names. Now anyone can **point**: `#12` means that very thing, whatever anyone calls it, for whoever perceives it; minds always point.
- **Nobody could get away from a charging animal.** A walker counts as still where they started until they arrive, so a cornered boar kept goring the stranger as they tried to leave, each wound cutting the walk short. Two rules fix it: **someone who has set off, by the time a strike begins, has gone** (a strike already under way still lands); and **people decide before creatures** in each second, so someone setting off to get away is gone before a charge begun the same moment. The story where a boar's charge cuts a walk short now has the islander set off two seconds after the boar has turned on them.
- **Defending first meant "strike while the attack was within the last minute"**, which is four blows at five seconds each. It means one blow back for each blow now.
- **A law we didn't need:** "a wound doesn't cut short a strike" was added to let blows be exchanged, but with both sides deciding in the same second, both blows land anyway; a sabotage check showed nothing depends on it, so it's gone.
- The console now says "wound them: they're bleeding" of people, and "it" of creatures.

**Simplified:** a strike is always with the finest edge carried; fleeing takes the first way out that the danger isn't on, and doesn't look for safety beyond it; a danger stays one for an hour.

## Stage 3 result, 2026-09-30: passed

**Asking:** `ask <someone> to <do something>`, to someone here, awake, with a mind of their own. They weigh the request **in their own words, against what they see and carry**, and either agree or say why not, in their own voice: *The stranger says, "I don't see the flint here."* A **confined** mind takes on only what its own orders already do ("it isn't what they do"); a **resident** one takes on anything the laws allow. What they agree to comes first when they're next free, after danger only, and they take it up once: doing it, or finding they can't and letting it go.

- **The story** (`data/scripts/companion-3-asking.txt`): asked to gather driftwood and hand it over, the stranger does, once they've finished the shellfish they were gathering; asked to drink from the sea, they say they can't; asked to take the flint the islander fetched, they don't know the word (to them it's glassy grey stone), and once told it, they agree.
- **Proofs** (`crates/engine/tests/companion.rs`): a confined stranger won't gather driftwood but will gather shellfish, their own work; a boar "decides for themselves"; a sleeper doesn't hear; a request taken up is gone, and one that can no longer be done when they're free (the wood taken back first) is let go rather than blocking what comes after.
- **Sabotage checks:** a confined mind that takes on anything, a request weighed in the asker's words instead of the listener's, and requests never acted on each fail a proof.

**What the attempt found:**

- **The stranger asked back.** "Take wood", with driftwood and a lump of wood both there, got *The stranger says, "Which wood: the driftwood on the sand, or the lump of wood?"*: the ambiguity law from the vocabulary stage, now in someone else's mouth.
- A request made while they're busy waits: they finish what they're doing first.

**Simplified:** requests are one action each, with no "go there and then do this"; the asker isn't told how a request came out, and sees it only by looking; someone who has set off can still be asked, though they can't be struck; nothing yet stops a resident stranger from agreeing to anything at all, however costly to them: what they'd want in return is stage 4.

## Stage 4 result, 2026-09-30: passed

**Barter.** `offer <something> to <someone> for <something>`. A mind has **values** in data, a worth per kilo for each of their own words (the stranger: `barb = 20, fish = 4, "pale flesh" = 2, wood = 1`); what a thing is worth to them is the highest value whose word fits it, as they call things, times its mass, and nothing they have no word-value for is worth anything to them. They weigh the offer in their own words: what they'd give must be something they carry, and what they'd get must be worth at least as much to them. Then **both things change hands in one step through the gate**, or neither does.

**Ownership lives in minds.** What comes into someone's hands is theirs, in their mind; it stays theirs when they put it down, and stops being theirs when they hand it to someone. Two minds can each believe the same shell is theirs. **Taking** something that someone here, awake, believes is theirs is seen: they remember who, and want nothing more to do with them ("You took what's mine") for requests or trades. Taken while they're away or asleep, nothing happens.

- **The story** (`data/scripts/companion-4-barter.txt`): driftwood won't buy the stranger's iron barb (*"The iron barb is worth more to me than the lump of wood."*); the islander's flint spear will, and the islander walks off with what they call a "flake of dark metal". At 13:00, with the stranger at the stream, the islander takes one of their shells from the beach: nothing. At 15:00 the stranger agrees to gather driftwood. Then the islander takes another shell in front of them: *"The stranger sees you take it: it's theirs."*, and after that, *The stranger says, "You took what's mine."*
- **Proofs** (`crates/engine/tests/companion.rs`): no use, worth less, and worth as much, with who owns what in each mind afterwards; a shell taken while the stranger sleeps leaves both believing it's theirs and no grudge, and one taken while they watch is remembered; a boar decides for themselves.
- **Sabotage checks:** trading whatever the worth, a giver who never lets go of a thing in their mind, sleepers who see thefts, and no grudge each fail a proof.

**What the attempt found:**

- **Values read from data come in alphabetical order**, so "the first value whose word fits" would have depended on spelling. It's "the highest that fits" instead: a spear with a barb on it is worth what a barb is.
- The islander has to offer "the flake", not "the barb": what they call it, not what the stranger does.
- Shells the stranger dropped as rubbish are still theirs, in their mind. Nothing yet lets someone give a thing up for good.

**Simplified:** one thing for one thing; no counter-offers or haggling; values never change, however much of something they have; the grudge lasts forever and does nothing but refuse; players can't yet trade with each other (someone without a mind "decides for themselves").
