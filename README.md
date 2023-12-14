# lethalfixshaders

Automatically creates shader directories that Unity expects, and copies all materials into their appropriate folders based on the guid.

Usage: `lethalfixshaders --shaders-path <SHADERS_PATH> --material-path <MATERIAL_PATH> [--output-shaders-path <OUTPUT_SHADERS_PATH>]`

Arguments:

- `--shaders-path`: Path to the shaders folder in your project. This is usually `Assets/Shaders`.
- `--material-path`: Path to the materials folder in your project. This is usually `Assets/Materials`.
- `--output-shaders-path`(optional): Path to the output shaders folder in your project. It will be created if it doesn't exist already, and it will default to your shaders path + "Out" if not specified, e.g. `Assets/ShadersOut`.
