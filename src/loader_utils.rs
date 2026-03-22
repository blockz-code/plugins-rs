use crate::{ Error, Result };



#[derive(serde::Deserialize)]
struct JsrPackageMeta {
    scope: String,
    name: String,
    latest: String,
}


#[derive(serde::Deserialize)]
struct PackageMeta {
    name: String,
    version: String,
    #[serde(rename(deserialize = "exports", deserialize = "main"))]
    entry: String,
}

fn jsr_package_meta(pkg: &str) -> Result<JsrPackageMeta> {
    Ok(reqwest::blocking::get(format!("https://jsr.io/{}/meta.json", pkg))?.json::<JsrPackageMeta>()?)
}

fn jsr_package_main(pkg: &str, version: &str) -> Result<PackageMeta> {
    let res = match reqwest::blocking::get(format!("https://jsr.io/{}/{}/deno.json", pkg, version)) {
        Ok(res) => Ok(res),
        Err(_) => match reqwest::blocking::get(format!("https://jsr.io/{}/{}/package.json", pkg, version)) {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::Unknown(format!("No deno.json or package.json found in this package"))),
        },
    }?;

    Ok(res.json::<PackageMeta>()?)
}

fn jsr_package_file(pkg: &str, version: &str, filename: &str) -> Result<String> {
    Ok(reqwest::blocking::get(format!("https://jsr.io/{}/{}/{}", pkg, version, filename))?.text()?)
}


// https://registry.npmjs.org/react-chuck

// https://registry.npmjs.org/react-chuck/1.1.0