# Source worlds

The worlds as built, one folder each: `worlds/<world>/` holds its Blender files and its manifest (`<world>.world.toml`: which files make it, and its settings). **They're not in git**: they're binary, change often, and, while we train the system, temporary (the owner, 2026-10-02). They live in an S3 bucket, `universe-game-worlds`, public to read, which keeps every version sent.

    blender/worlds.sh list               the worlds there are
    blender/worlds.sh pull skill-yard    fetch one into worlds/skill-yard
    blender/worlds.sh push skill-yard    send back what you changed (needs the AWS profile)
    blender/worlds.sh versions skill-yard

Then `blender/convert.sh skill-yard` makes it into the game's data (see [blender/README.md](../blender/README.md)).

| Folder | What's in it | In git |
| --- | --- | --- |
| `worlds/<world>/` | The source: Blender files and the manifest | No: S3 |
| `blender/` | The tools, and the catalogue they make | The tools and `catalogue.toml` |
| `assets/worlds/<world>/` | The export, which the client draws | No: it holds the packs' models |
| `data/<world>.toml` | The world's data, which the engine loads | Yes |
