## LSDBLOX ASSET PREVIEWER

Requirements:
<br>
[cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
<br>
<br>

## Instructions to build
`cargo build --release`
<br>
<br>

## How to run
Linux/macOS/BSD:

`./asset_preview model.obj [model.png]`
<br>
<br>
That weird one we don't talk about:
<br>

`asset_preview.exe model.obj [model.png]`
<br>

## Controls
Keyboard:
<br>
ADSW: Move camera
<br>
QE: Adjust zoom
<br>
Tab: Invert camera

Mouse:
Right click and moving mouse: Move camera
<br>
Mouse wheel: Adjust zoom
<br>
<br>

## Tips and tricks
If your textures look wrong, even if they look fine in your 3D modeling software, try to flip the texture image horizontally with any image editing software. **UVs are parsed upside down for some reason, and any pull requests to fix this behaviour are welcome**.
<br>
<br>

## Credits
Checkerboard image: Blender
