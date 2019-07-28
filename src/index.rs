use std::collections::HashMap;

#[derive(Debug)]
pub struct Index {
    last_id: u128,
    last_op: u128,
    docs: HashMap<u128,HashMap<String,f64>>,
    terms: HashMap<String,Vec<u128>>,
    idfs: HashMap<String, (u128,f64)>,
    tfidfs: HashMap<String, Vec<(u128,f64)>>,
    doc_tfidfs: HashMap<u128, Vec<(String, f64)>>,
}

impl Index {

    pub fn new() -> Index {
        Index {
            last_id:0,
            last_op:0,
            docs: HashMap::new(),
            terms: HashMap::new(),
            idfs: HashMap::new(),
            tfidfs: HashMap::new(),
            doc_tfidfs: HashMap::new(),
        }
    }

    pub fn add_doc(&mut self, freq: HashMap<String,f64>) -> u128 {
        let id = self.last_id + 1;
        self.last_id = id;
        self.last_op += 1;
        let l = (self.docs.len()+1) as f64;
        self.docs.insert(id, freq);
        let mut dtfidf=vec!();
        let tfidfs = &mut self.tfidfs;
        for (s,_) in self.docs.get(&id).unwrap() {
            let v = self.terms.entry(s.clone()).or_insert(vec!());
            v.push(id);
            let idf =  (l / v.len() as f64).log10();
            self.idfs.insert(s.clone(), (self.last_op,idf));
            Index::update_tfidf(&s, idf, v, &self.docs, tfidfs);
            let tfidf = tfidfs.get(s).unwrap().iter().filter(|(d,_)| *d == id).next().unwrap().1;
            dtfidf.push((s.clone(),tfidf))
        }
        dtfidf.sort_by(|(_,s1),(_,s2)| s2.partial_cmp(s1).expect("NaN"));
        self.doc_tfidfs.insert(id, dtfidf);
        id
    }

    pub fn update_index(&mut self) {
        let op = self.last_op;
        let l = self.docs.len() as f64;
        let tfidfs = &mut self.tfidfs;

        for (s,v) in self.idfs.iter_mut(){
            if (*v).0 < op {
                let docids = self.terms.get(s).expect("no term!");
                let tl = docids.len();
                let idf =  (l / tl as f64).log10();
                *v = (op,idf);
                Index::update_tfidf(s,idf,docids,&self.docs, tfidfs);
            }
        } 
        
    }

    fn update_tfidf(key: &str, idf:f64, docids: &Vec<u128>
        , docs: &HashMap<u128,HashMap<String,f64>>
        , tfidfs: &mut HashMap<String, Vec<(u128,f64)>> ) {
        let mut v = vec!();
        for docid in docids{
            if let Some(f) = docs.get(&docid) {
                if let Some(tf) = f.get(key) {
                    v.push((*docid, idf * tf));
                }
            }
        }
        v.sort_by(|(_,a),(_,b)| b.partial_cmp(a).expect("NaN"));
        tfidfs.insert(String::from(key), v);
    }

    pub fn search(&self, terms: &Vec<&str>) -> Vec<(u128,f64)> {
        let mut m = HashMap::new();
        for t in terms {
            if let Some(tfidfs) = self.tfidfs.get(*t) {
                for tfidf in tfidfs {
                    let v = m.entry(tfidf.0).or_insert(0.0);
                    *v += tfidf.1;
                }
            }

        }
        let mut v = vec!();
        for (k,val) in m.iter(){
            v.push((*k,*val));
        }
        v.sort_by(|(_,s1),(_,s2)| s2.partial_cmp(s1).expect("NaN"));
        v
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wikipedia(){
        let mut m1 = HashMap::new();
        m1.insert( String::from("this"), 1.0 / 5.0);
        m1.insert( String::from("is"), 1.0 / 5.0);
        m1.insert( String::from("a"), 2.0 / 5.0);
        m1.insert( String::from("sample"), 1.0 / 5.0);
        let mut m2 = HashMap::new();
        m2.insert( String::from("this"), 1.0 / 7.0);
        m2.insert( String::from("is"), 1.0 / 7.0);
        m2.insert( String::from("another"), 2.0 / 7.0);
        m2.insert( String::from("example"), 3.0 / 7.0);
        let mut idx= Index::new();
        idx.add_doc(m1);
        idx.add_doc(m2);
        idx.update_index();

        assert_eq!(Some(&(2,0.0)),idx.idfs.get("this"));
        assert!(idx.tfidfs.get("this").unwrap().iter().all(|(_,v)| *v == 0.0));

        approx_eq(0.301,idx.idfs.get("example").unwrap().1);
        let v = idx.tfidfs.get("example").unwrap();
        assert_eq!(1,v.len());
        assert_eq!(2,v[0].0);
        approx_eq(0.129,v[0].1);
      
        let sv=idx.search(&vec!("example"));
        assert_eq!(1,sv.len());
        assert_eq!(2,sv[0].0);
        approx_eq(0.129,sv[0].1);


    }

    // approximate equality on floats.
    // see also https://crates.io/crates/float-cmp
    fn approx_eq(f1: f64, f2: f64) {
        assert!((f2-f1).abs() < 0.0001, "{} != {}", f1, f2)
    }
}