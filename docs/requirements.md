# Requirements

Captured 2026-09-28 across the opening conversations. These are what was asked for, kept close to the words used. Everything else in `docs/` is a proposal or research.

## The world is an engine

The world is a virtual engine of its own. The game runs on top of it. This is the foundation, and the first attempt builds it before anything else.

### Nothing comes from nowhere

The flaw this project starts from: you land, walk into a town, and the shop sells an unlimited stock of weapons. Nobody made them, nobody brought them, and they never run out. The town is protected, but nobody pays its soldiers. Things that would be possible in real life are not possible.

In this world:

- A shop has stock because someone made it and brought it there. When it is sold, it is gone.
- A guard works because someone pays them. A town pays from taxes, trade, or an empire that is itself funded. When the money stops, the service stops.
- Everything depends on everything. The world is balanced by its own dependencies, not by rules that forbid things.

### Consequences are physical

What is possible in real life should be possible here, with the consequences real life would bring. A gang can raid a town. If the raiders blow up their own ships in the process, they are stranded. They are not stuck forever, but leaving means building a ship from whatever they can find, or from scratch.

### Things are made of things

Like Minecraft, A + B makes C. The building blocks of everything are defined by the engine's rules and by code, not by a list of finished items.

- Digging a hole, melting the rock to extract iron, and forging a sword should be possible without the engine containing "sword". The engine knows what rock and iron are and how they behave. The sword is something someone makes.
- An item is a label, attributes, and behaviour. The engine does not know what things *are*; it knows how they interact.
- Because items are built, an item can be unique in the universe.
- The ceiling is open. Reaching things nobody planned is the ultimate destination. The example given was a teleporter.
- Not everything has to be code. The world itself has to be the engine.

### Start in the space age, without hardcoding it

The universe does not start in the stone age and evolve. It starts in the space age. But the space age should exist because it was built up and taught to the engine, not hardcoded into it. The engine must still know the stone age, because the space age is built on it.

## Intelligence

Things don't build themselves; people do. Materials and parts are not enough. Stranded next to a wrecked ship with every part you need, you still cannot build a ship if you don't know how.

Knowledge is a third input to making anything. You must already have it, find a book and read it, or talk to others and collaborate to put things together. Maybe someday computers will build things too.

This is integral to the overall design. It may not be in the very first stages.

## Duplication

Just because something exists doesn't mean anybody can build another one, even if its design or code is common knowledge. You have to go through all the datasheets of the things in it and how they fit together. Some components are easy to find. Others, like a drill, can't be made by hand; they need a machine factory, and the factory is a much more complex thing that isn't born on its own.

- Everything depends on everything else. That is where the effort and the cost are: you have to find a drill, find a scale, find the rest, and have the skill to put it together, or have it built in a factory somewhere.
- As things are invented, there must be a way to reproduce them through a chain of things.
- Stranded on a desert island, you can find food and eat it. You can't build a computer in a day. Eventually, perhaps, but first you have to make metals, melt them, make batteries, wires, and motors to machine things, and so on.
- Don't get too carried away. This is the basic idea, not a demand for every real-world step.
- As the world is trained and templates exist, things solidify.

## Knowledge is located

Knowledge is very key. If a planet holds datasheets and has never shared them, and the planet is destroyed, everything is lost. It isn't backed up anywhere. Anything that was produced on that world can no longer be produced, because everything about it was destroyed.

## Loss is real, and so is the climb back

Clarified 2026-09-28. Information can be lost, and the last chip factory can be destroyed. Nothing prevents it.

- Training gets to the design of a chip factory layer by layer, under the laws of the universe and the evolution it took to get there. Once built, it is released: the bill of materials, the templates, the spec sheets, everything needed to make it.
- If it is lost, getting back to that point is the challenge of whoever survives.
- How far can you go from a desert island: to a chip fab, to teleportation, and beyond, to things we can't even think of? If the engine's rules allow it, maybe even time travel.
- That climb is one of the challenges of the game.

## Many universes

Not one universe but many, the way World of Warcraft runs many realms.

- Some universes have certain types of rules, for training. Others are free-for-all.
- If a civilisation decides to annihilate itself, that is the choice it made. The safeguard is having good people in a universe.
- Many universes will destroy themselves, as it is human nature to destroy when there is too much power. Some will survive and thrive. That is the ultimate experience.

## The world runs on code

The original idea is a virtual computer as the world itself. Things are not only created by code, they run through code. A spaceship is higher-level code made of many other operations. When ship A meets ship B, code is running for both. Code far away doesn't need to be considered, because it's outside the cone of influence.

This is the most important early design decision: whether the world is rules with templated ways to build things, or code that does things, and where the core engine ends. See [ideas/code-and-laws.md](ideas/code-and-laws.md).

## Money

What makes money valuable should be answered, not assumed. Science fiction says "I have credits". Credits towards what: a computer system, like a crypto that controls it, or an element like gold that is rare in the universe and cannot be duplicated? At the end of the day, money is something rare that people are willing to use.

- There should be money before the space age, and money in the networked space age.
- Space-age money has to be validated somehow.
- Either way, money has to rest on the physics and primitive rules of the core universe.

## The in-game computer

A virtual computing engine inside the game. Players write code in a sandbox, and that code can reach the core of the game. The examples given: a better autopilot, and a programmatic way to create objects that can be traded. The game should grow from the inside, by what players make, rather than only by patches from outside.

This was called out as the distinctive part of the original concept.

## The game on top

The eventual game, once the engine can carry it:

- An Elite-style career. You start with a ship. Trade between worlds, fight in space, dock, take missions, and make money.
- The play is a journey: land on a planet, leave the ship, and continue on foot.
- Flight is a 3D simulator. On the ground, in cities and ports, play becomes an Ultima-style walk: streets, shops, bars, mission boards, shipyards. The two modes share one character, one purse, and one set of obligations.
- Money is credits, described as crypto: the one money that works everywhere in the universe. What stands behind credits is in [ideas/money.md](ideas/money.md).
- The universe is small on purpose, so that a networked game puts players near each other. The point of the size is more encounters with real people.

## Training by challenges

Added 2026-09-28. Before modelling towns, run challenge exercises to see whether the engine has enough in it yet. For example: dropped on a desert island with nothing, what series of steps gets a person onto a raft and across to a nearby island? Food first (a sharp rock, branches made into spears, fishing), then fire (rubbing wood or stones), then an axe (finding iron, melting it, casting, sharpening with a rock), chopping trees, making rope, building the raft, a sail or paddle, and the crossing.

- Once the engine's primitives let a challenge like this succeed, inject another random scenario and see whether the engine covers it without inventing anything new.
- This sets the tone for how the system is trained: basic survival, boats, and weapons first, then mechanical things (a bicycle, a car), then simple rockets, until the system is trained up to the space age.
- **A stranded person can die**, and often will in early testing. The chance of success should be about what a real person's would be. Run a challenge 20 or 30 times: perhaps five succeed and the rest die. Dying doesn't fail the test. **The test fails only if it runs out of logical options.** Better skills and better ways to feed yourself improve the odds, but there's always a chance of dying.
- Time in tests doesn't run in real time. Days of activity should compute in seconds, and waiting an hour should happen instantly.

See [challenges/stranded.md](challenges/stranded.md).

### Keep the spirit, not every detail

Added 2026-09-28, after stage 2 (fire). It's fine to simplify laws if that makes things easier. We don't have to be extremely granular to mimic a real-world example. This kind of fine tuning would happen a lot, and we don't want to over-engineer: keep the spirit of something alive.

**This is a standing rule** for every design like this: don't overcomplicate.

## Skills, and talking to the game

Added 2026-09-28. Making fire showed a new direction for how the game works.

- **Skills.** The whole fire-making process (grab twigs and grass, a 20-second window to heat and light it) would be very hard to build a user interface for. When a character succeeds at making a fire, the engine can save that as a skill. As you learn things, they become additional skills.
- **Talking to the game.** A very likely interface is an open microphone: explain what you want to do, and the game engine takes your prompt, checks whether it works with the rules of the world, and applies it.
- **The game interprets prompts itself.** Clarified 2026-09-28: this doesn't mean building a third-party AI into the engine. A voice prompt like "gather wood and make a fire", or "use your skill to do it", goes to the game's own interpreter, which turns it into the commands the core engine runs. If AI helps with that, fine, but it isn't required.
- **An adaptive interface.** The user interface itself becomes reflective and adaptive as you learn.
- **Where skills come from.** A blank mind knows nothing and is trained. Characters start with skills most people would know, such as building a fire. You can go to a library to learn skills beyond basic common sense.

See [ideas/skills-and-interface.md](ideas/skills-and-interface.md).

## Knowing what things are

Added 2026-09-28.

- A blank mind has no skills. It's very rare for a character to start there, because they'd have to be taught everything. With a person nearby, they could be told what fire is, or how to extract iron from rock. Without basic skills, a stranded person might figure out how to fish, but will never get off the island, at least not in their generation. Starting skill sets matter, and learning skills is good.
- **The extra dimension: skills let you identify things.** Walking around the island, you recognise what you know. Someone who has never seen a coconut wouldn't know what it is; they might guess it's some sort of fruit. A bow and arrow abandoned by someone else wouldn't mean anything to them, beyond common sense.
- **This matters when buying from others.** Most games have an honour system: ask a shop for a gun and you get a gun. Here, if you don't know what a gun is, the seller could hand you a stick, say it's a gun, and you'd pay for it. The same for a table or anything else: if you don't know what it is, you can't verify what you bought.
- Skills build the common sense to know what things are, because you've seen them before or built them yourself. A very basic human in early civilisation wouldn't know what an aeroplane is if they saw one flying overhead.
- This is part of the evolution of a person's mind.

See [ideas/recognition.md](ideas/recognition.md).

## The skill learning paradigm

Added 2026-09-28, **for a major design conversation later.**

- A player can learn skills by talking to someone else, going to a library, and so on, or on their own.
- But the player at the computer knows things too. I may know how to build a fire when my starting character doesn't, so I tell it: go find some grass, take a stick, start rubbing, you'll notice smoke. That becomes a skill. If I can teach my character to build a fire from what's in my brain, why can't I teach it to build a rocket to the Moon? That's not practical.
- One way to solve it, to experiment with: make skills hierarchical. A player can't teach a rocket to the Moon because everything before it has to come in order: crawl before you walk, walk before you run. That's how skills can be taught from my brain to my character.
- We also have to prevent someone downloading a script from the internet, putting it in the prompt, and having their character learn everything. That's not practical either.

See [ideas/skill-learning-paradigm.md](ideas/skill-learning-paradigm.md).

## Walking and carrying

Added 2026-09-28. Walking and carrying weren't taken into account and need to come into play. As soon as there's a basic interface, a character will be standing in the middle of a desert and walking around. Walking takes time, consumes energy, and so on.

## Getting off the island

Added 2026-09-28, when the crossing came up.

- A paddle, and relying on natural water currents, is the only form of propulsion we have. A sail would be nice, but there's nothing on the island to make a sail with at the moment.
- How far is another land mass, and how do we survive the journey? We need water and food. We can maybe fish along the way, and rely on capturing rain water along the way, plus any water we bring with us.
- Rough seas and storms are also a factor.
- It won't be easy, but start under some perfect conditions first.

See the stage 8 result and "After the crossing" in [challenges/stranded.md](challenges/stranded.md).

## More island scenarios before validation

Added 2026-09-28. The harder crossing is saved for later.

- Not validation scenarios yet: anything else thrown in now would fail, because we don't have enough yet. First build two more scenarios of what a person would do on an island like this, which will create more laws. Discuss them first.
- **Explore first.** Even seeing another island a few kilometres away, my instinct wouldn't be to build a raft to get there, unless I knew exactly where I was and what that island is. There's no way to know it's any better than the one I'm on. I'd want to know where I am. I may not even be on an island: I may be on a mainland, a few kilometres from civilisation.
- **Climb for a view.** Islands usually have a big mountain. Find the high ground and climb it for a bird's-eye view of where I am.
- **A moving camp.** In the first version, a residence at the beach while the raft is built. But it might take days to reach the top of the mountain: gather wood, make campsites along the way, and go up to take a look. All the other survival skills still apply.
- **Wildlife.** There's wildlife on the island, such as boars, or other hostile creatures. You have to protect yourself from them, but they're also food: a boar has a lot of meat. Make weapons to fight wild animals, and cook and eat them.
- **Protection from the elements.** Build a shelter against the night cold, or a barrier against animals; maybe a tree house.
- **Shoes and clothing.** Anything you have wears out soon, so you must be able to craft shoes and clothing from other materials.
- **Then validation.** After this exercise, try some validation scenarios.
- **Then other people.** If all this passes, the next evolution is NPCs: we meet others on the island, friends or foes. Maybe we form colonies and build houses together. That leads into slice 3: building a town, spending, and exploring.

## Sleep, and later age and body types

Added 2026-09-28, when day and night came up.

- If we introduce night, it makes sense to also introduce tiredness. What would a person do at night? It would be illogical for them to keep travelling at night just because they're not tired, and to use that time to keep going. That's not natural. **Make the need to sleep a necessity, now.**
- **Later: age.** Typical game characters are always young, fit bodies. We could have an older person, not as strong and not as mobile. Keep to a range: no one under 18 or over 55 for a game like this, for now. Age may be a factor.
- **Later: body types.** Different body types have different strengths: male, female, and so on. For now keep it as generic as possible.

## Classifying living things

Added 2026-09-29, starting the animals stage.

- Be clear on how we classify animals. We've already had animals that aren't human, such as fish, and now we're bringing in boars and others, so this has to be designed generically.
- Even humans should be classified under the animal category in some capacity, because later, in the space age, there will very likely be aliens: non-humans who are intelligent.
- Make sure it's designed correctly for classifying everything. It's almost like a taxonomy, without getting too complex, but that's what we have to be prepared for.

## Maps, and knowing what you know

Added 2026-09-29. **A phase after the animal work, before the vocabulary conversation.** After it, we should be in a good spot to try random scenarios.

- A person knows only what they have observed: they can't know the world around them until they've seen it. Your knowledge of your surroundings, and what your memory holds, will be a very important key item in a universe game: knowing what you know.
- You might find a map on the island while exploring. It fills in your memory with more about the island. So there has to be a way to expand your memory from what you've been told or found in a map.
- That doesn't make it definite: a map could be wrong. It's marked in your memory as a possibility, not a certainty.
- As you explore by the map and confirm things with your own eyes, what you know grows.
- If the map is wrong, you have to correct your memory. The same happens in the space age: given a star chart, you travel somewhere and only when you get there find the chart was wrong, or that the star it showed has since exploded.
- Even what you do know can change. A marker you spotted on the way out may be gone on the way back, for any number of reasons. Now you're in a contradiction: what you know is no longer there, and you have to correct it. You must be able to keep correcting your memory.

## How things cause harm

Added 2026-09-29, after the injury stage. Not to build yet beyond a pointed spear and an edged weapon, but structure for it now.

- We have the basics: a pointed spear, and an edge (an axe, for example).
- Soon we need to expand into blunt force, like a hammer; projectiles, like rocks, which can eventually also be a bullet; and eventually explosives, and other things that can cause damage.
- Structure the data to be oriented that way, and remember the structure of how weapons are classified.

See [ideas/harm.md](ideas/harm.md).

## From text adventure to the game's interface

Added 2026-09-29, **for a discussion soon**, along with the other pending conversations.

- Years ago there were text adventures such as Zork or The Pawn. They weren't a third-person view of someone walking around in 3D: the situation was explained to you and you prompted what to do. Walk around, go west, go north, gather this, make a fire, look under a rock, open the door.
- Our engine is very well suited to that kind of setup as its first proving ground: we set the laws of the universe and write scripts that carry out instructions.
- At some point we have to move to the game's real perspective: translate real-time actions of walking and moving around, creating skills, and buttons for repeating skills, and apply that to the scripting engine, to see how the two marry and go further.

See [ideas/game-interface.md](ideas/game-interface.md).

## Sacrifices, and universes that are configurable

Added 2026-09-29, starting the conversation on time.

- Every game that has attempted anything close to this has had to make sacrifices to be playable and fun. A player who binge-plays for 12 hours won't care about fatigue, the need to sleep, or nutrition. These are the first things engines like this sacrifice.
- We don't get rid of them: we've solved them as rules. But universes should be configurable. For example, main playing characters can be set not to need sleep or nutrition.
- Fatigue in general doesn't have to go away: run for a long time and you get tired and have to stop; fighting, your strength goes down as you tire. But the natural cycle of sleep can be removed as a need.
- NPCs follow the rules. Walk into a village at night and most people are asleep, because they had to sleep; you don't have to.
- It's up to each universe's administrator to turn things on and off. The norm would be no need for sleep or nutrition for players, which makes the game a lot more fun. Other concepts can come in instead, such as an occasional need for some kind of rest, or rations stocked up to travel long distances in space.
- We keep the rules in, so we can practise them, and perhaps have introductory tutorials based on hunting.
- Hunting and cooking definitely stay: you might capture a boar and sell its meat as an early form of currency, or cook for an NPC, who follows the rules of hunger, so feeding them gives you something to trade.

## How time passes

Added 2026-09-29.

- Don't change the overall speed of time (not a 12× world). Running from A to B at 10 km/h takes as long in real time as it would: time isn't going faster for what your body is going through.
- The day can be shorter or longer, but not because time goes faster: it's the rotation of the planet you're on. Earth is 24 hours; another planet could be 2 hours, 10 hours, or 30 days. It's a setting of the planet.
- Other world processes are where the game gets fun. Everything has a natural process and time. Digging 100 kg of ore, even with a shovel, might take 5 to 10 minutes in real life; nobody wants to wait five minutes, so digging could be accelerated to 10 or 30 seconds.
- Making a sword for real, from starting a fire and melting ore to casting, cooling, and hammering, could take two or three days. Everything is configurable: ore melts at its temperature, but might need to stay there only 10 seconds instead of three hours; cooling can be accelerated. Set right, making a sword could be a five-minute process.
- It still behaves by the world's physics, except that the time for certain processes varies, faster or slower, by our settings. Other engines do this too, but they cut huge corners; we follow real physics.
- Discuss these two concepts further before the conversation about what happens when you're away, which is different.
- **Refined 2026-09-29.** Everything that happens to your body runs at one speed, and so does how fast you move: one to one. In most games players move very fast and the terrain isn't very big; for now keep it one to one.
- This game won't be played on a desert island in the end: we're setting the rules. The goal is a space game where you dock and walk around a city. Areas to discover may be small, so walking at normal speed reaches things in a short time, and there will be transport to speed things up.
- Food spoiling can be real time.
- The need for sleep should not be tied directly to a planet. Life may evolve with its planet's day and night, as it did on Earth, but a planet with a two-hour day might evolve life differently, and a body or an NPC moving to another planet won't agree with that planet's cycle. On a planet with a 30-day rotation, nobody stays awake 30 days and sleeps 30 nights: colonists' sleep follows their species. So sleep belongs to the species, in the taxonomy: what they're naturally designed to do. The two are usually proportional, but not tied.
- Fire and fuel keep their real life cycle: 10 kg of wood can't go up in flames in 10 seconds. That can't be fast.
- Instead of speeding up the physics of a fire (a fire giving too much energy to smelt faster won't work), change the physics of the process: an ore needs a certain temperature, but only so much time at it to melt. Things harden in a short time after melting. That can be changed safely, and things will conserve pretty well.
- Could you really make 10 swords from 10 kg of wood? In reality no, but here, why not: if you're in the business of making swords, you use the one fire you made. Small sacrifices like that are acceptable, and it's all configurable.

## Pausing the stages for the big conversations

Added 2026-09-29. The stages' natural path keeps going until a village with many people, houses, people walking into buildings, trading, and money, and that could be many more stages. At some point we have to steer, and solve the outstanding questions that are piling up, perhaps only in early forms, and then continue the stages.

- **Added 2026-09-29:** settle vitality and what a player sees, and put those to bed, before continuing the island scenarios.

## Vitality, and moving to real time and 3D

Added 2026-09-29, settling vitality and what a player sees.

- The owner agreed with vitality as a named energy flow with stamina for hard work, and with healing needing food or medicine.
- **Rendering is its own discussion.** Something like a Grand Theft Auto engine is very good for walking around the earth, driving a car, and flying an airplane. But we might need different modes: flying a spacecraft is more of a cockpit view, but you might also have a third-person view of your spaceship. The choice of 3D rendering and how it works is something we have to discuss.
- **Space and gravity are very important too.** At some point we have to go a little science-fiction: using traditional rules we can't get from the Earth to the Moon in a short time (about six days with Newtonian physics), never mind other planets. So hyperdrive, light speed, and perhaps wormholes would have to be invented, with rules we apply. We have to expand the universe using science and theoretical laws to make things work eventually. These are different discussions.
- **The real question now: at what point do we move from developing with test scripts to something visual?** A game running in 3D is no longer turn-based; it's real time. Do we first evolve our testing system to have time passing regardless of our player taking action, so tests can be rewritten correctly and even involve several things happening at once, such as an NPC doing things? Or do we jump into building an actual 3D visualisation?
- I really like the back and forth we're having now: there's not much need for me to launch a browser and look at things. When we get there, hopefully we can use Playwright to take screenshots as needed, and Claude will build itself a channel to the game engine to move the player around automatically, so we can do practice runs in a more practical way as we work together.

See [ideas/game-interface.md](ideas/game-interface.md), [ideas/rendering.md](ideas/rendering.md), and [ideas/space-travel.md](ideas/space-travel.md).

## Players' rules, and healing by vitality

Added 2026-09-29, before real time. All our tests are based on the main player needing food and rest. Those are rules we can enable for certain actors: NPCs will have them, but the main player won't. It's worth making tests that don't need sleeping and eating (Claude proposed a short stage: rule switches, vitality, stamina, and player-mode proofs; agreed).

- **But not "a wound needs food to heal."** Vitality is the replacement for the food that's coming in. So it's the same law: if you have a wound, it heals on its own, from the flow of vitality. Eating extra food doesn't help healing; that's not how our game works.
- If you're wounded and the wound is superficial, it heals on its own. Otherwise it keeps bleeding you dry until you see a doctor.
- **Medical aid comes later**: a blood transfusion, a certain medicine, or an operation makes healing go faster. Bandages and the like can stop bleeding. But eating food shouldn't be what you need to heal: your supply of vitality is what's coming in.

## Time passing while a player is away

Added 2026-09-29, **for a discussion soon: before the vocabulary conversation and the other planned discussions.** Not to be solved yet.

- The passage of time when a player is not playing has suddenly become a big point: a person needs to feed and replenish themselves.
- If you leave the game, how does your person survive until you come back, from the perspective of others?
- This has surely come up in many other games that attempted something similar.

- **Decided 2026-09-29, after looking at what other games do.** Borrow something from each and come up with our own.
- Combat logging is a big problem. If you're in the middle of a space battle, or fighting boars, and log off, you'll die. That's the price you pay. To survive while logged off, you have to create a safe condition for yourself.
- You can set rules so that when you log off, your person instantly becomes an NPC, under conditions you choose. It tries to keep a low profile and stay alive as best it can, as aggressively or cautiously as the NPC profile you set.
- You might get notifications, by push or email, on how your person is doing, or that they've died.
- For example, you might have enough money to rent a room at a hotel, assuming the village isn't raided. At some point your money runs out and you're evicted. With a spaceship, you might park somewhere quiet inside a nebula and stay there indefinitely until you resurface.
- A player who walks away for many years and comes back is a problem to solve eventually. These are solvable one at a time, given a starting basis.
- **The general rule: your body stays, and things can happen to it.** How you protect yourself, and how you set your NPC rules, are among the first things to tackle; enhance as we go.

## Dying, and starting again

Added 2026-09-29, before the vocabulary conversation. For the time-away notes.

- Every game probably solves this differently. Dying is a reality, but it wipes everything out. Some games revert you to your last save point, which doesn't make sense in a real-time network game.
- You spend three months acquiring wealth and a spaceship, and when you die you start with nothing and go through it all again. That could get annoying in very hostile universes.
- I don't see a lot of good solutions for dying and losing everything in a realistic universe game.
- One balance: when you're resurrected, at least all your skills and everything you knew come with you, but not your money and possessions.
- **Decided 2026-09-29, with Claude's refinements.** In our basic example, when you die you're resurrected at a spawn point remembering everything, and your body loses everything it was carrying.
- If I dug a hole, put valuables in it, and buried it, I remember where it is. Assuming nobody else has found it, I can go back and get them.
- In a space game, a spaceship could be looted if it was in the middle of a fight. If I collided with an asteroid and died, then when I'm resurrected I can find my backup ship, or buy another, travel to the wreck, and see if I can salvage it.
- Credits you carry could be stolen, depending on how we decide what credits are. Reserves in a bank account, in a futuristic space game, are reachable another way: assuming the bank itself isn't destroyed, your resurrected body has access to them.
- These balances keep the game enjoyable: you risk losing what you have on you.

See [ideas/time-away.md](ideas/time-away.md).

## How new vocabulary emerges

Added 2026-09-29, **for a conversation after the where-am-I? and living-with-the-island challenges, and before any unplanned (validation) scenarios.**

- After these next scenarios are done, and before we begin the random scenarios to see if we can handle an unplanned scenario, take a pause and discuss an important evolution aspect of the engine: how new vocabulary words emerge as we are training.
- This could have a major impact on the evolution of the engine, so have it earlier rather than later.
- **Updated 2026-09-29:** this is a very, very critical conversation, and it must happen **before continuing living with the island stages 5 and 6**. The owner will explain it in greater detail; once explained in full, its real importance will be clear. Order: time passing first (with the interface), then vocabulary.

### The owner's explanation

Added 2026-09-29.

- This is tailored to a premise of the game's design: training the engine all the way from the Stone Age to the Space Age. The core engine doesn't know objects; it only understands concepts. It knows what fire is, what a container is, perhaps what a furnace situation is, and other laws of physics. But an engine starting from scratch doesn't know what a sword is. It does understand what an edge is, and that edges can cause damage to living things.
- **Here's the problem: what does the engine know to begin with**, beyond the forces of the universe and its concepts? It has to have a starting point for a player to invent their first weapon, so they can go and hunt. They might build a spear.
- The system used for commands will eventually need to understand "pick up the sword". But at first it doesn't know what a sword is; it has to be invented. **So how does the label "sword" get into the system? Where does the vocabulary of a sword begin?** Does the player build something and call it a sword, and then the word "sword" is added?
- If so, what happens when two players in the same world both invent a sword? Each might call it a different word. When they meet and show each other their items, **what labels show up between them?**
- **This is an important fork in the road for the design.** Do we preload everything and let players work their way to discovering things? Or does a different system let things be built and named through a consensus of players, just through normal conversations? Even if every player were American and spoke English, players around the world will label things differently. How do they communicate? Does the game have a hierarchical preset list, or do things get invented as they go along?
- This will dictate a lot about the design of the game. However far we train the system, even to the Space Age, new things will have to be invented. **How do they get named?**

### A practical example: two spears, and ships

Added 2026-09-29, in answer to Claude's first take (the owner's dictation says "sphere"; it means spear).

- I build a spear from a stick that I sharpen somehow. The system asks what I want to call this thing; I call it a spear. Now I have the item, the skill, and the word in my vocabulary.
- If I meet another person who has done the same thing, I won't know what they call it unless they tell me in conversation. But I'll be able to see that they have something that resembles a spear, and I can say "I see that you have a spear". They might not understand what a spear is in their language, but that's very normal in everyday life too.
- **So when the system creates the ID for the second player's invention, does it know about mine and give it the same ID? Or does everything get a fresh ID? Is the universe aware of similar inventions, to classify them as things progress?**
- A spear is very simple, but there can be very subtle variations. Someone might have a metal head with a piece of rope tied around it, which differs from a plain sharpened wooden stick. They're both spears, but the processes that built them are quite different, so a comparison of how they were made wouldn't give them the same ID. **This is a very crucial point.**
- The same happens with ships. No two ships flying in space are the same. If I see another one, I'll say it's a ship, but it could be built completely differently from mine.
- **So how does the engine start classifying things,** so that when I see other things, what I'm seeing can be explained to me based on my own knowledge? This is quite a challenge, but if we can solve it, it would make things easier. To be discussed further.

### Decided: recognising by resemblance, and visuals

Added 2026-09-29. The owner agreed with Claude's second take (recognising by resemblance), with a refinement:

- This will be much better once we have a practical game example: things built by two players who meet, and what each sees. It will all be 3D anyway.
- When you build something, the engine shows a resemblance of what it looks like. This is where visuals come in: build a spear, and the engine shows you something resembling a spear, because it knows the basics of what to do. See another player's stick with a metal blade tied on, and that's what you see.
- The engine can tell you "this looks like a spear", but the visuals also give your own mind a clue to what you're looking at. That's very important: sometimes the engine can't tell you what something is, but when you see it, you might fill it in yourself.
- Practically, when we build the 3D, real-time game, some of these problems will probably solve themselves through the natural evolution of the game.

See [ideas/vocabulary.md](ideas/vocabulary.md).

### The same word for different things

Added 2026-09-29, before the vocabulary stage (the dictation says "sphere"; it means spear).

- If I invent something and the system asks me to name it, I say it's a spear. If I then make a better spear from a stick, some rope, and a metal edge, the system asks me to name it. What if I say "spear" again? They're both technically spears, but a different variety. First, will this be allowed? I think it should be.
- But then how does the system work when I say "use my spear to do this"? Which one? I have two items that are now of type spear. In a user interface I'd look at my inventory and pick the other spear. But in the engine's command-line system, everything translates to words. So how does it work behind the scenes? As long as we have this kind of handling and processing, we have support for something like this.

## Why we climb from the Stone Age

Added 2026-09-29, in answer to Claude's questions on the skill learning paradigm. **This is the purpose of the training.**

- Starting from scratch and going all the way to the space age serves two purposes, and **its real purpose is to refine our game engine from the practical experience of doing it.** We could have started at the space age with everything pre-trained, but we'd have missed the natural evolutionary experience. It doesn't only make the game's evolution better; it actually defines how we build the game.
- **When the game is released, it will most likely start at a space-age level already**, so everything will already exist. How much your player knows will depend on what they go and learn and apply, and we take it from there.
- There might be other variations where we do the island experience. Those might be the tutorials and things like that.
- I see it more as an exercise in building the game by allowing ourselves to evolve. **I want to get to the space age using the exact way we're doing this.** We'll soon reach the village stage. From there we'd reach electricity, and we might make explosives. We might get into industries and make locomotives and pistons. Then we might make rockets, airplanes, and so on.
- **I actually want to go through the process of building a spaceship, launching it, and reaching orbit, just like the natural human technological evolution.** I think it will be a very, very practical approach. That's why I'm doing it this way: to refine the engine, the laws, and everything through this process.

See "The climb" in [ideas/production.md](ideas/production.md) and "Decided" in [ideas/skill-learning-paradigm.md](ideas/skill-learning-paradigm.md).

## Are we ready for the village?

Added 2026-09-30, answering Claude's village proposal ([ideas/village.md](ideas/village.md)). **Analyse before building.**

- Are we actually ready for this step yet? How many basic primitives and laws exist to attempt it? We have a person who can walk around, hunt, and survive. We don't have many building materials, building designs, or roads, and we don't have the NPC systems figured out yet, especially how trade can happen.
- **Start with a smaller community first before we go big**, or build a roadmap that takes us to the full village one step at a time.
- **My biggest concern:** when we make the real transition, taking our rules processor and turning it into a real-time system. When we build a 3D representation and move around and do things, how does that translate to a real-time system, and how does it interact with our prompts?
- **How much processing time do we devote to the NPCs?** I'd think they need a much simpler brain. If an NPC's mind works at exactly the same level as a real player's, it might be a burden on processing, or it might not: tell me.
- Before we jump into the village, analyse all this and make a solid plan, especially how we transition into visualisation, and whether the two worlds will coexist as we make the transition.

## Scoped NPC minds

Added 2026-09-30, a theoretical conversation on Claude's road ahead ([ideas/the-road-ahead.md](ideas/the-road-ahead.md)).

- We need the ability to **set a scope on NPC minds**. A basic, confined shopkeeper has limited things it can do: its job is to sell weaponry and the like in a shop, and nothing else.
- If the shop is attacked, the NPC might defend itself. **But if the shop is destroyed, what does the NPC still standing there do?** Do they take on a mind of their own and start doing other things, or not? That depends on the level of mind assigned to that person.
- So we have **different levels of NPC mind**: what they intend to do, and their survival skills.
- We might have **very intelligent NPCs that act completely autonomously, as another player would**. They need more processing time, because they could literally do the same things a regular player could; they become indistinguishable from players. A basic shopkeeper has different mind limitations.

See [ideas/npc-minds.md](ideas/npc-minds.md).

## The cone of influence

Added 2026-09-30, the same conversation, on Claude's "bigger steps" for places nobody watches.

- Think of the analogy: if a tree falls in the forest and nobody is around to hear it, does it make a difference? **It's all about the cone of influence.**
- If a boar is walking around in the forest and nobody is within its sphere of influence, it doesn't really matter exactly where it is. **It's almost as if history is written when it is discovered.**
- As a player walks around, the game doesn't have to worry about the exact placement of things outside the cone of influence. Only when they come into a measurement do they matter: **the measurement reveals the location.** It's almost the world of quantum mechanics.
- This lets the universe avoid spending CPU cycles on things that are irrelevant.
- Think about both concepts, scoped minds and the cone of influence, as ways to solve a lot of the scale problems.

See [ideas/cone-of-influence.md](ideas/cone-of-influence.md).

### Deciding it: a balance we'll discover

Added 2026-09-30, answering Claude's takes on scoped minds and the cone of influence.

- The cone of influence is a big decision point. If the world were static and everything about it known by the engine, the engine would know the world itself. It's really the cone of influence of where people are.
- **Some NPCs will make critical choices on their own**: an NPC might decide to blow up a planet, and that would happen in the system.
- It all comes down to the processing power, and how this scales. **A very delicate balance has to be reached**, and I don't have good answers for it. We'll discover it as the village grows, becomes several villages, and real players are in different parts of the world.
- For example, the village's smoke: if a player climbs and sees a village in the distance, the server doesn't need much detailed information about that village, unless a real player is in it; then that information would exist.
- **What a server remembers depends on what real players, or real NPCs, are affecting in the world.** So the scale problem goes up and down as things happen. We'll learn it as we expand the game.
- I don't have very good answers to the questions: **go with Claude's best recommendations.** If the server backend can scale, everything is fine. It comes down to how many players can comfortably live in the world, with the server keeping up with the world's changes.

## The browser stack

Added 2026-09-30, before the first browser page (a companion, stage 5).

- Have a quick discussion on how we build for the web. We're not jumping into 3D, but let's talk about the stack used at the browser level: what are the pros and cons of the different systems?

See [ideas/web-stack.md](ideas/web-stack.md).

### Performance first

Added 2026-09-30, answering Claude's take on the browser stack.

- There's a lot to consider, and I don't know what a good decision is. **Ultimately I care about performance when we're in a 3D world.**
- With WebGPU, the fastest direct access to the hardware, available from JavaScript directly or from WebAssembly using Rust, I'm torn on the right approach, and want to know how others are doing it.
- With WebAssembly we could write Rust directly for the client, which might make things easier for Claude. But Claude is writing the code, so it's up to Claude what's best.
- **Performance is very important**, because we want eventually to reach good visuals.
- When a new version is available, people should just refresh their browsers and get the latest version.
- Have the conversation from the aspect of overall performance.
- **Also consider physics engines.** There are a lot of good ones out there, and the game will eventually have rockets launched into orbit, cars driving, and space battles, so a good, reliable physics engine is important.
- **A sound engine is important.**
- **A very fast, robust networking engine for fast gameplay is important.** (The dictation says "robot networking"; it means robust.)
- Put those in the equation too.

### A thick client, not only the web

Added 2026-09-30, after Claude's take on performance ([ideas/web-stack.md](ideas/web-stack.md)). The owner brought two reports from another AI and asked Claude to marry them to its own research, give a new report, and an honest answer on the approach.

- I want to seriously explore thick apps as well as the web.
- **In all honesty, I don't mind abandoning a web client for a thick app if the web becomes our big bottleneck. In fact, I'm becoming more of a thick client fan, for a multitude of reasons.**
- The other AI's first report (web): for a GTA-scale game that walks a planet and fights in space, neither pure three.js nor a full Bevy-in-WebAssembly rewrite is the right first bet; it recommends a TypeScript WebGPU engine (Babylon.js or PlayCanvas) with WebAssembly modules for physics and simulation, and says pure Rust/Bevy wins later if the game goes native first. It stresses that such a game is a streaming, level-of-detail, and memory problem, not an engine-choice problem.
- The other AI's second report (thick client): if "open a URL and play" can go, don't use Electron (it keeps the browser's limits); build a real 64-bit Windows and macOS binary (Bevy or native wgpu), with the URL as storefront and updater, not the runtime. Its wins: memory that's yours, real threads, the full GPU API rather than WebGPU's subset, streaming from disk, precision for two-scale worlds, input, audio, and frame pacing, UDP networking, and your own install model (Steam, or a small launcher with delta patches).

### Sold on a thick client, and building for Windows from WSL2

Added 2026-09-30, answering Claude's report ([ideas/the-client.md](ideas/the-client.md)).

- **I'm definitely sold on a thick app**, for a multitude of reasons.
- Before this project, Claude and I built a Car Wars-style simulator, and went quite far: C++ and WebGL, as a thick application, and it worked great. But we hit a problem we'll most likely hit again, whatever the language.
- We were building natively for WSL2. A WSL2 window does have access to the GPU, but **the performance hit from a WSL2 app to the Windows GPU layer is big**. It looks fast at first, but as the game scales the triangle count isn't what you'd hope for. **Building a Windows version gave literally a 10x improvement in rendering speed.** Working in WSL2 and also working with Windows was quite a challenge.
- We solved it by installing the **MSYS2 MinGW64 shell** on Windows: a Linux-type environment with all the toolchains, where Claude built. All the scripts stayed intact, it built natively for Windows, and the source stayed in a Linux-type format.
- **Consider that for our first prototype**, unless Claude objects and wants to build directly on Windows. There are no good compilers on the Windows side, but we have the MinGW shell and can install whatever we want in it, Rust or anything else. The owner can give its location and access.

### The first scene, and assets for later

Added 2026-09-30, after the WSL benchmark.

- Go ahead and try a basic scene render, and we'll go from there.
- **Later we need conversations on assets:** where our 3D assets come from (bushes, trees, everything, people, building pieces). Getting them all created is a challenge of its own: build them ourselves, or get free default game assets, or even paid assets if Claude knows good sources. We need something to work with until the day we have our own graphics people producing genuine content.
- Starting from the basic spear experience (dictated as "sphere"), we need a stick, a rock: things to build our first layer.
- The rendering world is a complete world of its own, apart from the game underneath. It's a long journey, and we need a starting point somewhere.
- **We don't need to build for WSL any more: the point was proven. From now on, always build native Windows clients.**

See [ideas/assets.md](ideas/assets.md) and [ideas/the-client.md](ideas/the-client.md).

### Back on track: commands and the world together

Added 2026-09-30, after flying around the first scene.

- A good start: I see the island and I'm flying around it. I see trees, small bushes, maybe a couple of pieces of trees, but I don't see any actors walking around anywhere, and I don't know how to find them. There are no labels on anything.
- We don't need to concentrate too much on that right now: the basics are up and running.
- **Now we need to get back on track** and see where we go from here: the progression to start incorporating, at a very basic level, the commands and the world together. Come up with a plan of next steps.

See "The client joins the road" in [ideas/the-road-ahead.md](ideas/the-road-ahead.md).

### Played from the actor's perspective

Added 2026-09-30, before answering Claude's plan.

- Do we need to tell other things what to do? Maybe it's good for testing, but **the game will naturally be played from the perspective of the actor.**
- There are two main ways for the camera to work: first person, or **third person, looking at the person from above and behind: the classic online-game perspective, Tomb Raider and the like.** First-person shooters aren't the style here. **We want to see the actor walking around.**
- **Whatever we can see from that perspective is what we can see; what we can't see, we can't.** That's how the view of the world grows. There could be **a side map that expands as we explore.** Whatever I can see by zooming, or rotating with the mouse, is what I'm seeing. That way the engine doesn't have to make things dark or black because I can't see them: that's just how it is.
- I walk around with the actor. When I come near something, I can type a command: for example, "start a fire". If I have the skill, it will most likely tell me there's nothing nearby to start a fire with, and I have to go and find things. If I come across grass, I can say "cut the grass", "take it". **You have to be near things for them to happen, and we have to show them visually.**
- **As we pick things up, they go in our backpack, and we can show the items in it.**
- Clicking on a boar and telling it what to do is a nice feature, but **it's not how the game would be played**; maybe for our development purposes.
- Give a take on laying out the initial stage, and how we grow from there.

### The first milestones: camera, console, fire, backpack, voice

Added 2026-09-30, agreeing to all of Claude's proposals for the actor's view.

- **The camera** starts above and behind the person. I can rotate that perspective with the mouse and it stays at that angle while I keep walking, so I could watch the character sideways while they walk, or go back behind them, like any other game. I can change the up and down, and zoom.
- **A key breaks out into free flying: a debugging feature**, and I can snap back to the player when I want. While I'm flying freely, my regular WASD controls still move the player.
- **Another hotkey brings up a graphical console where we can type.** Maybe it's always present, with a little translucency so it looks nice. The same console could carry debugging messages to read: dual purpose, with an area to type in and an area where output comes in. Its size can be made bigger or smaller, but it has to be transparent enough to see the game behind it. Placement: wherever most games put it.
- **Aim for challenges again**, very small ones, as before. **A first goal: the actor walks around and makes a fire.**
- **Work towards a basic backpack** to put our items in. No need to show items graphically yet: labels inside a backpack we can open and close in a very simple way.
- **Sooner or later, voice input.** Typing is very annoying; we need a reliable voice-to-text engine, and plenty exist. Integrate it sooner rather than later.
- **A statistics window about our body:** click on it and expand all our vitals (nutrition, hunger, tiredness, health, bleeding, everything about the person), in a window we can open and close.
- These are our first milestones. **Put together a plan in the order Claude thinks best.** Once we pass these initial challenges, we can expand: other players, towns, and everything else.

### Moving by keys, and by commands

Added 2026-09-30, after stages 1 and 2.

- **For now, in normal mode, WASD doesn't move the islander.** The islander moves only through commands to go places.
- **In free camera mode, WASD pans around and goes places** (the camera, not the islander).
- **Later, WASD moves the islander, like a real game.** If the islander is under a command to walk somewhere and I touch WASD, that's an interruption: I take over.

**Decided, 2026-09-30:** as the owner describes. This replaces "while I'm flying freely, my regular WASD controls still move the player" from the first milestones: in free flight, WASD flies the camera. Walking by keys comes with stage 6, being near things; a key pressed during a commanded walk cuts it short where the islander is, and they walk on by hand from there.

Added the same day: there's a clash between WASD moving the camera and moving the person. It's only in debugging for now, but it might happen in some utility of the game. **We can split them: the arrow keys as one thing and WASD as another, so you can pan around the world and move the islander directly.** To address in stage 6, when the islander starts moving.

### Debugging as a lasting feature: snapping back, and a vocabulary of tools

Added 2026-09-30, after trying the fire with the clock sped up.

- **The clock snaps back to regular time when a command is finished:** if I say "go forest" and speed it up to x64, the moment I get to the forest it snaps back to x1. It can be a debug setting that's turned on and off.
- I'm sure we'll have **a lot of these manual debug settings** during development.
- **Think of debugging as a permanent feature of the game.** When we offer a world designer and things like that, these become actual features for single players and world and game designers to use. That day will come.
- So **we should start thinking about the debug command vocabulary and its architecture soon**, so that lots of settings can be turned on and off from the terminal.

Claude's proposal is in [ideas/tools.md](ideas/tools.md).

Added the same day, agreeing mostly with the proposal, on who can do what. **There are usually three layers:**

1. **Single player:** pretty much anything applies, because no one else is affected. You can change virtually anything about the game.
2. **Multiplayer:** there are certain things you just can't do, with respect to time, speed, and so on, because they'd have an adverse effect on everybody else. But there's no reason you can't give yourself extra strength, feed yourself from the command line, or turn on god mode; that's strictly the preference of the administrator running the server, who enables what's allowed and what isn't. When I played Quake, some servers allowed god mode for anybody, so they could absorb any damage and not die; sometimes it wasn't allowed. Changing the world map can also be allowed in multiplayer, because it doesn't have a cone-of-influence problem. But you can't speed up the game, because that speeds it up for everybody else. So be mindful of what can and can't be done.
3. **Always allowed:** commands allowed whatever game server you're attached to, because they're utility commands anybody can use.

These are the three layers of separation to be mindful of when designing the security context of the commands.

### Being near things, agreed; and how fine-grained things are

Added 2026-09-30. On the stage 6 proposal ([ideas/nearness.md](ideas/nearness.md)): **all those are fine.**

One more design decision for the whole game, which affects everything else profoundly: **the granularity of the items we interact with.**

- **Digging ore:** virtually anywhere you stand is dirt. If I dig for ore, am I affecting the terrain near me, or just extracting it? Does a hole appear that's there forever? That would mean a limited amount of ground to harvest, and ground altered constantly. Most games don't do that.
- **Grass:** standing on grass and gathering some, I just get some grass. But does the ground lose grass, does the patch disappear?
- **Trees:** in a forest, if I chop down a tree, do I just get some wood because a tree got chopped down? Which individual trees are still standing? Or do we separate into individual items, where an actual tree disappears from the map?
- **Boars:** if there are 10 boars on the island and I kill one, there are nine. So collecting grass is different from collecting a boar, or wood from a tree, or fish in the sea. I don't think the engine counts the fish individually; we just happen to catch one.
- **Houses:** a house can be destroyed, so it's limited.
- So: **where do we cross from one to the other, and how do we count things? Are some things unlimited, so we just get them, and others limited in the world? Is every tree marked on the map individually, while grass and dirt in general are not?** I need a ruling on this to decide how we shape these things, because it affects gameplay overall.

Claude's proposal is in [ideas/granularity.md](ideas/granularity.md).

Added the same day, on the proposal: **the rest is pretty good.**

- **Not breeding yet**; let's not worry about it for now.
- **Individual trees go to the map itself.** If the map draws the trees, it must know about them, so why not remove one when it's destroyed? Then they might also grow over time. They don't have to be tracked in much detail, but a dynamic world would require something like that. **A group of people will chop down an entire section of trees to build a city, so the map has to be altered.** A conversation for later.
- **Holes in the dirt: no, not directly.** But we might excavate an enormous amount of dirt for a project, and then the map might be specially altered.
- These will become clearer when we get into world building and village building, and especially with **futuristic cities, how they get built and destroyed**. Some advanced thinking will be important.

### Turning the fire into a skill

Added 2026-09-30, after the fire steps were written down to try. **When do we take a list like the fire's and turn it into a skill, so it's saved?** Then you could go to a forest and, near sticks and the other things, say "start fire": the skill would check everything is in order, it has the instructions, and it would go through the sequence. A conversation to have soon.

### Shortcuts, faster gathering, and clicking

Added 2026-09-30, after trying the fire.

- **Shortcuts in the terminal:** "gather wood x3", or by grams, "gather wood 500g": it would work out that's two lumps, giving 400 g of wood. Good shortcuts.
- **A command to drop things**, if we don't have one already.
- **Gathering takes far too long.** I can't type "gather wood" and wait 3 minutes; that wouldn't be fun. Gathering the fire's inventory from start to finish, assuming everything is next to you, should take **under a minute**.
- **Clicking on the objects near you**, to bypass the terminal: at the end of the day, **the terminal is for processes, not object interaction**. Ideas to follow.

### Clicking on things

Added 2026-09-30. It's simple: right now **the mouse hover tells you what something is**, so **if I left-click right there, a small context menu shows the things I can do with it**, like "gather", "push", "pull", whatever, **depending on what interactions are allowed at any given situation**.

Agreed and built the same day: the engine works out the menu from the laws (each action checked as if done now, only what would work offered); far things are walked up to first; choosing sends the command as if typed; a click on open ground walks there; gathering was offered x3 and x10 too, until the owner asked for just once (below). See "Shortcuts, faster gathering, and clicking" in [challenges/first-steps.md](challenges/first-steps.md).

### The context menu: what you can tell, and what to offer

Added 2026-10-01, after trying the menu. Things are looking good. Early user interface options; maybe it's too early to decide, but let's have the conversation, and it will be very much ongoing.

- **No canned multipliers.** I don't like gather once, three times, or ten times: it's pretty much canned, and doesn't look like good UX. We should be able to gather repeatedly, but we need a better interface to decide how much. **For now, just the standard "gather", without the multiplier**, until we figure out how to do it better.
- **Far things may not be identifiable yet.** A tree very far away, yes, the mouse can tell you that's a tree. But a little box or a small weapon on the ground, you won't be able to: hovering over it, you might just know there's something there. Get closer, and you'll know what it is. **Does hovering tell you something different far away and close by?**
- **What you can do with it.** Gather, kick, push, whatever. **Should those choices appear when you can't do them yet, or only when you're next to it?** Choosing gather from far away walks you to it to do it, which isn't bad. Maybe they should show ghosted. But you can always walk up to something: **maybe far away the only option should be go to, or walk to, and the other options appear when you get close.** I want your take on what's a good user interface for these choices.

Added the same day, on Claude's take ([ideas/context-menu.md](ideas/context-menu.md)): **those are all good.** And something else:

- **How does the engine know the distances for items, including new inventions?** A piece of wood is inherent to the game's design, so we know its distance problem. But build a cabinet from wood, and it becomes a new item. Put it down, walk away, and someone else sees it from far away: the engine has to decide its distance attributes. **It must be automatic: not up to the inventor, but the engine knowing, by size and things like that, what it is.**
- **It also has to be something the other person knows about.** Invent something complex, like a rocket engine, and someone who isn't advanced won't know what it is from far away, or even walking up to it close by.
- **Go ahead and implement this round.** After that, the entire fire-building process: an excellent exercise, but I see a lot of issues with the way it's set up. If we get it right, it sets up the basis for skill building.

### The long tests can wait

Added 2026-10-01, after a round of trials and benchmarks. **These tests are just taking way too long.** I understand what they're for, the castaway's long survival, but in these stages, especially when we turn off the need to sleep and food, **we need to step back on these and work more towards skill creation and other aspects** before we run these long kinds of tests.


### The fire, and a fork in the road

Added 2026-10-01. **This is the big fork in the road in the design of this game.** Before getting into the directions we could go, an exercise: explain the fire-making process. Looking at the script, it's quite complex, all the things you have to do to make fire: gather the twigs, the wood, the grass, the rocks, each picked up and gathered a certain number of times; then build the ring of rocks, put the grass in, rub things together, and add more wood. **What would happen if you did all of this partially?** Not enough wood, or no ring of rocks? **Does the script have to be exactly this, or will other variations work to build the fire?**

Claude played out eleven variations against the laws: the fire works by day as well as night, in any gathering order, with the rubbing sticks as the last fuel; it fails, as in life, without tinder, skipping the twigs, or unfed; and it fails, unlike life, with no ring (fire must be in something), with the fire laid first and lit after, with an ember tipped into tinder, and when fed without waiting for each size to catch, which nothing shows. See [ideas/ai-and-skills.md](ideas/ai-and-skills.md).

Added the same day, on those findings:

- **The ring of stones.** It builds a little container for the fire. Technically you can start a fire without a secure container and it will start, **but maybe it will blow out and dissipate faster than usual if it's not in a proper container**, and the rocks act as that. **Digging a little hole and putting the fire in it** would be the equivalent: the hole acts as a container too. Remember to work that into the physics.
- **The big fork in the road.** This is a literal physics engine of a game. A player has to look at their surroundings and at all the commands available. But **for a player to discover all of this naturally, with the vocabulary available and their surroundings, would take an awful long time** to discover the fire-building skill. It's possible, but it takes longer. We don't have natural language processing yet: I can't tell the engine "start a fire by coming up with a container, finding fuel, and putting it together"; I have to be very literal with the commands.
- **Marry natural language to the vocabulary.** We have an interface where a user can see all the commands available: the terminal. We need something like **AWS Bedrock** (not the only tool; there may be other ways) that marries natural language processing to the vocabulary, to get the approximations and build the prompt correctly: **your prompt looks at the commands available and your inventory, and puts together the correct terminal commands to get the job done.** This is the direction I think the engine should go: I want to get through the fire-building exercise in a fun, intuitive way.
- **The AI doesn't make decisions for you.** An AI's intelligence is basically a human intelligence, and we assume that, at the end of the day, **you're applying your own skills to the player**: you know how to make a fire, so you type that in, and it does it. The AI looks at the literal commands available, your inventory, and what else you need, and goes through the process.
- **Skills, good and bad.** You might do exactly what Claude did with the eleven variations: make fire eleven ways, and fail in some. **Each can be saved as a skill. It doesn't have to be a good skill; it could be a bad one.** And **skills can be traded**: with a better fire-making skill, I can give it or teach it to another player. **This is the basis for our game to evolve.**
- **This is one big fork in the road in the design.** Let me know what you think overall.

Added the same day, correcting Claude's take: **it isn't asking the AI a straight question** ("I want to make fire, how do I go about it?"). **I want to explain, in a natural language prompt, the process of making fire as I know it:** go find something small that could burn first, find something we can rub together, dig a hole or build a rock circle, put things in there, rub things together, and make a fire. Prompted exactly like that, our engine has no way to get from it to the literal commands.

- **The first thing the AI solves is bridging natural language to the command set, but only in the scope of what's available.** Tell it to use a cigarette lighter to light some grass and twigs: it knows what twigs and grass are, but comes back saying **it has no idea what a cigarette lighter is**, because that isn't in our scope of knowledge, and the AI won't do anything about that.
- **It takes the natural language prompt and looks at all the commands at its disposal, all the knowledge, and all the inventory, and comes up with a way to execute it** with what we have. That's the helper, and **that's the exercise I want to do.**
- **Later you can tell it to build a rocket, but only once we've gathered all the necessary steps before it.** Building a rocket is a skill, but how to build a combustion chamber to mix two gases is part of the process of how we evolve into building a rocket.
- **It's useful all through the game**, bridging the gap between our words and the command set.
- **This comes back to the skill paradigm: I can't bring my personal knowledge into the game if the player's current knowledge doesn't support it.** I know how to build a rocket personally, but the island doesn't have what we need yet. Once we solve one thing at a time, we'll get there. **The AI layer must be able to query the entire language, the skills, the inventory, and everything else to put the plan together.**

Added the same day, after the cost projections ([research/translator-costs.md](research/translator-costs.md)): **there's definitely hope here, but we still have to test the viability.** And there will be **multiple layers**: what's said is first scanned internally. **If it says "go forest", we're not going to waste time going to Haiku**: we take a first pass ourselves. **A second layer: built-in things we've made that take a stab at translating what was said into the commands** (the approach Claude wanted to do first). **And if all else fails, it goes to the next level up, the cloud.**

## The first attempt

No graphics. Text only. Very simple slices of the engine, each proving one simple concept, to chart a course and see where it goes. See [slices.md](slices.md).

## Explicitly not decided

- The number of systems, planets, or ports.
- The language, instruction set, or API of the in-game computer.
- Whether player code may change rules for everyone.
- Engine, networking, art, perspective on foot, single-player offline play.
- The implementation language for the slices.
