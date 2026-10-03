"""Points a world's Blender file at the catalogue again, after the file has
moved: its link to blender/catalogue.blend is kept relative to where the
file is, so a move breaks it. Nothing else in the file changes.

    blender --background --factory-startup --python blender/relink.py -- <file.blend>
(from WSL: blender/blender.sh relink.py <file.blend>, the file's path given
as Windows sees it)
"""

import os
import sys

import bpy

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOGUE = os.path.join(HERE, "catalogue.blend")

path = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.open_mainfile(filepath=path)
for library in bpy.data.libraries:
    if os.path.basename(bpy.path.abspath(library.filepath)) == "catalogue.blend":
        library.filepath = bpy.path.relpath(CATALOGUE)
        library.reload()
        print(f"RELINKED {library.filepath}")
bpy.ops.wm.save_mainfile()
