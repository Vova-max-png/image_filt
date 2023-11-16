# Image filt

## What's Image filt?

### Image filt - library to apply filters to an image. Make the image lighter or darker. Apply blur, smooth etc. All of that is here.🪛

## Usage

### Before starting work, you need to understand that Image filt is in dev and there can be a lot of issues.

### Make the picture lighter or darker:
```rust
// If you wanna to make the picture lighter or darker, use .light_filter(offset)
Filter::new("Path to the file to modify")
    .open()
    .unwrap()
    .light_filter(1.3) // Lighter on 30%
    .unwrap()
    .save("Path to the file to save".to_string());
```

### Apply blur to the image:
```Rust
// As you see we use .box_blur(offset) to do that
Filter::new("Path to the file to modify")
    .open()
    .unwrap()
    .box_blur(5) // 5 is an example
    .unwrap()
    .save("Path to the file to save".to_string());
```