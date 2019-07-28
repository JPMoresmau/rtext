use std::str::SplitWhitespace;
use std::collections::HashMap;

pub fn term_frequency(doc: Vec<String>) -> HashMap<String,f64> {
    let mut m=HashMap::new();
    for s in doc {
        let count = m.entry(s).or_insert(0.0);
        *count += 1.0;
    }
    m
}

pub fn term_frequency_bool(doc: Vec<String>) -> HashMap<String,f64> {
    let mut m=HashMap::new();
    for s in doc {
        m.insert(s,1.0);
    }
    m
}

pub fn term_frequency_len(doc: Vec<String>) -> HashMap<String,f64> {
    let s= doc.len() as f64;
    let mut m = term_frequency(doc);
    for (_,val) in m.iter_mut(){
        *val /= s;
    }
    m
}

pub fn term_frequency_log(doc: Vec<String>) -> HashMap<String,f64> {
    let mut m = term_frequency(doc);
    for (_,val) in m.iter_mut(){
        *val = (1.0 + *val).log10();
    }
    m
}

pub fn term_frequency_aug(doc: Vec<String>) -> HashMap<String,f64> {
    let mut m = term_frequency(doc);
    let max = m.values()
        .max_by(|f1,f2| f1.partial_cmp(f2).expect("NaN"))
        .expect("No terms!") * 2.0;
    for (_,val) in m.iter_mut(){
        *val = (*val / max) + 0.5;
    }
    m
}

pub fn tokenize(s: &str) -> Vec<String> {
    split_space(s).map(|t| clean_token(t)).filter(|t| t.len()>0).collect()
}

fn split_space(s: &str) -> SplitWhitespace {
    s.split_whitespace()
}

fn clean_token(t: &str) -> String {
    let mut v = vec!();
    let mut write = false;
    for c in t.chars() {
        if write {
           v.push(c);      
        } else {
            if c.is_alphabetic(){
                write = true;
                v.push(c);
            }
        }
    }
    while v.len()>0 && !v[v.len()-1].is_alphabetic() {
        v.pop();
    }
    v.iter().collect::<String>().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_token(){
        assert_eq!(String::from("jp"),clean_token("jp"));
        assert_eq!(String::from("jp"),clean_token(".jp"));
        assert_eq!(String::from("jp"),clean_token("jp."));
        assert_eq!(String::from("jp"),clean_token("jp1"));
        assert_eq!(String::from("jp"),clean_token("1jp"));
        assert_eq!(String::from("j-p"),clean_token("j-p"));
        assert_eq!(String::from("j-p"),clean_token("J-P"));
    }

    #[test]
    fn test_tokenize(){
        let v = vec!("the","lazy","fox","jumps","over","the","fence");
        assert_eq!(v,tokenize("The, lazy, fox  2  jumps over the fence!!"))
    }

    #[test]
    fn test_term_frequency(){
        let v = vec!("the","lazy","fox","jumps","over","the","fence")
            .iter().map(|s| String::from(*s)).collect();
        let mut m = HashMap::new();
        m.insert( String::from("the"), 2.0);
        m.insert( String::from("lazy"), 1.0);
        m.insert( String::from("fox"), 1.0);
        m.insert( String::from("jumps"), 1.0);
        m.insert( String::from("over"), 1.0);
        m.insert( String::from("fence"), 1.0);
        assert_eq!(m,term_frequency(v));
    }

    #[test]
    fn test_term_frequency_bool(){
        let v = vec!("the","lazy","fox","jumps","over","the","fence")
            .iter().map(|s| String::from(*s)).collect();
        let mut m = HashMap::new();
        m.insert( String::from("the"), 1.0);
        m.insert( String::from("lazy"), 1.0);
        m.insert( String::from("fox"), 1.0);
        m.insert( String::from("jumps"), 1.0);
        m.insert( String::from("over"), 1.0);
        m.insert( String::from("fence"), 1.0);
        assert_eq!(m,term_frequency_bool(v));
    }

    #[test]
    fn test_term_frequency_len(){
        let v = vec!("the","lazy","fox","jumps","over","the","fence")
            .iter().map(|s| String::from(*s)).collect();
        let mut m = HashMap::new();
        m.insert( String::from("the"), 2.0 / 7.0);
        m.insert( String::from("lazy"), 1.0 / 7.0);
        m.insert( String::from("fox"), 1.0 / 7.0);
        m.insert( String::from("jumps"), 1.0 / 7.0);
        m.insert( String::from("over"), 1.0 / 7.0);
        m.insert( String::from("fence"), 1.0 / 7.0);
        assert_eq!(m,term_frequency_len(v));
    }

    #[test]
    fn test_term_frequency_log(){
        let v = vec!("the","lazy","fox","jumps","over","the","fence")
            .iter().map(|s| String::from(*s)).collect();
        let mut m = HashMap::new();
        m.insert( String::from("the"), f64::log10(3.0));
        m.insert( String::from("lazy"), f64::log10(2.0));
        m.insert( String::from("fox"), f64::log10(2.0));
        m.insert( String::from("jumps"), f64::log10(2.0));
        m.insert( String::from("over"), f64::log10(2.0));
        m.insert( String::from("fence"), f64::log10(2.0));
        assert_eq!(m,term_frequency_log(v));
    }

    #[test]
    fn test_term_frequency_aug(){
        let v = vec!("the","lazy","fox","jumps","over","the","fence")
            .iter().map(|s| String::from(*s)).collect();
        let mut m = HashMap::new();
        m.insert( String::from("the"), 1.0);
        m.insert( String::from("lazy"), 0.75);
        m.insert( String::from("fox"), 0.75);
        m.insert( String::from("jumps"), 0.75);
        m.insert( String::from("over"), 0.75);
        m.insert( String::from("fence"), 0.75);
        assert_eq!(m,term_frequency_aug(v));
    }
}