# 🍞 gLUTen

gLUTen is a LUT library written in Rust, for use in color correction and
color grading applications. It has a Python interface that allows you to use
gLUTen in scripts for tools like Maya, Blender, and other digital content
creation (DCC) software.

## ✨ Features

### 🔨 Generate LUTs

Use gLUTen to generate LUTs from a reference image. Here's the workflow:

1. Apply your color correction/grading on <original.png>.
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
