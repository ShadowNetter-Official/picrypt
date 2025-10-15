# picrypt

A CLI tool to convert text into encrypted image and vice versa written in **Rust** 🦀

---

![Overview](screenshots/overview.gif)

Example:

<img src="screenshots/example.png" alt="example" width="300" height="300"/>


---

## Setup Guide

To install picrypt on your system, follow these steps:

### Use installation script

```bash
wget https://raw.githubusercontent.com/ShadowNetter-Official/picrypt/main/install.sh && sh install.sh
```

### Or build and install manually

### 1. Clone This Repo

```bash
git clone https://github.com/ShadowNetter-Official/picrypt
cd picrypt
```
### 2. Build

```bash
cargo build --release
chmod +x target/release/picrypt
```

### 3. Install

```bash
cp target/release/picrypt ~/.cargo/bin/
```

---

## Usage

![Usage](screenshots/help.png)

---

## Removing

You can remove picrypt with:

```bash
rm ~/.cargo/bin/picrypt
```

---

### Done :)

I hope you like it

### MIT License

Copyright (c) 2025 ShadowNetter-Official

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
