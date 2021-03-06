# rmessenger
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)  

## A Rust Wrapper for the FaceBook Messenger Bot API
[Facebook's Messenger Platform](https://developers.facebook.com/docs/messenger-platform)  
crates.io: [rmessenger](https://crates.io/crates/rmessenger)

### About
you can:
- send text message
- send generic message
- send button message
- send file url
- send audio url

### Installation

#### Cargo.toml
```
rmessenger = "0.0.4"
```

### Usage

#### Send text message
https://developers.facebook.com/docs/messenger-platform/send-api-reference/text-message

```rust
extern crate rmessenger;
use rmessenger::bot;

fn main() {
    let bot = bot::Bot::new("<YOUR ACCESS TOKEN>", "<YOUR APP SECRET>");
    bot.send_text_message("<recipient_id>", "<message>");
}
````

#### Send generic message
https://developers.facebook.com/docs/messenger-platform/send-api-reference/generic-template  

```rust
extern crate rmessenger;
use rmessenger::bot;

fn main() {
    let bot = bot::Bot::new("<YOUR ACCESS TOKEN>", "<YOUR APP SECRET>");
    bot.send_generic_message("<recipient_id>>",
                             "[{'title': 'example',
                                'image_url': 'https://example.png'
                                }]");
}
````

> elements param is &str

### TODO
- send image
- send file
- send audio
- send video
