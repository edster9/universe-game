# Money

Working proposal, 2026-09-28. Not a decision. The requirement is "Money" in [requirements.md](../requirements.md).

## The question

Everyone in science fiction says "I have credits". Credits towards what? A ledger kept by a computer, like a crypto? A rare element, like gold, that cannot be duplicated? At the end of the day, money is something rare that people are willing to use. Whatever it is here, it has to work under the [world engine](world-engine.md)'s laws.

## The short answer

**The engine has no concept of money.** Money is whatever people in the world agree to accept. What the engine provides are the laws that make some things good money and others bad: mass is conserved, materials have density and composition, energy costs something, information has to travel, and knowledge can be kept secret.

That gives three kinds of money, each resting on a different law:

| Kind | What it is | What makes it scarce | What makes it trustworthy |
| --- | --- | --- | --- |
| Hard money | A rare physical material | Conservation of mass: it can only be mined | Physics: it can be tested |
| Issued money | A claim recorded by an issuer: notes, bank balances, town scrip | The issuer's restraint | The issuer's reputation, and the taxes it collects in its own money |
| Network credits | Balances on a shared ledger run by many machines | Energy: new credits cost real power to validate | The network's rules, and the secrecy of each owner's key |

"Credits" is the everyday word in the space age. What stands behind a particular credit depends on who issued it and how far the network reaches.

## What makes something good money

A thing makes good money if it is:

- **scarce**: hard to get more of;
- **durable**: it doesn't rot, rust, or evaporate;
- **divisible**: it can be split into small amounts and joined back;
- **portable**: it's worth a lot for its weight;
- **verifiable**: a stranger can tell the real thing from a fake;
- **accepted**: other people will take it.

The first five are physical properties, so under this engine they come from the laws. The last is social, and players and NPCs decide it.

## How fiction and history did it

**Fiction.**

- **Star Trek's gold-pressed latinum** is a substance replicators can't copy. That is hard money in a world where everything else can be duplicated.
- **Star Wars' credits** are issued money. In *The Phantom Menace*, a junk dealer on a frontier planet refuses Republic credits and wants "something more real". An issuer's money is only good where the issuer's reach is.
- **Most games' credits**, including Elite's, are a number with nothing behind it. That is the flaw this project starts from.

**History.** Each of these teaches something.

- **Cigarettes in prisoner-of-war camps.** Without official money, a camp settled on a durable, divisible, widely wanted good. Money emerges on its own.
- **Gold and silver coins.** Scarce and testable. A mint's stamp vouches for weight and purity, and the mint takes a cut.
- **Clipping and debasing.** People shaved coins and mixed cheaper metals in. Money invites fraud, and testing is the defence.
- **Tally sticks, bills of exchange, bank notes.** Money as a record of a debt, carried as paper instead of hauled as metal. Only as good as whoever pays it out.
- **The stone money of Yap.** Ownership of huge stones was recorded by what the community remembered. One stone lay at the bottom of the sea and still counted. Money can be purely a ledger.
- **Money that pays taxes.** A state that demands taxes in its own money creates demand for that money.
- **Bitcoin.** A ledger with no central keeper. Scarcity comes from the energy spent to validate it, and ownership is knowing a secret key.

## Money before the space age

This is the world of the slices: one town, one planet, no network.

### Barter first

With no money at all, trade is barter, and some good gets used as money anyway because everyone needs it: salt, iron nails, cloth. The engine doesn't need to arrange this; it emerges if agents trade.

### Hard money: metal coins

Pick a material in the world's data that is rare, doesn't corrode, and melts at a workable temperature. It doesn't have to be called gold. Every property money needs then comes straight from the laws:

- **Scarce.** Conservation of mass means the only source is a mine. Every coin in the world came out of the ground.
- **Durable.** The chemistry says it doesn't corrode.
- **Divisible.** It can be melted and recast.
- **Verifiable.** Its density is known. Archimedes' test (weigh it, then measure its volume in water) is the density law in action, not a special rule about money.

A **mint** turns metal into coins of a fixed weight with a stamp. The stamp is a promise: this coin has so much metal in it. A trusted mint's coins trade at face value without being weighed. That trust is the mint's reputation, and the mint charges for it.

**Counterfeiting is physics against skill.** A fake can be plated lead, or a mix with a cheaper metal. A very good fake could use a core of a different metal with nearly the same density. Real gold bars have been faked with tungsten cores, because tungsten's density is almost identical to gold's. Whether a fake is caught depends on the tester's tools and skill ([knowledge.md](knowledge.md)): a balance and a water bath catch crude fakes, and better instruments catch better ones. Nobody needs a rule saying "counterfeits are detected 80% of the time".

### Issued money: notes and banks

Hauling metal is heavy and risky. A bank keeps metal in a vault and hands out notes that promise it back. Notes are lighter and easier to steal, and they are worth exactly as much as the bank's promise.

- The vault is physical. It can be robbed.
- A bank that lends out more notes than it holds in metal can get away with it until too many people want their metal back at once. That is a bank run. The engine does not need a rule for it; it happens when agents stop trusting the bank.

### Town scrip and taxes

A town can issue its own money and demand taxes in it. That creates demand: everyone who owes tax needs scrip. The town pays its guard in scrip. This is the most direct answer to "who pays the guards?": the town, in money that is valuable because the town demands it back. If the town prints too much, prices rise, and the guard's wage buys less.

## Money in the networked space age

### Credits are only as good as the network's reach

In the space age, people want money that works everywhere without hauling metal. That means a ledger, and a ledger needs information to travel. **How information travels is a law of the engine**, and it decides what "universal" means:

- If messages travel only as fast as ships, every system has its own copy of the ledger, and copies disagree until a courier arrives. Someone could spend the same credits twice in two systems. Medieval bankers had this problem, and couriers carrying bills of exchange were their answer.
- If there are faster-than-light relays, they are physical things. They cost energy, need maintenance, and can be attacked. **Where the relays reach is where credits work.** Knock out a planet's relay and that planet goes offline: credits stop working there, and hard money and local trust take over.

### Network credits: the universal money

The closest thing to universal money is a shared ledger kept by many machines under fixed rules. No single empire can print it:

- **Scarcity is energy.** New credits are issued to whoever does the work of validating the ledger, and that work runs on [in-game computers](in-game-computer.md) that draw real power. Credits cost energy to make, so they can't be conjured.
- **The network pays for itself.** Relay and validator operators earn credits for carrying and confirming transactions. Just as a town's taxes pay its guards, network fees pay for the relays that make credits work. That is "who pays the guards?" asked of the network.
- **Ownership is knowledge.** Credits belong to whoever holds the key. A key is a design in the sense of [knowledge.md](knowledge.md): copying it is free, which is exactly the danger. Steal a key and you steal the money. Forget it, or die carrying the only copy, and those credits are gone for good.

This is the one money whose rules are fixed by the design team, not by an issuer in the world. It is the requirement's "crypto that works everywhere", within the network's reach.

### Issued credits still exist

Empires, corporations, and banks can issue their own credits on top of the network, or instead of it. Their money is as good as their promise and their tax base, like Star Wars' Republic credits. If an issuer prints too much, players move to network credits or hard money. Exchange rates between all these monies are set by trading, not by the design team.

### Hard money still matters

Out past the relays, on a planet whose network is down, or among people who don't trust anyone's ledger, metal (or some rarer space-age material) is what works. It is heavy, so hauling it is a trade run, and carrying it is a reason to be robbed. A ship destroyed with metal aboard leaves it in the wreck, because mass is conserved.

### Validation, in three layers

| Layer | How it's checked | What beats it |
| --- | --- | --- |
| Physical | Density, composition, instruments | A better fake, or a worse tester |
| Institutional | A mint's stamp, a bank's signature and serial number | Forgery, or the issuer failing |
| Network | Enough relays confirm the transaction | Being cut off, or controlling enough of the network to lie |

## Where money comes from and where it goes

The earlier rule that "credits are never created from nothing" becomes more precise:

- **Hard money** is conserved as mass. It enters by mining. It never truly leaves; it gets lost, buried, sunk, or scattered in wrecks.
- **Issued money** is created by its issuer, when it pays for things or makes loans. That isn't an exploit, it's what issuers do. The consequence is inflation, and people stop accepting it.
- **Network credits** are created only by the network's fixed rules, for validation work that costs energy. They leave when keys are lost.

The design team fixes only the network's rules and the physics. Everything else is decided by people in the world.

## Stories this allows

- **The stranded crew again.** Their credits are safe on the ledger, but the planet's relay was destroyed in the raid. The locals want metal or goods. Knowing your key is worth nothing until you can reach the network.
- **The perfect fake.** A bar passes the water test. A better instrument, or a smith who knows the metal, finds the core.
- **The run on the bank.** A rumour spreads that the port's bank has lent out more than its vault holds. The line forms. The rumour becomes true.
- **The relay raid.** Pirates take out a system's relay. For a week the whole system trades in metal and favours, and the pirates are the ones holding metal.
- **The dead man's key.** A trader dies carrying the only copy of their key. Their fortune is visible on the ledger and cannot be spent by anyone, ever.

## What the engine must know

Nothing about money itself. It needs laws that already belong to it:

- Conservation of mass, and material properties including density and abundance.
- The energy cost of computing.
- How information travels, and at what speed.
- Keys and designs as knowledge: copyable, and lost when nobody holds them.

If money needs a law of its own, the design has gone wrong.

## Not real money

These credits have no value outside the game and cannot be bought or cashed out with real money. Tying in-game money to real money brings gambling and securities law, speculators, and bots, and changes who plays and why. That is a separate decision. The recommendation is not to make it.

## In the slices

- **Slice 0** keeps a plain credits counter as a stand-in, so conservation can be tested before money exists.
- **Slice 1** includes one rare, non-corroding metal in its data file.
- **Slice 3** is where money becomes real. The town has coins minted from that metal, the guard is paid in them, and testing coins by density works. Barter comes first if no coins exist.
- **Network credits** wait until computers (slice 5) and a second place connected by messages (slice 8) exist.

## Open

- Which material is the hard money, before and after the space age? Should the space age have a rarer one?
- Can matter be transmuted into the money metal at some energy cost? If so, that cost caps its value.
- How does information travel between systems: only with ships, through relays, or both? This decides what "universal" means.
- In the fiction, who wrote the network's rules, and why does everyone accept them?
