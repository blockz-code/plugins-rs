# PluginSystem

<center class="display: flex; align-items: center; justify-content: center; width: 100%; gap: 5px;">
<h3>PluginSystem build with <a href="https://github.com/denoland/deno_core">[deno-core]</a></h3>
</center>

<center class="display: flex; align-items: center; justify-content: center; width: 100%; gap: 5px;">
    <span style="display: inline-block;">
        <a href="https://crates.io/crates/plugins-rs">
            <img src="https://img.shields.io/crates/v/plugins-rs?style=flat-square">
        </a>
    </span>
</center>

---

**Cargo.toml**
```toml
plugin-rs = { version = "0.1.2" }
```

---

**Features**
+ Embeded Plugins
+ Simple Standalone Plugins
+ Archived Plugins with `.7z`, `.zip`, `.tar`, `.tar.gz`, `.tar.xz`


**Not Supported yet**
+ packages like `npm`, `jsr`, `git` *(comming soon)*

---

**main.rs**
```rust
use plugins_rs::bind_dir;
use plugins_rs::{ Source, PluginSystem };

fn main() {

    let mut plugins = match PluginSystem::builder()
        .add_source(Source::from_static(std::env::current_dir().unwrap().join("javascript").join("plugins")))
        .add_embed(Source::from_embed(bind_dir!("javascript/embed")))
        .run(None) {
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

#### Static Plugins

**`javascript/static/{myplugin}/plugin.json`**
```json
{
    "name": "Test Plugin",
    "description": "Test Plugin",
    "identifier": "com.test.plugin",
    "version": "1.0.0",
    "entry": "index.ts"
}
```


**`javascript/static/{myplugin}/index.ts`**
```ts

// your Plugin Logic

globalThis.Plugins.registerPlugin("base", {
    name: "Base Test Plugin",
    exec: function () {
        return Date.now();
    },
    demo: CustomApi,
    do_action
});
```

#### Embeded Plugins
> to register plugins or custom api
> entry.ts as entry required

**example 1: `javascript/embed/{myplugin}/entry.ts`**
```ts

// your Plugin Logic

globalThis.Plugins.registerPlugin("base", {
    name: "Base Test Plugin",
    exec: function () {
        return Date.now();
    },
    demo: CustomApi,
    do_action
});
```

**example 2: `javascript/embed/{myplugin}/entry.ts`**
```ts
declare global {
    var  CustomApi: {
        title: string;
    }
}

globalThis.CustomApi = {
    title : "DemoProperty"
};
```