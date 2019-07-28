
use crate::doc::{term_frequency_aug,tokenize};
use std::io;
use std::fs::{self, File};
use crate::index::Index;
use std::path::{Path};
use std::collections::HashMap;
use std::io::Read;
use path_slash::PathExt;

#[derive(Debug)]
pub struct IndexedFolder {
    index: Index,
    doc_files: HashMap<u128,String>,
}

impl IndexedFolder {
    pub fn index(path: &str) -> io::Result<IndexedFolder> {
        let mut i = Index::new();
        let mut files = HashMap::new();
        IndexedFolder::index_dir(&Path::new(path),&mut i,&mut files)?;
        i.update_index();
        Ok(IndexedFolder {
            index: i,
            doc_files: files,
        })
    }

    fn index_dir(dir: &Path, idx: &mut Index, files: &mut HashMap<u128,String>) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    IndexedFolder::index_dir(&path, idx, files)?;
                } else {
                    let mut file = File::open(path.clone())?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    let ts = tokenize(&contents);
                    let f = term_frequency_aug(ts);
                    let i = idx.add_doc(f);
                    files.insert(i, path.to_slash().unwrap());
                }
                
            }
        }
        Ok(())
    }

    pub fn search(&self, terms: &Vec<&str>) -> Vec<(String,f64)> {
        let vids = self.index.search(terms);
        vids.iter().map(|(id,s)| (self.doc_files.get(id).unwrap().clone(),*s)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_test_onegroup(){
        let rif = IndexedFolder::index("data/20newsgroups/20news-bydate-train/alt.atheism");
        assert!(rif.is_ok(),format!("{:?}",rif));
        let sv = rif.unwrap().search(&vec!("darwin"));
        assert_eq!(4,sv.len());
        println!("{:?}",sv);
        let onedoc=sv.iter().filter(|(p,_)| *p == String::from("data/20newsgroups/20news-bydate-train/alt.atheism/49960")).next();
        assert!(onedoc.is_some());
    }

}