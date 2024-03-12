
Online Khabar TUI &middot; 📰
=====
A terminal User Interface to read latest news from [**online khabar**]("https://english.onlinekhabar.com/").

```bash
curl https://english.onlinekhabar.com/wp-json/
curl https://english.onlinekhabar.com/wp-json/wp/v2/posts?per_page=10
# check other available routes
curl https://english.onlinekhabar.com/wp-json/wp/v2
```
**Important!** These APIs are private, so be careful and only use them for learning. We're not trying to sneak into Online Khabar's data without permission. This is all for educational purposes.

## Demo

![Demo](demo.gif)
## Running The Application
-   **Rust and Cargo:** Make sure to have latest version of rustc and cargo installed in your computer: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
```bash
git clone https://github.com/BipulLamsal/onlinekhabar-tui
cd onlinekhabar-tui
```

```bash
# directly run via
cargo run
# or build
cargo build --release
./target/release/onlinekhabar-tui 
```




