# Auto Heckin' Texture Converter
Texture converter for DOOM Eternal.

Texture modding guides are available [here](https://wiki.eternalmods.com/books/eternal-texture-mods-a-comprehensive-guide).

## Usage
Drag and drop textures onto it, or pass them as arguments from the terminal.

## Compiling
NOTE: The static libs from the Oodle SDK are needed to build this project.

Place the lib for the target OS in the `lib` folder, with the indicated name:
```
lib/oo2core_win64.lib # Windows
lib/liboo2corelinux64.a # Linux
lib/liboo2coremac64.a # macOS
```

1. Install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).
2. Clone the repo using:
  ```
  git clone https://github.com/PowerBall253/AutoHeckinTextureConverter.git
  cd AutoHeckinTextureConverter
  ```
3. Compile the project using cargo:
  ```
  cargo build --release
  ```

The compiled binary will be located at `./target/release`.
