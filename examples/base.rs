use plugins_rs::bind_dir;
use plugins_rs::{ Source, PluginSystem };

fn main() {

    let mut prt = match PluginSystem::builder()
        .add_source(Source::from_static(std::env::current_dir().unwrap().join("examples").join("javascript").join("plugins")))
        .add_embed(Source::from_embed(bind_dir!("examples/javascript/embed")))
        .run(None) {
            Ok(prt) => prt,
            Err(err) => panic!("{}", err),
        };

    loop {
        println!("exec() > {:?}", prt.execute("namespace", "base", "exec()").unwrap());
        println!("do_action.audio_dir() > {:?}", prt.execute("namespace", "base", "do_action.audio_dir()").unwrap());
        println!("demo.title > {:?}", prt.execute("namespace", "base", "demo.title").unwrap());
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

}