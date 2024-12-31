use std::path::{Path, PathBuf};
use std::fs::read_dir;

struct Indexer {
    search_paths: Vec<PathBuf>,
}

impl Indexer {
    fn new(paths: Vec<PathBuf>) -> Indexer { Indexer{search_paths: paths} }
    fn from_str(paths: Vec<String>) -> Indexer {
        Indexer::new(paths.iter().map(|p| Path::new(p).to_path_buf()).collect())
    }
    fn from_pathstr(pathstr: String) -> Indexer {
        let paths: Vec<PathBuf> = pathstr.split(";")
            .map(|p| Path::new(p).to_path_buf())
            .collect();
        Indexer::new(paths)
    }
    fn get_all(&self) -> Vec<String> {
        &self.search_paths.iter()
            .flat_map(|path: &PathBuf| read_dir(path).unwrap())
            .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
            .collect()
    }
    fn search(&self, query: &str) -> Vec<String> {
        self.get_all().iter().filter(|path| {path.contains(query)}).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::indexer::Indexer;

    #[test]
    fn test_pathbuf_build() {
        let idxer: Indexer = Indexer::new(vec![Path::new("./src").to_path_buf()]);

        let files = idxer.get_all();
        assert!(files.contains(&"main.rs".to_string()))
    }
    fn test_str_build() {
        let idxer: Indexer = Indexer::from_str(vec!["./src".to_string()]);

        let files = idxer.get_all();
        assert!(files.contains(&"main.rs".to_string()))
    }
}