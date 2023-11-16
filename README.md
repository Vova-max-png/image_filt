# Image filt

## What's Image filt?

### Image filt - library to apply filters to an image. Make the image lighter or darker. Apply blur, smooth etc. All of that is here.🪛

## Usage

### Before starting work, you need to understand that Image filt is in dev and there can be a lot of issues.

### Make the picture lighter or darker:
```rust
// If you wanna to make the picture darker, use FilterType::Darker instead
// of FilterType::Lighter
Filter::new("Path to the original file", FilterType::Lighter)
        .open()
        .filter()
        .save("Path where save the modified image".to_string());
```

### Apply blur to the image:
```Rust
// As you see we use FilterType::Blur there to apply blur.
Filter::new("Path to the original file", FilterType::Blur)
        .open()
        .filter()
        .save("Path where save the modified image".to_string());
```