# Third-party assets

Models, textures, and sounds from other makers go here, **and never into git**, for two reasons: their size, and their licences, which differ from pack to pack (some packs we'll buy can't be shared at all). Even a free CC0 pack stays out, so there's one rule for all of them. The owner's decision, 2026-10-02.

What *is* in git is a **README in each pack's folder**, saying where to get the pack, which version, where its files go, and which of its files the game uses. Someone who forks the project downloads each pack and puts its files where the README says. Until they do, the game still runs: anything drawn with a missing model falls back to the shapes the client makes itself (`client/src/shapes.rs`).

`client/run.sh` copies this folder beside the program, where the renderer finds it. A client test checks that every model `client/style.toml` names is listed in its pack's README.

| Pack | Maker | Licence | README |
| --- | --- | --- | --- |
| Stylized Nature MegaKit (Standard) | [Quaternius](https://quaternius.com) | CC0 1.0 | [quaternius/stylized-nature-megakit](quaternius/stylized-nature-megakit/README.md) |

**Adding a pack:** a folder `<maker>/<pack>/` with its README (where to get it, the version, its licence, where its files go, and what we use), and a row here.
