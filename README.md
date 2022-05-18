## Dev
In order to compile the code, we need a different target:
First we need to add [cross](https://github.com/cross-rs/cross)
Best to follow this [guide](https://amritrathie.vercel.app/posts/2020/03/06/cross-compiling-rust-from-macos-to-raspberry-pi/) which worked for me
Afterwards, we can run commands:
```
cross test --target armv7-unknown-linux-musleabihf
```
