# How new vocabulary emerges

> **Decided 2026-09-29:** names live in minds, and things are recognised by resemblance. See "Decided" at the end.

Recorded 2026-09-29 **for a conversation the owner has asked for**, after the [where am I?](../challenges/where-am-i.md) and [living with the island](../challenges/living-with-the-island.md) challenges and before any unplanned scenarios. The requirement is "How new vocabulary emerges" in [requirements.md](../requirements.md), with the owner's full explanation under "The owner's explanation". Nothing here is decided yet.

## The owner's question

The engine knows concepts (fire, an edge, a container), never objects. A sword has to be invented. So where does the word "sword" come from? If a player builds one and names it, what happens when two players each invent one and name it differently, then meet and show each other? Should the game preload a vocabulary and let players discover it, or let things be invented and named as play goes on, by consensus? And how do players with different languages communicate? However far the engine is trained, new things will be invented, so this decides a lot.

## What's happened so far

Every training stage has added words. They're of three kinds, and they live in different places:

| Kind | Where it lives | Added in training so far |
| --- | --- | --- |
| **Player words:** commands a person can say | The command parser (`intent.rs`) | `explore`, `sleep`, `fill`, `go … on …`, `divide`, `rub … until it catches` |
| **Engine words:** the roles, measurements, and properties the laws understand | Engine code (`world.rs`, `datasheet.rs`, `laws.rs`) | Roles: pulling, pushing, containing, casting. Measurements: buoyancy, holds up to, pushes with, can hold, awake for. Properties: softens in, becomes, tensile strength, ignition point |
| **World words:** the names of things | Data files (`data/*.toml`) | Rope, raft, paddle, pot, fired clay, logs, the next island |

The standing rules already keep the third kind out of the engine: a pot is data, and the engine knows only "a shape whose role is to contain". But the first two kinds grow with every stage, and each new one so far was added by hand, in code, when a story needed it.

## Questions for the conversation

- When a story needs something new, how do we tell a new **law** from a new **word** for an existing law? A paddle turned out to need a new role (pushing); a pot needed another (containing). Could they have been one broader idea?
- Should roles and properties be data rather than code, so a new world can bring its own vocabulary without changing the engine? What would the engine still have to know?
- Where does a *player's* vocabulary come from? It ties to [recognition](recognition.md) (you can only name what you know) and to the [skill learning paradigm](skill-learning-paradigm.md) (a skill is a named procedure).
- How does the vocabulary stay small as the engine climbs toward chips and rockets? What's the test that a new word is earning its place?
- Does vocabulary differ between universes, or peoples within one?

## A take

Claude's take, 2026-09-29. **Not decided.**

### Names belong to minds, not to the world

The world holds things, and the patterns they were made to (designs). It holds no names at all. Every name is a word in someone's head, pointing at a pattern. So there's never a single answer to "what is this called?", only "what does *this person* call it?"

That's how the real world works too. An iron blade with a grip is the same object in Tokyo and in Texas; only the word differs. It's also the last step of "no oracles": today every thing carries a label from the data file, and everyone sees it. That label is the one oracle people still have.

### What's known at the start: four layers

| Layer | What it holds | Where it lives | How it grows |
| --- | --- | --- | --- |
| **Laws** | Forces, and the roles parts can play: cut, pierce, contain, float, push, bind, burn | Engine code | Training stages add laws |
| **Nature** | Materials and living kinds that exist without anyone making them | World data | Chosen per universe |
| **A people's starting culture** | The words they know, and the designs they already know how to make | Data, per people | Chosen by whoever sets up the universe |
| **Invention** | New designs and new words | Made in play | By players and NPCs |

Training from the Stone Age to the Space Age grows the first layer only. The engine never learns "sword"; it learns "edge", "hardness", "lever", "circuit". Words never enter the engine. Data files keep handles like `flint` so we builders can refer to things, but players never see them: a people's starting words point at those handles.

**Preload or invent? Both, in layers.** Laws always; nature per universe; a starting culture per people; everything beyond is invented. A stone-age islander starts knowing stone, stick, cord, sharp, and fire. A spaceport engineer starts knowing circuits and hulls, and perhaps not how to make fire from sticks.

### How the first spear gets its name

1. The islander's starting culture has stone, stick, cord, and sharp, but not spear.
2. The player says: knap the flint until it's sharp; bind the sharp stone to the end of the long stick.
3. The engine checks each step against the laws and makes the thing. It measures it like any assembly. It resembles nothing the islander has a word for, so it's new to them. (The first take had the engine record a new design under a plain number; the second take below drops that: there's no catalogue of inventions.)
4. The engine says: *"You've made something new: a long stick with a sharp stone bound to one end. What do you call it?"* The player says "a spear".
5. Now "spear" is in the islander's words, pointing at that design, and the steps just taken become the skill "make a spear". "Make a spear" and "pick up the spear" now work, for this islander.

### Two inventors, two words

The world sees one design, discovered twice. Anna calls it a spear; Ben calls it a stabber. When they meet:

- **Each sees it in their own words.** Ben's screen says "Anna holds a stabber".
- **Words are learned by being told.** When Anna says "this is my spear", Ben learns that Anna calls it a spear. It goes into his words as something he was told, like a told way on a map, and like one it could be a lie.
- **Nobody's word is the right one.** If Ben starts saying "spear" when he trades with Anna's village, the word spreads. Agreement comes from talk, as it does in real languages: no vote, no authority, no global list.
- **If Ben had never seen one**, he'd see a description of what anyone can observe: "a long stick with a sharp stone bound to one end". Once told, it's a spear to him.

### When are two things the same thing?

*Superseded by "Recognising by resemblance" below: the owner's two spears show that matching on how a thing was made fails.*

The match mustn't be exact, or every hand-made spear would be a new invention. The proposal: a word points at an **arrangement**, meaning which kinds of parts, playing which roles, joined how.

- **Material doesn't change the word.** A bronze sword is still a sword. The material is something you may also see, if you know it.
- **Size matters loosely.** A thing matches if it's within about half to double the size of the one you learned the word from. So a knife and a sword can be different words for the same arrangement.

That's roughly how human categories work: a word means "things like this one", not a strict definition.

**General words** fall out of the laws. "Weapon" can mean anything with a harming role, and "container" anything that holds. A people's hierarchy of words (weapon, blade, sword) is theirs; the engine keeps only its own hierarchy of kinds, which the laws need (what flees what), under handles, not names.

### Real-world languages

This follows for free. A Spanish-speaking player's character has Spanish words. When an English speaker and a Spanish speaker meet, their characters are foreigners to each other: they can show things, point, and learn each other's words. Translating live chat between real languages is a separate question, parked.

### What it would change in what's built

- **Things lose their built-in names.** Each mind gets its own words (a lexicon). The labels in today's data become the islander's starting words, so the existing worlds keep working.
- **What you see and what you can say go through your own words.** The view and the command parser resolve names through the actor's words. Anything you have no word for is described from what anyone can see: the plan in [recognition.md](recognition.md). So vocabulary and recognition turn out to be one piece.
- **Invention.** When something made resembles nothing the maker has a word for, the engine asks the maker what to call it.
- **Teaching words.** Saying "this is a spear", or "call this a spear", teaches the listener, as a told claim.
- **Our hand-written designs** (raft, lean-to, pot) become a people's starting culture, or are left out so a world can start from scratch.

Not now: inventing new *shapes* from simple forms (rod, sheet, bowl, point); for now invention means new arrangements of shapes a people can already make, which covers most early inventions (a spear, a shoe, a barrier). Also not now: translating between real languages, and dialects.

### What other games and research show

From the survey in [research/naming-and-vocabulary.md](../research/naming-and-vocabulary.md):

- **Games keep two labels.** Most games that let players name things keep a type the engine knows beside a name a player gave (EVE's ship name and ship type; Minecraft's renamed items, shown in italics). Minecraft stores its own names as language-neutral keys that each player's language file turns into words. That's the closest match to "laws know concepts, and words are a display layer".
- **No game merges two independent names for the same kind of thing.** When a name has to be shared, the first namer wins (No Man's Sky), or the owner names the one object (EVE). Keeping names in minds would be new ground.
- **Real languages.** Final Fantasy XIV shows a fixed dictionary of concepts to each player in their own language. That works for a preloaded vocabulary, not for invented words.
- **Learning words in the world.** Heaven's Vault and Chants of Sennaar have players learn another people's words from context, with guesses that firm up or get corrected. That's the same shape as possible and certain claims in [memory](memory.md).
- **Agreement does emerge without an authority.** In the "naming game" (Steels, Baronchelli), agents converge on one word through many small exchanges: success drops rival words, and failure spreads the new one. With people (Centola and Baronchelli 2015), a shared name took over when partners were mixed, but not when people only ever met fixed neighbours: then local dialects formed. A committed minority of about a quarter can flip an established word. **For a small universe where players meet many others, words should converge by themselves, and isolated colonies should drift into dialects, which would be a feature.**
- **People hold categories as typical examples** and recognise things by resemblance (Rosch). That supports matching by arrangement and rough size, and naming by showing.

### Recommendation

Names live in minds; the world holds only patterns. Knowledge comes in layers: laws, nature, a starting culture, and invention. Build it as the next stage, before living with the island stages 5 and 6. Those stages then become its first test: **hide shoes are not in the islander's starting culture, so the islander has to invent them and name them.**

## Second take: recognising by resemblance

Claude's take, 2026-09-29, after the owner's example of two spears and of ships ("A practical example" in [requirements.md](../requirements.md)). **Not decided.** It replaces "When are two things the same thing?" above.

### The universe doesn't classify; each mind does

The owner is right that matching on how a thing was made can't work: a sharpened stick and a metal head bound with rope are both spears, and no two ships are alike. So the answer to "does the second inventor get the same ID?" is: **there is no shared ID to get.** The universe keeps the truth about every thing (its parts, materials, and measurements) and no catalogue of inventions at all. Classifying is something a *mind* does, when it looks at a thing and compares it with what it knows.

Three separate things, each belonging to one person:

| | What it is | Example |
| --- | --- | --- |
| **A recipe** (a skill) | How *I* made it | Sharpen a stick; or bind a metal head to a shaft with rope |
| **A word** | A name, plus a few remembered examples of things I've called by it | "Spear": my sharpened stick, and the metal-headed one Anna showed me |
| **A look** | What anyone can observe about a thing, measured by the engine | Long and thin, about 2 m, pierces at one end, gripped along its length, wood and metal |

A thing's look is measured by the engine from its datasheet, like everything else, so it's no oracle: it's what your senses can tell you, and less at a distance. **Recognition** compares a thing's look with the examples behind your words.

### What a look is made of

Kept small. What matters is **what a thing does and roughly how it's shaped**, not how it was made:

- **What it does:** the roles it can play, which the engine already measures (pierces, cuts, contains, floats, carries people, pushes against water, flies).
- **Its form:** coarse proportions: long and thin, flat, round, hollow, and so on.
- **Its size:** roughly.
- **What it seems made of**, to the eye: wood, stone, metal, hide, fibre.

The sharpened stick and the rope-bound metal spear share what they do (pierce at one end), their form (long and thin), and their size. They differ only in what they seem made of. So both look like a spear.

### What you're shown depends on what you know

| How close the look is to an example you know | What you see |
| --- | --- |
| Very close | "A spear" |
| Close, with visible differences | "A spear, but with a metal head bound on with rope": the nearest thing you know, plus what's different |
| Nothing you know is close | A plain description: "a long, thin wooden thing with a metal point" |

That middle row is the owner's "explained based on my knowledge": what you see is described in terms of what you already know. An islander who sees a canoe for the first time might see "something like my raft, but hollow and narrow".

### Words widen by use

A word holds a few examples, not one. Each time you call something by a word, or someone shows you a thing and tells you its word, that example joins the word. So "ship" stretches to cover every ship you've learned to call a ship, however each was built. If you also know "freighter", the closer word wins: a big cargo ship is "a freighter"; an odd one you've never seen the like of is "a ship", or "something like a ship". General words come the same way: call enough different things "weapon", and "weapon" means anything that harms.

### Distance limits what you can see

Seen far off, a ship shows only its size and that it flies: "a ship". Up close you see its parts, and a word that fits better may win. That ties into the horizon and landmarks laws: at a distance you see less of a thing's look.

### Your two spears, played through

1. I sharpen a stick. The engine asks what I call it; I say "spear". My word "spear" holds one example, and "make a spear" is my recipe.
2. Anna, elsewhere, binds a metal head to a shaft with rope and calls it a "stabber".
3. We meet. My screen says "Anna holds a spear, with a metal head bound on with rope". Hers says "they hold a stabber, all of wood".
4. I say "I see you have a spear". Anna hears a word she doesn't know, while I'm looking at her stabber, so she learns that I call it a spear. The universe never had to decide which of us was right, or whether they're the same.

### What the engine would need

- **A look for every thing,** measured from what it's made of: roles (already measured), form (from its size and proportions; shapes need a little more size data), size, and apparent material (one small entry per material: what it looks like to the untrained eye, as [recognition.md](recognition.md) already proposes).
- **Words in each mind:** a name and a few example looks.
- **One comparison law:** how close two looks are. Same roles matter most, then form, then size, then material. That's a few numbers in data, not fussiness in code.
- **No catalogue of inventions.** A made thing needs no design ID to exist: the engine measures it from its parts, as assemblies are measured now. Recipes (skills) and words live in minds.

### Recommendation

Recognise by resemblance of look (what it does, its form, its size, what it seems made of), never by how it was made. The universe keeps only the truth about each thing; every mind keeps its own words and examples. That answers the three questions from the first take: names live in minds; you see things in your own words, learning others' only when told; and making something new asks you what to call it.

## Decided

**2026-09-29.** The owner agreed with the second take, "Recognising by resemblance", and added that visuals carry much of the load.

- **Names live in minds.** The world holds things and the truth about them, and no names and no catalogue of inventions. Each mind holds its own words, each a name plus a few example looks, and its own recipes (skills).
- **Knowledge comes in layers:** laws (engine code), nature (per universe), a people's starting culture (words and recipes, in data), and invention (made in play). Training toward the Space Age grows the laws, never words.
- **Things are recognised by their look,** meaning what they do, their form, their size, and what they seem made of, never by how they were made. You see things in your own words: a close match by its name, a near one as "like a spear, but…", an unknown one as a plain description. Words widen as you use them, and the closer word wins.
- **Making something that resembles nothing you have a word for asks what to call it.** Other people's words are learned only by being told, and what you're told can be wrong.
- **Visuals do much of the work.** In the 3D game, the engine draws every thing from what it's really made of, its parts, shapes, and materials. You *see* the other player's stick with a metal blade tied on, even when your character has no word for it, and your own mind can fill in what it is. A word is a label on top of what you see, not a replacement for it. So the engine's descriptions matter most in text, and for things too far away or too subtle to see (what a metal is, how good an edge is).
- **Details settle in practice.** How close counts as "a spear", and how much of a thing's look shows at a distance, will be tuned when two players' builds first meet in a real game. Some of it will solve itself as the game evolves.

One consequence for [recognition.md](recognition.md): with visuals, the swindle of "a stick sold as a gun" shifts. Your eyes will see a stick. Deception moves to what the eye can't check: a coin's metal, a blade's hardness, a gun that looks right but doesn't fire.

**Not built yet.** It becomes the next stage when the stages resume, before living with the island stages 5 and 6: each mind gets words; views and commands go through them; things get a measured look; and hide shoes, which aren't in the islander's starting culture, are the first thing invented and named.
