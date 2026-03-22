use plugins_rs::include_dir::include_dir;

fn main() {

    let mut prt = match plugins_rs::PluginSystem::builder()
        .add_source(plugins_rs::Source::Static(std::env::current_dir().unwrap().join("examples").join("javascript").join("plugins")))
        .add_source(plugins_rs::Source::Embed(include_dir!("examples/javascript/embed")))
        .run() {
            Ok(prt) => prt,
            Err(err) => panic!("{}", err),
        };

    loop {

        println!("{:?}", prt.send("namespace", r#"console.log("test")"#).unwrap());

        std::thread::sleep(std::time::Duration::from_millis(1000));

    }

}