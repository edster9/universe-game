"""Built worlds/skill-yard/skill-yard.blend, the first world made in Blender: the skill
grounds again, as one walled square yard 100 m across. It's a starting
point: after this, the .blend file is the master, edited by hand in Blender,
and this script isn't run again unless we start over.

What's in it, by the three kinds (docs/ideas/world-building.md):
- **Scenery** (anything without `ug_entry`, or an instance of a scenery
  entry): the ground, the walls, stepping stones, flowers, ferns, mushrooms,
  a dead tree and a twisted tree.
- **Sources**: each an empty marking its area (custom properties `ug_entry`,
  `ug_id`, and maybe `ug_label` or `ug_mass`), with the models that show it
  as its children: a stand's trees, a pile's stones. Add or remove children
  to change how much there is; move the empty to move the whole source.
- **Things**: instances of catalogue entries (a landmark rock, the player,
  the boars), with `ug_entry`, `ug_id`, and maybe `ug_label`.
- **Places**: empties with `ug_place` (and `ug_label`, `ug_size`): the yard,
  and the deep woods in its north-east corner, where the boars live.

Blender's axes are the world's: x east, y north, z up, in metres. The yard's
middle is 0, 0. Models are linked from catalogue.blend, never copied, so
this file holds none of a pack's models and can be kept in git.

Run it in Blender, without its window (from WSL: blender/blender.sh
make_skill_yard.py [preview.png]); it saves the file, and renders a preview
picture if given a path.
"""

import math
import os
import sys

import bpy
from mathutils import Vector

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOGUE = os.path.join(HERE, "catalogue.blend")
OUT = os.path.join(HERE, "..", "worlds", "skill-yard", "skill-yard.blend")
HALF = 50.0  # the yard is 100 m across
GOLDEN = math.pi * (3 - math.sqrt(5))


def link_catalogue():
    """Every collection in the catalogue, linked: name -> collection."""
    with bpy.data.libraries.load(CATALOGUE, link=True, relative=True) as (src, dst):
        dst.collections = [c for c in src.collections if "/" in c]
    return {c.name: c for c in dst.collections}


def variants(catalogue, entry):
    found = sorted(
        (c for name, c in catalogue.items() if name.split("/")[0] == entry),
        key=lambda c: c.name,
    )
    if not found:
        raise SystemExit(f"Error: no entry {entry} in the catalogue")
    return found


def collection(name):
    c = bpy.data.collections.new(name)
    bpy.context.scene.collection.children.link(c)
    return c


def instance(catalogue, entry, at, into, n=0, turn=None, name=None):
    """A placed model: an instance of one of the entry's collections."""
    options = variants(catalogue, entry)
    chosen = options[n % len(options)]
    o = bpy.data.objects.new(name or chosen.name, None)
    o.instance_type = "COLLECTION"
    o.instance_collection = chosen
    o.location = at
    o.rotation_euler.z = turn if turn is not None else (n * 2.399) % math.tau
    into.objects.link(o)
    return o


def bearing(degrees, distance):
    """A point `distance` metres out at a compass bearing from the middle."""
    r = math.radians(degrees)
    return Vector((math.sin(r) * distance, math.cos(r) * distance, 0.0))


# The sources, with how many models show each and over how many metres.
# Count is how much there is, for sources whose models each stand for a mass.
SOURCES = {
    "standing-trees": (12, 6.0),
    "fibrous-bushes": (5, 2.0),
    "dry-grass": (5, 0.8),
    "loose-stones": (10, 0.8),
    "flint-nodules": (5, 0.8),
    "fallen-sticks": (5, 0.8),
    "dry-twigs": (5, 0.8),
    "deadwood": (5, 0.8),
    "roots-and-nuts": (5, 0.8),
    "forest-floor": (1, 6.0),
    "water": (1, 3.0),
    "fish": (1, 2.5),
    "mussels": (1, 0.8),
    "bog-iron": (1, 0.8),
    "clay-bank": (1, 0.8),
}

# The six zones, as on the skill grounds: a compass bearing from the middle,
# the landmark's name and id, and what lies round each zone's middle, by
# catalogue entry and short id.
ZONES = [
    (0, "woodland", "the woodland", [
        ("roots-and-nuts", "forage"), ("flint-nodules", "flint"),
        ("loose-stones", "stones"), ("fallen-sticks", "sticks"),
        ("dry-grass", "grass"), ("dry-twigs", "twigs"),
        ("fibrous-bushes", "bushes"),
    ]),
    (60, "grove", "the fibre grove", [
        ("fibrous-bushes", "bushes"), ("fallen-sticks", "sticks"),
        ("deadwood", "logs"),
    ]),
    (120, "stream", "the stream bank", [
        ("water", "water"), ("fish", "fish"), ("mussels", "mussels"),
        ("roots-and-nuts", "forage"), ("flint-nodules", "flint"),
        ("loose-stones", "stones"), ("fallen-sticks", "sticks"),
    ]),
    (180, "forge", "the forge", [
        ("bog-iron", "bog"), ("clay-bank", "clay"), ("loose-stones", "stones"),
        ("flint-nodules", "flint"), ("dry-grass", "grass"),
        ("dry-twigs", "twigs"), ("fallen-sticks", "sticks"),
        ("deadwood", "logs"),
    ]),
    (240, "hearth", "the hearth", [
        ("dry-grass", "grass"), ("dry-twigs", "twigs"),
        ("fallen-sticks", "sticks"), ("deadwood", "logs"),
        ("loose-stones", "stones"),
    ]),
    (300, "knapping", "the knapping ground", [
        ("flint-nodules", "flint"), ("loose-stones", "stones"),
        ("fallen-sticks", "sticks"),
    ]),
]


def source(catalogue, entry, ident, at, into, label=None, mass=None, count=None):
    """A source: an empty marking its area, its models as children."""
    n, spread = SOURCES[entry]
    n = count or n
    mark = bpy.data.objects.new(f"source: {ident}", None)
    mark.empty_display_type = "CIRCLE"
    mark.empty_display_size = spread
    mark.location = at
    mark["ug_entry"] = entry
    mark["ug_id"] = ident
    if label:
        mark["ug_label"] = label
    if mass:
        mark["ug_mass"] = mass
    into.objects.link(mark)
    for i in range(n):
        # Spread evenly over the area, the same every time.
        r = spread * 0.9 * math.sqrt((i + 0.5) / n) if n > 1 else 0.0
        offset = Vector((math.cos(i * GOLDEN) * r, math.sin(i * GOLDEN) * r, 0.0))
        child = instance(catalogue, entry, offset, into, n=i, name=f"{ident} {i + 1}")
        child.parent = mark
        if entry in ("water", "forest-floor"):
            child.scale = (spread / 3.0,) * 3
            # Just above the ground, or the ground hides it.
            child.location.z = 0.03 if entry == "water" else 0.015
    return mark


def thing(catalogue, entry, ident, at, into, label=None, n=0):
    o = instance(catalogue, entry, at, into, n=n, name=f"thing: {ident}")
    o["ug_entry"] = entry
    o["ug_id"] = ident
    if label:
        o["ug_label"] = label
    return o


def place(ident, label, at, size, into):
    o = bpy.data.objects.new(f"place: {ident}", None)
    o.empty_display_type = "CIRCLE"
    o.empty_display_size = size
    o.location = at
    o["ug_place"] = ident
    o["ug_label"] = label
    o["ug_size"] = f"{size:g} m"
    o["ug_temperature"] = "300 K"
    o["ug_night"] = "292 K"
    into.objects.link(o)
    return o


def plain(name, colour):
    mat = bpy.data.materials.new(name)
    mat.diffuse_color = (*colour, 1.0)
    try:
        mat.use_nodes = True
    except AttributeError:
        pass
    if mat.node_tree:
        bsdf = mat.node_tree.nodes.get("Principled BSDF")
        if bsdf:
            bsdf.inputs["Base Color"].default_value = (*colour, 1.0)
            bsdf.inputs["Roughness"].default_value = 0.95
    return mat


def box(name, size, at, mat, into):
    bpy.ops.mesh.primitive_cube_add(size=1.0, location=at)
    o = bpy.context.active_object
    o.name = name
    o.scale = size
    bpy.ops.object.transform_apply(scale=True)
    o.data.materials.append(mat)
    for c in list(o.users_collection):
        c.objects.unlink(o)
    into.objects.link(o)
    return o


def scenery(catalogue, into):
    ground = plain("ground", (0.55, 0.47, 0.30))
    stone = plain("wall stone", (0.42, 0.40, 0.37))
    box("ground", (2 * HALF + 4, 2 * HALF + 4, 0.2), (0, 0, -0.1), ground, into)
    for name, size, at in [
        ("wall north", (2 * HALF + 1, 1.0, 3.0), (0, HALF + 0.5, 1.5)),
        ("wall south", (2 * HALF + 1, 1.0, 3.0), (0, -HALF - 0.5, 1.5)),
        ("wall east", (1.0, 2 * HALF + 1, 3.0), (HALF + 0.5, 0, 1.5)),
        ("wall west", (1.0, 2 * HALF + 1, 3.0), (-HALF - 0.5, 0, 1.5)),
    ]:
        box(name, size, at, stone, into)
    # Stepping stones from the clearing towards each zone.
    for degrees, *_ in ZONES:
        for k, d in enumerate((6.0, 10.0, 14.0)):
            instance(catalogue, "stepping-stones", bearing(degrees, d), into, n=k)
    # Decoration along the walls and between the zones.
    decor = ["flowers", "fern", "clover", "mushrooms"]
    for i in range(40):
        side = i % 4
        t = (i * 0.618034) % 1.0 * 2 * HALF - HALF
        inset = HALF - 2.0 - (i % 3)
        at = {
            0: (t, inset), 1: (inset, t), 2: (t, -inset), 3: (-inset, t),
        }[side]
        instance(catalogue, decor[i % 4], Vector((*at, 0.0)), into, n=i)
    for i, degrees in enumerate(range(30, 360, 60)):
        instance(catalogue, decor[i % 4], bearing(degrees, 18.0), into, n=i)
    instance(catalogue, "dead-tree", Vector((-42.0, -42.0, 0.0)), into)
    instance(catalogue, "twisted-tree", Vector((42.0, -40.0, 0.0)), into)


def light_and_camera():
    sun = bpy.data.lights.new("sun", "SUN")
    sun.energy = 3.0
    s = bpy.data.objects.new("sun", sun)
    s.rotation_euler = (math.radians(50), math.radians(10), math.radians(30))
    bpy.context.scene.collection.objects.link(s)
    world = bpy.data.worlds.new("sky")
    world.color = (0.55, 0.72, 0.9)
    try:
        world.use_nodes = True
    except AttributeError:
        pass
    if world.node_tree:
        bg = world.node_tree.nodes.get("Background")
        if bg:
            bg.inputs["Color"].default_value = (0.55, 0.72, 0.9, 1.0)
            bg.inputs["Strength"].default_value = 0.6
    bpy.context.scene.world = world
    cam = bpy.data.cameras.new("overview")
    cam.lens = 24
    c = bpy.data.objects.new("overview", cam)
    c.location = (-62.0, -70.0, 48.0)
    direction = Vector((0.0, 4.0, 0.0)) - c.location
    c.rotation_euler = direction.to_track_quat("-Z", "Y").to_euler()
    bpy.context.scene.collection.objects.link(c)
    bpy.context.scene.camera = c
    # A second view: from the clearing, looking north to the woodland.
    near = bpy.data.objects.new("from the clearing", bpy.data.cameras.new("near"))
    near.data.lens = 28
    near.location = (6.0, -6.0, 6.0)
    direction = Vector((0.0, 26.0, 1.0)) - near.location
    near.rotation_euler = direction.to_track_quat("-Z", "Y").to_euler()
    bpy.context.scene.collection.objects.link(near)
    bpy.context.scene.view_settings.view_transform = "Standard"


def main():
    bpy.ops.wm.read_factory_settings(use_empty=True)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    # Saved first, so the catalogue is linked by a path relative to it.
    bpy.ops.wm.save_as_mainfile(filepath=OUT)
    catalogue = link_catalogue()
    places = collection("places")
    things = collection("things")
    sources = collection("sources")
    view = collection("scenery")

    place("yard", "the skill yard", (0, 0, 0), HALF, places)
    deep = Vector((36.0, 36.0, 0.0))
    place("deep-woods", "the deep woods", deep, 13.0, places)

    thing(catalogue, "player", "player", (0, 0, 0), things)
    for degrees, ident, label, stocks in ZONES:
        middle = bearing(degrees, 24.0)
        thing(catalogue, "landmark-rock", ident, bearing(degrees, 30.5), things,
              label=label, n=degrees // 60)
        for k, (entry, short) in enumerate(stocks):
            at = middle + bearing(360 * k / len(stocks), 4.0)
            source(catalogue, entry, f"{ident}-{short}", at, sources)
    # The woodland's stand of trees, behind its landmark, on its forest floor.
    source(catalogue, "standing-trees", "woodland-trees", bearing(0, 39.0), sources)
    source(catalogue, "forest-floor", "floor", bearing(0, 39.0), sources)

    # The deep woods: trees, a spring, forage, and two boars.
    source(catalogue, "standing-trees", "deep-trees", deep, sources, count=24)
    source(catalogue, "forest-floor", "deep-floor", deep + Vector((0, 0, 0.01)),
           sources)
    source(catalogue, "water", "spring", deep + Vector((-6.0, -6.0, 0.0)), sources,
           label="a spring", mass="20 t")
    source(catalogue, "roots-and-nuts", "deep-forage", deep + Vector((4.0, -7.0, 0)),
           sources)
    for i in range(2):
        thing(catalogue, "boar", f"boar-{i + 1}", deep + Vector((-3.0 + 6 * i, 2.0, 0)),
              things)

    scenery(catalogue, view)
    light_and_camera()
    bpy.context.scene["ug_world"] = "skill-yard"
    bpy.ops.wm.save_as_mainfile(filepath=OUT, relative_remap=True)
    counts = {
        "things": len(things.objects),
        "sources": sum(1 for o in sources.objects if "ug_entry" in o),
        "drawn in sources": sum(1 for o in sources.objects if o.parent),
        "scenery objects": len(view.objects),
    }
    print(f"WORLD {counts} -> {OUT}")

    args = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    if args:
        render(args[0])


def render(path):
    scene = bpy.context.scene
    for engine in ("BLENDER_EEVEE", "BLENDER_EEVEE_NEXT", "BLENDER_WORKBENCH"):
        try:
            scene.render.engine = engine
            break
        except TypeError:
            continue
    scene.render.resolution_x = 1600
    scene.render.resolution_y = 900
    scene.render.filepath = path
    try:
        scene.eevee.taa_render_samples = 16
    except AttributeError:
        pass
    for camera in ("overview", "from the clearing"):
        scene.camera = bpy.data.objects[camera]
        stem, ext = os.path.splitext(path)
        scene.render.filepath = path if camera == "overview" else f"{stem}-near{ext}"
        bpy.ops.render.render(write_still=True)
        print(f"PREVIEW {scene.render.engine} -> {scene.render.filepath}")
    scene.camera = bpy.data.objects["overview"]


main()
