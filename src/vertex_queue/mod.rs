use core::fmt;

use geo_types::Polygon;

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub(crate) enum IndexType{
    PointerIndex(usize),
    RealIndex(usize),
}

impl fmt::Display for IndexType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            IndexType::PointerIndex(x) => write!(f, "Pointer index: {x}"),
            IndexType::RealIndex(x) => write!(f, "Real index: {x}"),
        }
    }
}

impl IndexType{
    pub(crate) fn get_index(&self) -> usize{
        if let IndexType::PointerIndex(res) = self{
            return *res;
        }
        panic!("Expected IndexType::PointerIndex");
    }

    pub(crate) fn get_real_index(&self) -> usize{
        if let IndexType::RealIndex(res) = self{
            return *res;
        }
        panic!("Expected IndexType::RealIndex");
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Node{
    pub(crate) index: IndexType,
    pub(crate) left: IndexType,
    pub(crate) right: IndexType,
    pub(crate) done: bool,
}

#[allow(dead_code)]
impl Node{
    fn new(index: usize, left: usize, right: usize) -> Self{
        Self{index: IndexType::RealIndex(index), left: IndexType::PointerIndex(left), right: IndexType::PointerIndex(right), done: false,}
    }

    pub(crate) fn lv(&self) -> IndexType{
        self.left
    }

    pub(crate) fn rv(&self) -> IndexType{
        self.right
    }
}

#[derive(Clone, Debug)]
pub(crate) struct VertexQueue{
    pub(crate) content: Vec<Node>,
    pub(crate) start_vertex: Vec<usize>,
}

#[allow(dead_code)]
impl VertexQueue{
    pub(crate) fn new() -> Self{
        Self { content: Vec::new(), start_vertex: Vec::new(), }
    }

    pub(crate) fn initialize_from_polygon(&mut self, p: &Polygon){
        self.initialize_from_polygon_vector(&vec![p.clone()])
    }

    pub(crate) fn initialize_from_polygon_vector(&mut self, pv: &Vec<Polygon>){
        for p in pv{
            let offset = self.content.len();
            let len = p.exterior().0.len() - 1;
            self.start_vertex.push(offset);
            for i in 0..len{
                let new_node = Node::new(i+offset, (i+len-1)%len+offset, (i+1)%len+offset);
                self.content.push(new_node);
            }
            for i in 0..p.interiors().len(){
                let offset = self.content.len();
                let len = p.interiors()[i].0.len()-1;
                self.start_vertex.push(offset);
                for j in 0..len{
                    let new_node = Node::new(j+offset, (j+len-1)%len+offset, (j+1)%len+offset);
                    self.content.push(new_node);
                }
            }
        }
    }

    pub(crate) fn get_real_index(&self, cv: IndexType) -> usize{
        if let IndexType::PointerIndex(cv) = cv{
            return self.content[cv].index.get_real_index();
        }
        panic!("Expected parameter \"cv\" as IndexType::RealIndex")
    }

    pub(crate) fn lv(&self, cv: IndexType) -> IndexType{
        if let IndexType::PointerIndex(cv) = cv{
            return self.content[cv].left;
        }
        panic!("Expected parameter \"cv\" as IndexType::PointerIndex");
    }

    pub(crate) fn rv(&self, cv: IndexType) -> IndexType{
        if let IndexType::PointerIndex(cv) = cv{
            return self.content[cv].right;
        }
        panic!("Expected parameter \"cv\" as IndexType::PointerIndex");
    }

    pub(crate) fn llv(&self, cv: IndexType) -> IndexType{
        let cv = self.lv(cv);
        self.lv(cv)
    }

    pub(crate) fn rrv(&self, cv: IndexType) -> IndexType{
        let cv = self.rv(cv);
        self.rv(cv)
    }

    pub(crate) fn remove(&mut self, cv: IndexType) -> IndexType{
        let tl = self.lv(cv);
        let tr = self.rv(cv);
        self.content[tl.get_index()].right = tr;
        self.content[tr.get_index()].left = tl;
        self.content[cv.get_index()].done = true;
        tr
    }

    pub(crate) fn remove_and_set(&mut self, cv: IndexType, nv: IndexType) -> IndexType{
        let cv = self.remove(cv);
        if let IndexType::RealIndex(_) = nv{
            self.content[cv.get_index()].index = nv;
        }
        else {panic!("Expected parameter \"nv\" as IndexType::RealIndex");}
        cv
    }

    pub(crate) fn split_and_set(&mut self, cv: IndexType, sv: IndexType, nv1: IndexType, nv2: IndexType) -> (IndexType, IndexType){
        let new_node = Node::new(0, sv.get_index(), self.rv(cv).get_index());
        let new_index = IndexType::PointerIndex(self.content.len());
        self.content.push(new_node);
        if let IndexType::RealIndex(_) = nv1{
            self.content[cv.get_index()].index = nv1;
        }
        else {panic!("Expected parameter \"nv1\" as IndexType::RealIndex");}
        if let IndexType::RealIndex(_) = nv2{
            self.content[new_index.get_index()].index = nv2;
        }
        else {panic!("Expected parameter \"nv2\" as IndexType::RealIndex");}
        let svx = self.rv(sv); // right of sv (split vertex)
        let cvx = self.rv(cv); // right of cv (current (anchor) vertex)
        self.content[cvx.get_index()].left = new_index;
        self.content[sv.get_index()].right = new_index;
        self.content[cv.get_index()].right = svx;
        self.content[svx.get_index()].left = cv;
        self.start_vertex.push(cv.get_index());
        self.start_vertex.push(new_index.get_index());
        (cv, new_index)
    }

    pub(crate) fn cleanup(&mut self){
        let mut sv_idx = 0;
        let mut visit = vec![false; self.content.len()];
        while sv_idx < self.start_vertex.len(){
            let mut cur = self.start_vertex[sv_idx];
            while self.content[cur].done == true && visit[cur] == false{
                visit[cur] = true;
                cur = self.content[cur].right.get_index();
            }
            if visit[cur] == true || self.content[cur].left.get_index() == self.content[cur].right.get_index(){
                self.start_vertex.swap_remove(sv_idx);
                continue;
            }
            self.start_vertex[sv_idx] = cur;
            visit[cur] = true;
            cur = self.content[cur].right.get_index();
            while cur != self.start_vertex[sv_idx] {
                if visit[cur] {
                    panic!("Something Worng in cleanup phase: cur {} from {}, sv {:?}", cur, sv_idx, self.start_vertex);
                }
                visit[cur] = true;
                cur = self.content[cur].right.get_index();
            }
            sv_idx += 1;
        }
    }

    pub(crate) fn iter<'a>(&'a self) -> Iter<'a> {
        Iter { item: self, sv_idx: 0, idx: usize::MAX, }
    }
}

impl fmt::Display for VertexQueue{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[").unwrap();
        for (e, ee, eee) in self.iter(){
            let ee = ee.get_index();
            write!(f, "({}, {}, {}), ", e, ee, eee).unwrap();
        }
        write!(f, "]")
    }
}

pub(crate) struct Iter<'a> {
    item: &'a VertexQueue,
    sv_idx: usize,
    idx: usize,
}

impl<'a> Iterator for Iter<'a>{
    type Item = (usize, IndexType, usize);

    fn next(&mut self) -> Option<Self::Item>{
        if self.idx == usize::MAX{
            if self.sv_idx >= self.item.start_vertex.len() {return None;}
            self.idx = self.item.start_vertex[self.sv_idx];
        }
        let ret = match self.item.content[self.idx].index{
            IndexType::RealIndex(rv) => rv,
            _ => panic!("Expected IndexType::RealIndex"),
        };
        let ret = (self.sv_idx, IndexType::PointerIndex(self.idx), ret);
        self.idx = self.item.content[self.idx].right.get_index();
        if self.item.start_vertex[self.sv_idx] == self.idx{
            self.sv_idx += 1;
            self.idx = usize::MAX;
        }
        Some(ret)
    }
}