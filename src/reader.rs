use std::path::PathBuf;
use crate::{ Result, Sources, SourceRes };




#[derive(Clone)]
pub struct Reader {
    basefile: String,
    pub sources : Sources,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            sources : Sources::new(),
            basefile: "plugin.json".to_string(),
        }
    }

    pub fn find(self, source: usize, entry: PathBuf, search: PathBuf) -> Result<Vec<u8>> {
        Ok(self.sources.find(source, entry, search)?)
    }

    pub fn load_plugins<F>(self, callback: F) -> Result<()>
    where 
        F: Fn(usize, SourceRes) + Send + Sync,
    {
        let sources = self.sources.0.read();

        for (i, source) in sources.iter().enumerate() {
            
            source.clone().read_source(true, |result| {

                if result.search.ends_with(self.basefile.clone()) {

                    callback(i, result);

                }

            })?;

        }

        Ok(())
    }

    pub fn load_base<F>(self, callback: F) -> Result<()>
    where 
        F: Fn(SourceRes) + Send + Sync,
    {
        let sources = self.sources.0.read();

        for (_, source) in sources.iter().enumerate() {
            
            source.clone().read_source(false, |result| {

                callback(result);

            })?;

        }

        Ok(())
    }

}


















