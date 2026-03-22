mod build_exts;

use std::env;
//use std::fs::create_dir_all;
use std::path::PathBuf;

fn main() {

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("RUNJS_SNAPSHOT.bin");

    let snapshot = deno_core::snapshot::create_snapshot(
        deno_core::snapshot::CreateSnapshotOptions {
            cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
            startup_snapshot: None,
            skip_op_registration: false,
            extensions: vec![
                #[cfg(feature = "media")]
                build_exts::media_js::init(),
                build_exts::core_js::init(),
                #[cfg(feature = "scrape")]
                build_exts::scrape_js::init(),
            ],
            with_runtime_cb: None,
            extension_transpiler: None,
        },
        None,
    )
    .unwrap();

    std::fs::write(snapshot_path, snapshot.output).unwrap();

    // Let cargo know that builds depend on these files:
    for path in snapshot.files_loaded_during_snapshot {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}
