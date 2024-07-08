# 🍞 gLUTen

gLUTen is a LUT library written in Rust, for use in color correction and
color grading applications. It has a Python interface that allows you to use
gLUTen in scripts for tools like Maya, Blender, and other digital content
creation (DCC) software.

> 🚧 gLUTen is a work in progress. Features are still due for the first Cargo
> release.

## ✨ Features

### 🔨 Generate LUTs

Use gLUTen to generate LUTs from a reference image. Here's the workflow:

1. Apply your color correction/grading on [original.png](original.png).
2. Save the edited image as a PNG; make sure the sizes match with the original.
3. Import the image using `Image::load_png`.
4. Generate the LUT using `Lut::build`.
5. Save the LUT as a file using `Lut::save`.

Check out example code in Rust and Python below. 👇

## 🦀 Usage in Rust

```rust
use gluten::{Image, Lut, LutFormat};

fn main() {
    let image = Image::load_png("edited.png").unwrap();
    let lut = Lut::build(&image, None).unwrap();
    lut.save("lut.cube", LutFormat::CubeLut, 6, true, false)
        .unwrap();
}
```

## 🐍 Usage in Python

```python
import gluten

image = gluten.Image.load_png("edited.png")
lut = gluten.Lut.build(image, title="Test")
lut.save("lut.cube", gluten.LutFormat.CubeLut, 6, True, False)
```

## 📄 Supported file formats

### 🖼️ Images

- PNG

### 🎨 LUTs

- Cube LUT

## 👷 Development

### 🦀 For Rust

```shell
cargo build
```

### 🐍 For Python

First, set up the Python virtual environment:

```shell
python -m venv .env
source .env/bin/activate
pip install maturin
```

Then use `maturin` to develop:

```shell
maturin develop
```

That's it! You can now use gLUTen.

```
$ python
>>> import gluten
>>> dir(gluten)
['Image', 'Lut', 'LutFormat', '__all__', '__builtins__', '__cached__', '__doc__', '__file__', '__loader__', '__name__', '__package__', '__path__', '__spec__', 'gluten']
```
