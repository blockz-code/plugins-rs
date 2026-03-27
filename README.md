# PluginSystem

#### PluginSystem build with [deno-core](https://github.com/denoland/deno_core)

---

**Cargo.toml**
```toml
plugin-rs = { git = "https://github.com/blockz-dev/plugins-rs" }
```

---

**Features**
+ Embeded Plugins
+ Simple Standalone Plugins
+ Archived Plugins with `.7z`, `.zip`, `.tar`, `.tar.gz`, `.tar.xz`


**Not Supported yet**
+ npm, jsr packages (only local)

---

**Example**

```rust
use plugins_rs::bind_dir;
use plugins_rs::{ Source, PluginSystem };

fn main() {

    let mut plugins = match PluginSystem::builder()
        .add_source(Source::from_static(std::env::current_dir().unwrap().join("javascript").join("plugins")))
        .add_embed(Source::from_embed(bind_dir!("javascript/embed")))
        .run() {
            Ok(prt) => prt,
            Err(err) => panic!("{}", err),
        };

    loop {
        println!("exec() > {:?}", plugins.execute("namespace", "base", "exec()").unwrap());
        println!("do_action.audio_dir() > {:?}", plugins.execute("namespace", "base", "do_action.audio_dir()").unwrap());
        println!("demo.title > {:?}", plugins.execute("namespace", "base", "demo.title").unwrap());
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

}
```