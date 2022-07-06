use log::{ info, error, debug, trace };
use std::collections::HashMap;



//#[derive(Debug)]
pub struct MWISInfo {
    vertex_list: HashMap::<usize,u64>,
    mwis_results: HashMap::<usize,u64>,
}



impl MWISInfo {

    pub fn new() -> Self {
        MWISInfo {
            vertex_list : HashMap::<usize,u64>::new(),
            mwis_results : HashMap::<usize,u64>::new(),
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
        let w1 = self.vertex_list.get(&1).unwrap().clone();
        if self.vertex_list.len() > 0 {
            self.mwis_results.insert(1,w1);
        }

        let num_vertex = self.vertex_list.len();
        for index in 2..num_vertex {
            debug!("index {}", index);
            
        }

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
    fn test_add() {
        init();
        let mut h = setup_basic();
        h.compute_mwis();

    }


 }
