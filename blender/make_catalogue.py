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
from mathutils import Vector

HERE = os.path.dirname(os.path.abspath(__file__))
PACKS = os.path.join(HERE, "..", "assets", "third-party")
# Pictures made here from the packs' models (cards): not in git, like them.
CARDS = os.path.join(HERE, "..", "assets", "generated", "cards")
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


def share_materials(objects):
    """One material for each name: every model imported brings its own copy
    of the pack's materials (Bark_NormalTree.001, .002, ...), and the game
    draws things that share a material together, far faster."""
    for o in objects:
        for slot in getattr(o, "material_slots", []):
            mat = slot.material
            if mat is None:
                continue
            base, _, suffix = mat.name.rpartition(".")
            if base and suffix.isdigit():
                first = bpy.data.materials.get(base)
                if first is not None and first != mat:
                    slot.material = first


def islands(bm):
    """The mesh's separate pieces, as lists of faces."""
    seen, pieces = set(), []
    for start in bm.faces:
        if start.index in seen:
            continue
        piece, todo = [], [start]
        seen.add(start.index)
        while todo:
            f = todo.pop()
            piece.append(f)
            for e in f.edges:
                for g in e.link_faces:
                    if g.index not in seen:
                        seen.add(g.index)
                        todo.append(g)
        pieces.append(piece)
    return pieces


def decimated(mesh, share):
    """The mesh with about `share` of its triangles, by Blender's Decimate."""
    holder = bpy.data.objects.new("decimating", mesh)
    bpy.context.scene.collection.objects.link(holder)
    mod = holder.modifiers.new("simpler", "DECIMATE")
    mod.ratio = share
    mod.use_collapse_triangulate = True
    depsgraph = bpy.context.evaluated_depsgraph_get()
    out = bpy.data.meshes.new_from_object(holder.evaluated_get(depsgraph))
    bpy.data.objects.remove(holder)
    return out


# Foliage: a part made of many small separate pieces (leaf cards). Reducing
# it like a solid would collapse the cards and leave the tree bare, so it's
# thinned instead, whole cards at a time, the rest grown to fill the gap.
CARD_TRIANGLES = 24
MANY_CARDS = 8


def simpler(o, share):
    """A copy of the mesh with about `share` of its triangles, part by part
    (one part per material): solid parts reduced by Decimate, foliage
    thinned by its cards, keeping more of them (the square root of the
    share), since cards are few triangles and most of a tree's look."""
    whole = bmesh.new()
    for index in range(max(1, len(o.material_slots))):
        bm = bmesh.new()
        bm.from_mesh(o.data)
        bmesh.ops.delete(
            bm, geom=[f for f in bm.faces if f.material_index != index], context="FACES"
        )
        bm.faces.ensure_lookup_table()
        if not bm.faces:
            bm.free()
            continue
        bm.faces.index_update()
        pieces = islands(bm)
        part = bpy.data.meshes.new("part")
        if len(pieces) >= MANY_CARDS and len(bm.faces) / len(pieces) <= CARD_TRIANGLES:
            keep = share ** 0.5
            grow = (1 / keep) ** 0.25
            gone = []
            for n, piece in enumerate(pieces):
                if int((n + 1) * keep) == int(n * keep):
                    gone += piece
                    continue
                verts = list({v for f in piece for v in f.verts})
                middle = sum((v.co for v in verts), Vector()) / len(verts)
                for v in verts:
                    v.co = middle + (v.co - middle) * grow
            bmesh.ops.delete(bm, geom=gone, context="FACES")
            bm.to_mesh(part)
        else:
            bm.to_mesh(part)
            reduced = decimated(part, share)
            bpy.data.meshes.remove(part)
            part = reduced
        bm.free()
        whole.from_mesh(part)
        bpy.data.meshes.remove(part)
    mesh = bpy.data.meshes.new(f"{o.data.name} simpler")
    whole.to_mesh(mesh)
    whole.free()
    for mat in o.data.materials:
        mesh.materials.append(mat)
    mesh = with_normals_of(o, mesh)
    # The pack's vertex colours (shading baked in) are kept, but only used,
    # in Blender and in the export, when marked as the ones to use.
    colours = o.data.color_attributes
    if colours.active_color_name and colours.active_color_name in mesh.color_attributes:
        mesh.color_attributes.active_color = mesh.color_attributes[colours.active_color_name]
        mesh.color_attributes.render_color_index = colours.render_color_index
    return mesh


def with_normals_of(o, mesh):
    """The pack's models carry their own normals (soft, rounded shading on
    the leaves), stored in a way that rebuilding the mesh scrambles: they're
    copied back from the full model, each corner from the nearest surface."""
    if "custom_normal" in mesh.attributes:
        mesh.attributes.remove(mesh.attributes["custom_normal"])
    if not o.data.has_custom_normals:
        return mesh
    scene = bpy.context.scene.collection
    scene.objects.link(o)
    holder = bpy.data.objects.new("normals", mesh)
    holder.matrix_world = o.matrix_world
    scene.objects.link(holder)
    mod = holder.modifiers.new("normals", "DATA_TRANSFER")
    mod.object = o
    mod.use_loop_data = True
    mod.data_types_loops = {"CUSTOM_NORMAL"}
    mod.loop_mapping = "POLYINTERP_NEAREST"
    depsgraph = bpy.context.evaluated_depsgraph_get()
    out = bpy.data.meshes.new_from_object(holder.evaluated_get(depsgraph))
    bpy.data.objects.remove(holder)
    bpy.data.meshes.remove(mesh)
    scene.objects.unlink(o)
    return out


def simpler_copies(objects, shares, coll):
    """For each mesh, copies with fewer triangles (each about a share of
    them), for drawing far away: tagged `ug_lod` 1, 2, ..., and the full
    mesh 0. The game picks one by distance (the `detail` setting); Blender
    shows them all, one inside another."""
    copies = []
    for o in objects:
        if o.type != "MESH":
            continue
        o["ug_lod"] = 0
        for level, share in enumerate(shares, start=1):
            copy = o.copy()
            copy.data = simpler(o, share)
            copy.name = f"{o.name} lod{level}"
            copy["ug_lod"] = level
            copies.append(copy)
    for c in copies:
        coll.objects.link(c)
    return copies


def world_matrix(o):
    """Where an object is in its collection, through its parents (objects
    outside a scene don't keep their world matrix up to date)."""
    m = o.matrix_basis.copy()
    while o.parent is not None:
        m = o.parent.matrix_basis @ o.matrix_parent_inverse @ m
        o = o.parent
    return m


def photograph(coll, shown, name, looking_down, size, middle, top):
    """A picture of the model, flat-lit, on a clear background: from the
    south, or from above."""
    scene = bpy.context.scene
    scene.collection.children.link(coll)
    hidden = [o for o in coll.objects if o not in shown and not o.hide_render]
    for o in hidden:
        o.hide_render = True
    cam = bpy.data.objects.new("card camera", bpy.data.cameras.new("card camera"))
    cam.data.type = "ORTHO"
    w, h = size
    if looking_down:
        cam.location = (middle.x, middle.y, top + 50.0)
        cam.rotation_euler = (0.0, 0.0, 0.0)
        cam.data.ortho_scale = w * 1.02
        res = (256, 256)
    else:
        cam.location = (middle.x, middle.y - 100.0, h / 2)
        cam.rotation_euler = (math.radians(90), 0.0, 0.0)
        cam.data.ortho_scale = max(w, h) * 1.02
        res = (256, max(64, min(1024, round(256 * h / w))))
    cam.data.clip_end = 500.0
    scene.collection.objects.link(cam)
    scene.camera = cam
    # Lit evenly from a white sky, so the picture is the model's own
    # colours; the game lights the card.
    world = bpy.data.worlds.get("cards") or bpy.data.worlds.new("cards")
    world.color = (1.0, 1.0, 1.0)
    try:
        world.use_nodes = True
    except AttributeError:
        pass
    if world.node_tree:
        bg = world.node_tree.nodes.get("Background")
        if bg:
            bg.inputs["Color"].default_value = (1.0, 1.0, 1.0, 1.0)
            bg.inputs["Strength"].default_value = 1.0
    scene.world = world
    engines = [e.identifier for e in bpy.types.RenderSettings.bl_rna.properties["engine"].enum_items]
    scene.render.engine = "BLENDER_EEVEE_NEXT" if "BLENDER_EEVEE_NEXT" in engines else "BLENDER_EEVEE"
    scene.render.film_transparent = True
    scene.render.resolution_x, scene.render.resolution_y = res
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = "RGBA"
    scene.view_settings.view_transform = "Standard"
    os.makedirs(CARDS, exist_ok=True)
    path = os.path.join(CARDS, f"{name}-{'top' if looking_down else 'side'}.png")
    scene.render.filepath = path
    bpy.ops.render.render(write_still=True)
    bpy.data.objects.remove(cam)
    for o in hidden:
        o.hide_render = False
    scene.collection.children.unlink(coll)
    return path


def card(coll, objects, level):
    """The model as cards, for drawing farthest away: two crossed upright
    pictures of it and one flat one at its crown, each two triangles, lit
    as if facing the sky (so they light alike from every side). Tagged
    `ug_lod` `level`."""
    shown = [o for o in objects if o.type == "MESH"]
    points = [world_matrix(o) @ v.co for o in shown for v in o.data.vertices]
    if not points:
        return None
    lo = Vector([min(p[i] for p in points) for i in range(3)])
    hi = Vector([max(p[i] for p in points) for i in range(3)])
    middle = (lo + hi) / 2
    w = max(hi.x - lo.x, hi.y - lo.y)
    h = hi.z
    name = shown[0].name
    side = photograph(coll, shown, name, False, (w, h), middle, hi.z)
    top = photograph(coll, shown, name, True, (w, h), middle, hi.z)
    # A cut-out material to copy (picture times vertex colour, with a
    # cut-out edge, as the packs' leaves are, and exported so), its picture
    # swapped for the photograph: the model's own if it has one, or else
    # any in the catalogue so far.
    def cut_out(m):
        return m and m.node_tree and any(
            n.type == "BSDF_PRINCIPLED" and n.inputs["Alpha"].is_linked
            for n in m.node_tree.nodes
        )
    own = [m for o in shown for m in o.data.materials]
    template = next((m for m in own + list(bpy.data.materials) if cut_out(m)), None)
    materials = []
    for path in (side, top):
        if template is not None:
            mat = template.copy()
            for n in mat.node_tree.nodes:
                if n.type == "TEX_IMAGE":
                    n.image = bpy.data.images.load(path)
        else:
            mat = bpy.data.materials.new("card")
        mat.name = f"card {os.path.basename(path)[:-4]}"
        materials.append(mat)
    r, crown = w / 2, lo.z + (hi.z - lo.z) * 0.65
    cx, cy = middle.x, middle.y
    verts = [
        (cx - r, cy, 0.0), (cx + r, cy, 0.0), (cx + r, cy, h), (cx - r, cy, h),
        (cx, cy - r, 0.0), (cx, cy + r, 0.0), (cx, cy + r, h), (cx, cy - r, h),
        (cx - r, cy - r, crown), (cx + r, cy - r, crown), (cx + r, cy + r, crown),
        (cx - r, cy + r, crown),
    ]
    faces = [(0, 1, 2, 3), (4, 5, 6, 7), (8, 9, 10, 11)]
    mesh = bpy.data.meshes.new(f"{name} card")
    mesh.from_pydata(verts, [], faces)
    uv = mesh.uv_layers.new(name="UVMap")
    square = [(0, 0), (1, 0), (1, 1), (0, 1)]
    for poly in mesh.polygons:
        for k, li in enumerate(poly.loop_indices):
            uv.data[li].uv = square[k]
        poly.material_index = 1 if poly.index == 2 else 0
    for mat in materials:
        mesh.materials.append(mat)
    colours = mesh.color_attributes.new("Color", "BYTE_COLOR", "CORNER")
    for d in colours.data:
        d.color = (1.0, 1.0, 1.0, 1.0)
    mesh.color_attributes.active_color = colours
    mesh.color_attributes.render_color_index = 0
    mesh.normals_split_custom_set([(0.0, 0.0, 1.0)] * len(mesh.loops))
    o = bpy.data.objects.new(f"{name} card", mesh)
    o["ug_lod"] = level
    coll.objects.link(o)
    return o


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
            share_materials(objects)
            for o in objects:
                coll.objects.link(o)
            if path and entry.get("lods"):
                simpler_copies(objects, entry["lods"], coll)
            if path and entry.get("card"):
                card(coll, objects, len(entry.get("lods", [])) + 1)
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
