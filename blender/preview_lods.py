"""Renders a catalogue entry's models at each level of detail side by side,
the full model first, to judge the simpler copies make_catalogue.py makes
(the `lods` of an entry in catalogue.toml).

    blender/blender.sh preview_lods.py <entry> <out.png> [models...]
    e.g. blender/blender.sh preview_lods.py standing-trees C:\\temp\\lods.png 1 6

Models are numbered as in the catalogue (entry/1, entry/2, ...); the first
two if none are given. Each row is a model; each column a level.
"""

import math
import os
import sys

import bpy
from mathutils import Vector

HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[sys.argv.index("--") + 1 :]
entry, out = args[0], args[1]
numbers = args[2:] or ["1", "2"]

bpy.ops.wm.read_factory_settings(use_empty=True)
names = [f"{entry}/{n}" for n in numbers]
with bpy.data.libraries.load(os.path.join(HERE, "catalogue.blend"), link=False) as (src, dst):
    dst.collections = [c for c in src.collections if c in names]
scene = bpy.context.scene
width, row = 0.0, 0.0
for r, coll in enumerate(dst.collections):
    levels = sorted({o.get("ug_lod", 0) for o in coll.objects})
    size = max((max(o.dimensions) for o in coll.objects), default=1.0)
    for c, level in enumerate(levels):
        holder = bpy.data.objects.new(f"{coll.name} level {level}", None)
        holder.location = (c * size * 1.3, 0.0, -r * size * 1.3)
        scene.collection.objects.link(holder)
        for o in coll.objects:
            if o.type != "MESH" or o.get("ug_lod", 0) != level:
                continue
            copy = o.copy()
            copy.parent = holder
            scene.collection.objects.link(copy)
            tris = sum(len(p.vertices) - 2 for p in o.data.polygons)
            print(f"LEVEL {coll.name} {level} {o.name}: {tris} triangles")
        width = max(width, (len(levels) - 1) * size * 1.3)
    row = r * size * 1.3

sun = bpy.data.objects.new("sun", bpy.data.lights.new("sun", "SUN"))
sun.data.energy = 3.0
sun.rotation_euler = (math.radians(50), math.radians(10), math.radians(30))
scene.collection.objects.link(sun)
cam = bpy.data.objects.new("cam", bpy.data.cameras.new("cam"))
cam.data.type = "ORTHO"
cam.data.ortho_scale = max(width, row) * 1.25 + 12.0
cam.rotation_euler = (math.radians(90), 0, 0)
cam.location = (width / 2, -60.0, -row / 2 + 4.0)
scene.collection.objects.link(cam)
scene.camera = cam
world = bpy.data.worlds.new("sky")
world.color = (0.55, 0.72, 0.9)
scene.world = world
scene.render.engine = "BLENDER_EEVEE_NEXT" if "BLENDER_EEVEE_NEXT" in [
    e.identifier for e in bpy.types.RenderSettings.bl_rna.properties["engine"].enum_items
] else "BLENDER_EEVEE"
scene.render.resolution_x, scene.render.resolution_y = 1600, 900
scene.view_settings.view_transform = "Standard"
scene.render.filepath = out
bpy.ops.render.render(write_still=True)
print(f"PREVIEW -> {out}")
