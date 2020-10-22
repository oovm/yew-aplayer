APlayer for Yew
===============

- The online preview: https://galaster.github.io/yew-aplayer

## How to use

1. It's only do the wasm bind, so load cdn first

```html
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/aplayer/1.10.1/APlayer.min.css">
```

2. Easily used by `<APlayer/>`

```rust
use yew_aplayer::{APlayer, APlayerAudio};
let songs = vec![
    APlayerAudio::new(
        "前前前世",
        "RADWIMPS",
        "https://cn-south-17-aplayer-46154810.oss.dogecdn.com/yourname.mp3",
        "https://cn-south-17-aplayer-46154810.oss.dogecdn.com/yourname.jpg",
        Some("https://cn-south-17-aplayer-46154810.oss.dogecdn.com/yourname.lrc"),
    ),
    APlayerAudio::new(
        "回レ！雪月花",
        "小倉唯",
        "https://cn-east-17-aplayer-35525609.oss.dogecdn.com/snowmoonflowers.mp3",
        "https://cn-east-17-aplayer-35525609.oss.dogecdn.com/snowmoonflowers.jpg",
        Some("https://cn-east-17-aplayer-35525609.oss.dogecdn.com/snowmoonflowers.lrc"),
    ),
];

html! {
    <APlayer audios=songs/>
}
```

## Todo

- [ ] Automatically import css cdn when the first component is loaded
