"""Exports a world's Blender file as glTF, with its tags (custom properties)
as glTF extras, for the converter (blender/convert.sh runs this for each file
a manifest lists). The export holds the packs' models, so it goes where git
doesn't look: assets/worlds/<name>/<name>.gltf. The client draws the world's
scenery from it.

    blender --background --factory-startup --python blender/export_world.py -- <file.blend> <out.gltf>
"""

import sys

import bpy

args = sys.argv[sys.argv.index("--") + 1 :]
source, out = args[0], args[1]
bpy.ops.wm.open_mainfile(filepath=source)
bpy.ops.export_scene.gltf(
    filepath=out,
    export_format="GLTF_SEPARATE",
    export_extras=True,
    export_cameras=False,
    export_lights=False,
    export_yup=True,
)
print(f"EXPORTED {source} -> {out}")
