# Production

Working proposal, 2026-09-28. Not a decision. The requirement is "Duplication" in [requirements.md](../requirements.md).

## Knowing is not making

Suppose someone invents a money validator, and the code becomes common knowledge. That doesn't mean anyone can build another one.

The validator's design lists its parts. Some are easy to find. Others aren't: the drill that takes a core sample can't be made by hand. It has to be made in a machine shop, and the machine shop is itself a much more complex thing that didn't appear on its own. To build a validator you have to find a drill, find a scale, find the rest, and either have the skill to put it all together or have it built in a factory somewhere.

That chain is where the effort and the cost are. It is how the real world works. Stranded on a desert island, you can find food and eat it. You can't build a computer in a day. Eventually, perhaps: first metals, melting, then batteries, wires, and motors to machine things with, and so on.

## Nobody knows how to make a pencil

Two real-world anchors:

- **"I, Pencil"** (Leonard Read, 1958) points out that no single person knows how to make a pencil. The wood, graphite, lacquer, brass, and eraser each come from separate chains of people and machines.
- **The Toaster Project** (Thomas Thwaites, 2009). Thwaites built a toaster from raw materials, smelting his own iron and making his own plastic. It took about nine months, and the result barely worked.

That is the design, and it is also why this game has trade. Chains are too deep for one person, so people specialise and exchange. Everything depending on everything is what makes other players worth meeting.

## The chain comes from laws, not from a recipe list

No one writes "a drill requires a machine shop". A few **laws of manufacture** produce that dependency on their own:

- **Hardness.** A tool can only cut, drill, or shape something softer than itself. To drill hardened steel you need something harder, and making that needs something harder still.
- **Precision is inherited.** A part is at most as precise as the machine that made it. A hand-filed part is rough, and a design that needs a tight fit fails with rough parts.
- **Precision can be bootstrapped, slowly.** Precision can still come from nothing: rubbing three plates against each other in turn produces surfaces flatter than any tool used to make them. By hand, precision costs time. With good machines, it's fast. This is why a lathe is sometimes described as the machine tool that can make itself.
- **Conditions.** Some processes need a temperature, a vacuum, or a cleanliness that only a facility can provide. A chip needs precision and cleanliness together, which means a fabrication plant.
- **Throughput.** A skilled person makes one part a day. A factory makes thousands, because its machines and programs do the repetition.

Everything else follows. A validator needs a drill, which needs hard tool steel and a precise spindle. Those need a lathe and a heat-treating furnace. The lathe needs precise parts, which need either another lathe or weeks of hand work with plates. The dependencies were never written down; they are the laws, applied.

## Factories

A factory is an assembly like anything else. What it adds:

- **Conditions**: heat, vacuum, cleanliness, precision.
- **Machines**, whose datasheets decide the quality of everything they make.
- **Programs** that run the repetition. Automation is designs running as code (see [code-and-laws.md](code-and-laws.md)).
- **People** with skills to run, repair, and adjust it.
- **Inputs, and energy.** Cut off either and it stops.

A factory is also where knowledge settles. Its tuned machines, its programs, and its workers' skills together hold more than any single design document.

## From prototype to production

Things "solidify" as the world is trained:

1. **Prototype.** The first one is built by hand or in a workshop. It is slow, expensive, and each copy comes out a bit different.
2. **Proven design.** It has been built and measured. Its parts, processes, and required tools are known and recorded. The engine has its datasheet.
3. **Production line.** A factory is set up to make it. Copies are cheaper, faster, and consistent.

Prehistory (see [world-engine.md](world-engine.md)) is this process run in advance. It builds the space age's chains and factories so that the world opens with proven designs and working production, all made under the laws.

## No dead ends

A world where the only chip factory can be destroyed has a risk: if no chips can ever be made again, no ships can ever be built again.

The rule proposed: **every design must be reachable from raw materials, skill, and time, even if that is extremely slow.** There is always a hand route, like the three plates. That route may take years of game time, but it exists. The verifier can check this on the whole production graph: no cycle may be unbreakable. For example, if making chips needs chips, there must be a slower route that makes a first crude chip without one.

Prehistory should also leave the universe with some redundancy: more than one factory for critical things, and powers that have reasons to protect them.

## Not getting carried away

Real chains are thousands of steps deep. This one shouldn't be. The layers in [world-engine.md](world-engine.md) are where the depth is cut:

- Aim for a handful of steps per layer, not dozens.
- Below a layer's boundary, parts are bought or found, not traced back to atoms.
- Deep chains matter most when the ordinary supply fails: the stranded crew, a frontier, or a collapse. In ordinary play, people buy parts, the way nobody makes their own pencil.

## Related

- [knowledge.md](knowledge.md): knowledge lives somewhere, and can be lost.
- [code-and-laws.md](code-and-laws.md): designs as data, programs as behaviour.
- [money.md](money.md): chains are why prices differ, and why trade exists.
