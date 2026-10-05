"""Builds a forest world for the rendering benchmarks
(docs/research/rendering-benchmarks.md): trees and undergrowth scattered
over a square, the same every time for the same arguments, a clearing in
the middle where the player stands, and nothing else. Everything in it is
scenery, as most of a big world's forest would be: drawn, never used.

    blender/blender.sh make_forest.py <name> <trees> <half-width in metres>
    e.g. blender/blender.sh make_forest.py forest-small 200 50

It writes worlds/<name>/<name>.blend and its manifest; blender/convert.sh
<name> then exports it and makes data/<name>.toml. These worlds are made,
not built by hand, so they live nowhere but on the machine that made them:
blender/bench.sh makes any that are missing.
"""

import math
import os
import random
import sys
import tomllib

import bpy

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOGUE = os.path.join(HERE, "catalogue.blend")

# What grows under each tree, on average, by catalogue entry: the forest's
# floor is mostly grass, with bushes, ferns, and the odd rock.
UNDERGROWTH = {
    "dry-grass": 1.5,
    "fibrous-bushes": 0.3,
    "fern": 0.3,
    "flowers": 0.15,
    "mushrooms": 0.1,
    "loose-stones": 0.1,
    "landmark-rock": 0.03,
}
# A few trees are dead or twisted, for variety.
ODD_TREES = {"dead-tree": 0.03, "twisted-tree": 0.02}
CLEARING = 8.0  # metres round the middle with nothing in it


def link_catalogue():
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


def scatter(catalogue, entry, count, half, into, rng, size=(0.8, 1.25)):
    options = variants(catalogue, entry)
    placed = 0
    while placed < count:
        x, y = rng.uniform(-half, half), rng.uniform(-half, half)
        if math.hypot(x, y) < CLEARING:
            continue
        o = bpy.data.objects.new(f"{entry} {placed + 1}", None)
        o.instance_type = "COLLECTION"
        o.instance_collection = rng.choice(options)
        o.location = (x, y, 0.0)
        o.rotation_euler.z = rng.uniform(0.0, math.tau)
        o.scale = (rng.uniform(*size),) * 3
        into.objects.link(o)
        placed += 1


def ground(half, into):
    mat = bpy.data.materials.new("forest ground")
    mat.diffuse_color = (0.36, 0.42, 0.24, 1.0)
    try:
        mat.use_nodes = True
    except AttributeError:
        pass
    if mat.node_tree:
        bsdf = mat.node_tree.nodes.get("Principled BSDF")
        if bsdf:
            bsdf.inputs["Base Color"].default_value = (0.36, 0.42, 0.24, 1.0)
            bsdf.inputs["Roughness"].default_value = 0.95
    bpy.ops.mesh.primitive_plane_add(size=2 * half + 60.0, location=(0, 0, 0))
    o = bpy.context.active_object
    o.name = "ground"
    o.data.materials.append(mat)
    for c in list(o.users_collection):
        c.objects.unlink(o)
    into.objects.link(o)


def toml_value(v):
    if isinstance(v, str):
        return f'"{v}"'
    if isinstance(v, list):
        return "[" + ", ".join(toml_value(x) for x in v) + "]"
    return str(v)


def manifest(name, folder):
    """The world's manifest, with the skill yard's settings for the world."""
    with open(os.path.join(HERE, "..", "data", "skill-yard.toml"), "rb") as f:
        settings = tomllib.load(f)["world"]
    lines = [
        f"# A forest for the rendering benchmarks, made by blender/make_forest.py.",
        f'files = ["{name}"]',
        'catalogue = "../../blender/catalogue.toml"',
        'exported = "../../assets/worlds"',
        f'output = "../../data/{name}.toml"',
        'uses = ["island-things.toml"]',
        "",
        "[world]",
    ]
    lines += [f"{k} = {toml_value(v)}" for k, v in settings.items()]
    with open(os.path.join(folder, f"{name}.world.toml"), "w") as f:
        f.write("\n".join(lines) + "\n")


def main():
    args = sys.argv[sys.argv.index("--") + 1 :]
    name, trees, half = args[0], int(args[1]), float(args[2])
    folder = os.path.join(HERE, "..", "worlds", name)
    out = os.path.join(folder, f"{name}.blend")
    os.makedirs(folder, exist_ok=True)
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.ops.wm.save_as_mainfile(filepath=out)
    catalogue = link_catalogue()
    scene = bpy.context.scene.collection
    view = bpy.data.collections.new("scenery")
    scene.children.link(view)
    marks = bpy.data.collections.new("places and things")
    scene.children.link(marks)

    place = bpy.data.objects.new("place: forest", None)
    place.empty_display_type = "CIRCLE"
    place.empty_display_size = half
    place["ug_place"] = "forest"
    place["ug_label"] = "the forest"
    place["ug_size"] = f"{half:g} m"
    place["ug_temperature"] = "300 K"
    place["ug_night"] = "292 K"
    marks.objects.link(place)
    player = bpy.data.objects.new("thing: player", None)
    player.instance_type = "COLLECTION"
    player.instance_collection = variants(catalogue, "player")[0]
    player["ug_entry"] = "player"
    player["ug_id"] = "player"
    marks.objects.link(player)

    rng = random.Random(1)
    ground(half, view)
    odd = sum(round(trees * share) for share in ODD_TREES.values())
    scatter(catalogue, "standing-trees", trees - odd, half, view, rng)
    for entry, share in ODD_TREES.items():
        scatter(catalogue, entry, round(trees * share), half, view, rng)
    for entry, per_tree in UNDERGROWTH.items():
        scatter(catalogue, entry, round(trees * per_tree), half, view, rng)

    bpy.context.scene["ug_world"] = name
    bpy.ops.wm.save_as_mainfile(filepath=out, relative_remap=True)
    manifest(name, folder)
    print(f"WORLD {name}: {len(view.objects)} scenery objects, {trees} trees, "
          f"{2 * half:g} m across -> {out}")


main()
