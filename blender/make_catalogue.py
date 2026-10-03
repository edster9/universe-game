"""Builds blender/catalogue.blend from blender/catalogue.toml: one collection
for each model of each entry, named "<entry>/<n>", marked as an asset so it
can be dragged in from Blender's asset browser, and carrying the entry's id
and kind as custom properties. Models come from the packs in
assets/third-party (see each pack's README); entries without a model, or
whose pack isn't there, get a simple shape made here.

The file holds the packs' models, so git ignores it: run this again after
downloading the packs. World files link to it rather than copying from it.

Run it in Blender, without its window:
    blender --background --factory-startup --python blender/make_catalogue.py
(from WSL: blender/blender.sh make_catalogue.py)
"""

import math
import os
import tomllib

import bmesh
import bpy

HERE = os.path.dirname(os.path.abspath(__file__))
PACKS = os.path.join(HERE, "..", "assets", "third-party")
OUT = os.path.join(HERE, "catalogue.blend")


def clear():
    bpy.ops.wm.read_factory_settings(use_empty=True)


def material(name, hex_colour):
    mat = bpy.data.materials.get(name)
    if mat:
        return mat
    h = hex_colour.lstrip("#")
    rgb = [int(h[i : i + 2], 16) / 255 for i in (0, 2, 4)]
    # sRGB to linear, as Blender's colours are.
    lin = [c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4 for c in rgb]
    mat = bpy.data.materials.new(name)
    mat.diffuse_color = (*lin, 1.0)
    try:
        mat.use_nodes = True
    except AttributeError:
        pass
    if mat.node_tree:
        bsdf = mat.node_tree.nodes.get("Principled BSDF")
        if bsdf:
            bsdf.inputs["Base Color"].default_value = (*lin, 1.0)
            bsdf.inputs["Roughness"].default_value = 0.9
    return mat


def mesh_object(name, build, colour):
    """A simple shape, made with bmesh: `build(bm)` adds its geometry."""
    bm = bmesh.new()
    build(bm)
    me = bpy.data.meshes.new(name)
    bm.to_mesh(me)
    bm.free()
    me.materials.append(material(name, colour))
    return bpy.data.objects.new(name, me)


def rod(bm, length, radius, at=(0, 0, 0), turn=0.0, lie=True):
    """A cylinder lying along x (or standing, if not `lie`)."""
    m = bmesh.ops.create_cone(
        bm, cap_ends=True, segments=8, radius1=radius, radius2=radius, depth=length
    )
    verts = m["verts"]
    if lie:
        bmesh.ops.rotate(
            bm, verts=verts, cent=(0, 0, 0), matrix=_rot("Y", math.pi / 2)
        )
    bmesh.ops.rotate(bm, verts=verts, cent=(0, 0, 0), matrix=_rot("Z", turn))
    bmesh.ops.translate(bm, verts=verts, vec=at)


def lump(bm, radius, squash, at=(0, 0, 0), subdivisions=1):
    m = bmesh.ops.create_icosphere(bm, subdivisions=subdivisions, radius=radius)
    for v in m["verts"]:
        v.co.z *= squash
        v.co.x += at[0]
        v.co.y += at[1]
        v.co.z += at[2] + radius * squash
    return m


def _rot(axis, angle):
    from mathutils import Matrix

    return Matrix.Rotation(angle, 3, axis)


# The simple shapes, about their real size, resting on the ground at z = 0.
SHAPES = {
    "log": lambda bm: rod(bm, 1.2, 0.13, at=(0, 0, 0.13)),
    "stick": lambda bm: [
        rod(bm, 0.9, 0.025, at=(0, i * 0.12 - 0.12, 0.025), turn=i * 0.4)
        for i in range(3)
    ],
    "twig": lambda bm: [
        rod(bm, 0.35, 0.01, at=(0, i * 0.06 - 0.12, 0.01), turn=i * 0.7)
        for i in range(5)
    ],
    "shard": lambda bm: [
        lump(bm, 0.12, 0.6, at=(i * 0.25 - 0.25, (i % 2) * 0.2, 0), subdivisions=0)
        for i in range(3)
    ],
    "nugget": lambda bm: [
        lump(bm, 0.14, 0.7, at=(i * 0.3 - 0.3, (i % 2) * 0.25, 0)) for i in range(3)
    ],
    "nuts": lambda bm: [
        lump(bm, 0.06, 1.0, at=(math.cos(i) * 0.2, math.sin(i) * 0.2, 0))
        for i in range(6)
    ],
    "shell": lambda bm: [
        lump(bm, 0.06, 0.35, at=(i * 0.15 - 0.3, (i % 3) * 0.1, 0)) for i in range(5)
    ],
    "mound": lambda bm: lump(bm, 1.5, 0.25, subdivisions=2),
    "patch": lambda bm: bmesh.ops.create_circle(
        bm, cap_ends=True, segments=24, radius=3.0
    ),
    "pool": lambda bm: bmesh.ops.create_circle(
        bm, cap_ends=True, segments=32, radius=3.0
    ),
    "fish": lambda bm: lump(bm, 0.2, 0.4, at=(0, 0, -0.05)),
    "person": lambda bm: rod(bm, 1.75, 0.2, at=(0, 0, 0.875), lie=False),
    "beast": lambda bm: rod(bm, 1.2, 0.35, at=(0, 0, 0.45)),
}


def import_model(path):
    """Imports a glTF model; returns its objects, unlinked from the scene."""
    before = set(bpy.data.objects)
    # Textures stay in the pack, referred to, not copied in.
    bpy.ops.import_scene.gltf(filepath=path, import_pack_images=False)
    new = [o for o in bpy.data.objects if o not in before]
    for o in new:
        for c in list(o.users_collection):
            c.objects.unlink(o)
    return new


def main():
    clear()
    with open(os.path.join(HERE, "catalogue.toml"), "rb") as f:
        entries = tomllib.load(f)["entry"]
    root = bpy.data.collections.new("catalogue")
    bpy.context.scene.collection.children.link(root)
    made, missing = 0, []
    for entry in entries:
        models = [os.path.join(PACKS, m) for m in entry.get("models", [])]
        found = [m for m in models if os.path.isfile(m)]
        missing += [m for m in models if not os.path.isfile(m)]
        # Each model is a variant; with none, the entry's own shape.
        variants = found or [None]
        for n, path in enumerate(variants, start=1):
            coll = bpy.data.collections.new(f"{entry['id']}/{n}")
            if path:
                objects = import_model(path)
                scale = entry.get("scale", 1.0)
                for o in objects:
                    if o.parent is None:
                        o.scale = [s * scale for s in o.scale]
            else:
                shape = entry.get("shape", "nugget")
                objects = [
                    mesh_object(
                        f"{entry['id']} ({shape})",
                        SHAPES[shape],
                        entry.get("colour", "#909090"),
                    )
                ]
            for o in objects:
                coll.objects.link(o)
            coll["ug_entry"] = entry["id"]
            coll["ug_is"] = entry["is"]
            coll.asset_mark()
            coll.asset_data.description = (
                f"{entry['is']}: {entry.get('label', entry['id'])}"
            )
            coll.asset_data.tags.new(entry["is"])
            root.children.link(coll)
            # Kept out of view in this file: it's a library.
            bpy.context.view_layer.layer_collection.children["catalogue"].children[
                coll.name
            ].exclude = True
            made += 1
    bpy.ops.file.make_paths_relative()
    bpy.ops.wm.save_as_mainfile(filepath=OUT)
    print(f"CATALOGUE {made} collections from {len(entries)} entries -> {OUT}")
    for m in missing:
        print(f"MISSING {m} (drawn as a simple shape)")


main()
