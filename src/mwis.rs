use log::{ info, error, debug, trace };
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::cmp::max;
use std::collections::HashSet;



//#[derive(Debug)]
pub struct MWISInfo {
    vertex_list: HashMap::<usize,u64>,
    mwis_results: BTreeMap::<usize,u64>,
    mwis_vertex : HashSet::<usize>,
}



impl MWISInfo {

    pub fn new() -> Self {
        MWISInfo {
            vertex_list : HashMap::<usize,u64>::new(),
            mwis_results : BTreeMap::<usize,u64>::new(),
            mwis_vertex : HashSet::<usize>::new()
        }
        
    }

    pub fn add_vertex(&mut self,weight:u64) -> usize {
        // vertexes are number 1-N
        let id = self.vertex_list.len() + 1;
        self.vertex_list.insert(id,weight);
        debug!("Added Simple Vertex {}  w={}",id,weight);
        id
    }

    pub fn compute_mwis(&mut self) {

        self.mwis_results.insert(0,0);
        if self.vertex_list.len() > 0 {
            let w1 = self.vertex_list.get(&1).unwrap().clone();
            self.mwis_results.insert(1,w1);
        }
        if self.vertex_list.len() > 1 {
            let w1 = self.vertex_list.get(&1).unwrap().clone();
            let w2 = self.vertex_list.get(&2).unwrap().clone();
            self.mwis_results.insert(2,max(w1,w2));
        }


        let max_vertex_id = self.vertex_list.len();
        for index in 3..=max_vertex_id {
            let result1 = self.mwis_results.get(&(index-2)).unwrap().clone() +self.vertex_list.get(&index).unwrap().clone();
            let result2 = self.mwis_results.get(&(index-1)).unwrap().clone();
            self.mwis_results.insert(index,max(result1,result2));
            debug!("index {}", index);
        }

    }

    pub fn identify_mwis_vertex(&mut self) {

        let mut index = self.vertex_list.len();
        
        while index > 0 {
            trace!("Identifying {}", index);
            let current_mwis = self.mwis_results.get(&index).unwrap().clone();
            let result2 = self.mwis_results.get(&(index-1)).unwrap().clone();
            if current_mwis == result2 {
                index = index.saturating_sub(1);
            }
            else {
                // add the current vertex to the set
                self.mwis_vertex.insert(index);
                index = index.saturating_sub(2);
            }

        }


    }


    pub fn process(&mut self) {
        self.compute_mwis();
        trace!("Final weight {:#?}",self.mwis_results);
        self.identify_mwis_vertex();
        trace!("Vertexes {:#?}",self.mwis_vertex);

    }

    pub fn check_verts_in_mwis(self,check_vertex: Vec<usize>) -> String {

        let mut result = "".to_string();

        for v in check_vertex {
            if self.mwis_vertex.contains(&v) {
                result += "1";
            }
            else {
                result += "0";
            }
        }
        result

    }

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    fn init () {
        let _ = env_logger::builder().is_test(true).try_init();
        info!("Init {}",module_path!())
    }

    fn setup_basic() -> MWISInfo {

        let mut h = MWISInfo::new();
        assert_eq!(h.add_vertex(10),1);
        assert_eq!(h.add_vertex(20),2);
        assert_eq!(h.add_vertex(30),3);
        assert_eq!(h.add_vertex(5),4);
        h
    }

    #[test]
    fn test_basic() {
        init();
        let mut h = setup_basic();
        h.process()

    }

    #[test]
    fn test_compare() {
        init();
        let mut h = setup_basic();
        h.process();
        assert_eq!(h.check_verts_in_mwis(vec![1,3]),"11".to_string());

     }

 }
