pub const OLD_THRESHOLD:u8 = 1;
#[derive(Clone)]
pub enum HeapHandle {
    CopyHeap(usize),
    OldHeap(usize),
}

#[derive(Clone, PartialEq)]
pub enum Color {
    White = 0x01,
    Gray = 0x02,
    Black = 0x03,
}
use std::sync::Mutex;

#[derive(Clone)]
pub struct GCBlock {
    pub v: super::super::executer::Value,
    pub ref_to: Option<Vec<HeapHandle>>,

    pub lived_copy: u8,
}
impl GCBlock{
    pub fn new(value:super::super::executer::Value)->Self{
        GCBlock{
            v:value,
            ref_to:None,
            lived_copy:0
        }
    }
}

pub struct CopyHeap{
    pub heap:Vec<(GCBlock,HeapHandle)>,
}
impl CopyHeap{
    pub fn new()->Self{
        Self{heap:vec!()}
    }
    pub fn push(&mut self,v:super::super::executer::Value)->*mut HeapHandle{
        let culen = self.heap.len();
        let blc = GCBlock::new(v);
        let mut hh = HeapHandle::CopyHeap(culen);
        self.heap.push((blc,hh));
        &mut self.heap.last_mut().unwrap().1
    }
    pub fn set(&mut self,handle:&HeapHandle,value:super::super::executer::Value){
        unsafe{(*self.index_by_handle(handle)).v = value}
    }
    pub fn get(&mut self,handle:&HeapHandle)->super::super::executer::Value{
        unsafe{(*self.index_by_handle(handle)).v.clone()}
    }
    pub fn index_by_handle(&mut self,handle:&HeapHandle)->*mut GCBlock{
        match handle{
            HeapHandle::CopyHeap(i) => &mut self.heap[*i].0,
            // global old heap index
            _ => unimplemented!()
        }
    }
    pub fn get_refs(&mut self,fix:&mut Vec<(GCBlock, HeapHandle)>,handle:&mut HeapHandle){
        if let HeapHandle::CopyHeap(hh) = handle {
            let vv = self.index_by_handle(handle);
            if let Some(vs) = unsafe{&mut (*vv).ref_to} {
                for vv in vs{
                    self.get_refs(fix,vv);
                }
            }
            *handle = HeapHandle::CopyHeap(fix.len());
            let mut p = unsafe{(*vv).clone()};
            p.lived_copy += 1;
            if p.lived_copy > OLD_THRESHOLD{
                // push to global managed old heap
                unimplemented!();
            }else{
                fix.push((p,handle.clone()));
            }
        }
    }
    pub fn clean(&mut self,handles:&mut Vec<HeapHandle>){
        let mut nt = vec!();
        for h in handles{
            self.get_refs(&mut nt, h);
        }
        self.heap = nt;
    }
    pub fn new_ref(&mut self,refer:&mut HeapHandle,refee:&mut HeapHandle){
        match refer{
            HeapHandle::CopyHeap(i) => {
                match refee{
                    HeapHandle::CopyHeap(j) => {
                        match &mut self.heap[*i].0.ref_to{
                            None => self.heap[*i].0.ref_to = Some(vec!(refee.clone())),
                            Some(v) => v.push(refee.clone()),
                        }
                    },
                    // global old heap index
                    _ => unimplemented!()
                }
            }
            // global old heap index
            _ => unimplemented!()
        }
    }
}
// use std::alloc::*;
// use Color::*;
// use HeapHandle::*;
// use Generation::*;

// impl GCBlock {
//     // gray block
//     pub fn new(size: u64) -> GCBlock {
//         GCBlock {
//             ptr: Box::new([]),
//             size,
//             color: Gray,
//             generation: Young,
//             ref_to: None,
//         }
//     }

//     // white block
//     pub fn from_ptr(size: u64, ptr: *mut u8) -> GCBlock {
//         let _1 = unsafe { std::slice::from_raw_parts_mut(ptr, size as usize) };
//         let mut dst = [];
//         dst.clone_from_slice(_1);
//         GCBlock {
//             ptr: Box::new(dst),
//             size,
//             color: White,
//             generation: Young,
//             ref_to: None,
//         }
//     }
// }

// // if == 0 then never copy xdddd
// pub const YOUNG_GENERATION_MAX_SIZE: u64 = 0;

// pub struct GCHeap {
//     // for fast index
//     pub young_size: u64,
//     pub old_size: u64,
//     pub young_indexs: Vec<u64>,
//     pub old_indexs: Vec<u64>,

//     pub white_indexs: Vec<u64>,
//     pub gray_indexs: Vec<u64>,
//     pub black_indexs: Vec<u64>,

//     pub young_blocks: Vec<Option<HeapHandle>>,
//     pub old_blocks: Vec<Option<HeapHandle>>,
// }

// impl GCHeap {
//     pub fn new_block(&mut self, size: u64) -> u64 {
//         if self.young_size + size >= YOUNG_GENERATION_MAX_SIZE {
//             //gc
//             unimplemented!();
//         } else {
//             let nb = GCBlock::new(size);
//             self.young_blocks.push(Some(This(nb)));
//             let index = self.young_blocks.len() - 1;
//             self.young_indexs.push(index as u64);
//             self.gray_indexs.push(index as u64);
//             index as u64
//         }
//     }
//     pub fn new_user_defined_block(&mut self, size: u64, ptr: *mut u8) -> u64 {
//         if self.young_size + size >= YOUNG_GENERATION_MAX_SIZE {
//             //gc
//             unimplemented!();
//         } else {
//             let nb = GCBlock::from_ptr(size, ptr);
//             self.young_blocks.push(Some(This(nb)));
//             let index = self.young_blocks.len() - 1;
//             self.young_indexs.push(index as u64);
//             self.white_indexs.push(index as u64);
//             index as u64
//         }
//     }
//     pub fn gc(&mut self, root: &mut Vec<(u64, Generation)>) {
//         // TODO: ザ・ワールド
//         let mut root_young_indexs = vec![];
//         let mut root_old_indexs = vec![];
//         for r in 0..root.len() {
//             if root[r].1 == Young {
//                 if let Some(This(data)) = self
//                     .young_blocks
//                     .get(root[r].0 as usize)
//                     .expect("ERROR! INDEX FAILED")
//                 {
//                     if data.generation == Young {
//                         root_young_indexs.push(r);
//                     } else {
//                         root_old_indexs.push(r);
//                     }
//                 } else {
//                     // clean redirect
//                     let mut id = root[r].0;
//                     loop {
//                         let mut cb = &mut self.young_blocks[id as usize];
//                         match cb {
//                             Some(Redirect(idi)) => {
//                                 id = *idi;
//                                 cb = &mut None
//                             }
//                             Some(This(_)) => {
//                                 root[r].0 = id;
//                                 break;
//                             }
//                             None => unreachable!(),
//                         }
//                     }
//                     return self.gc(root);
//                 }
//             } else {
//                 unimplemented!();
//             }
//         }
//         // TODO: end

//         // young
//         let mut new_young = vec![];
//         for r in root_young_indexs.clone() {
//             match self
//                 .young_blocks
//                 .get(root[r].0 as usize)
//                 .expect("ERROR! INDEX FAILED")
//             {
//                 Some(This(data)) => new_young.push(Some(This(data.clone()))),
//                 //TODO: i think is do nothing
//                 Some(Redirect(index)) => unreachable!(),
//                 None => unreachable!(),
//             }
//         }

//         for i in self.young_indexs.clone() {
//             match &mut self.young_blocks[i as usize] {
//                 Some(Redirect(id)) => {
//                     let mut id = *id;
//                     loop {
//                         let mut cb = &mut self.young_blocks[id as usize];
//                         match cb {
//                             Some(Redirect(idi)) => {
//                                 id = *idi;
//                                 cb = &mut None
//                             }
//                             Some(This(_)) => {
//                                 cb = &mut None;
//                                 break;
//                             }
//                             None => unreachable!(),
//                         }
//                     }
//                 }
//                 Some(This(_)) => self.young_blocks[i as usize] = None,
//                 None => unreachable!(),
//             }
//         }
//         // TODO: ザ・ワールド
//         for i in 0..root.len() {
//             root[i].0 = i as u64;
//         }
//         // TODO: end
//         self.young_blocks = new_young;
//     }
// }
