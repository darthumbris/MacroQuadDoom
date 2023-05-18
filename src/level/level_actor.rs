use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Actor {

}

#[derive(Default)]
pub struct ClassActor {

}

#[derive(Default)]
pub struct EventManager {}

#[derive(Default)]
pub struct BehaviorContainer {}

pub struct StrifeDialogueNode {}

#[derive(Default)]
pub struct CajunMaster {}


#[derive(Default)]
pub struct Player {}

#[derive(Default)]
pub struct ThinkerCollection {}

#[derive(Default)]
pub struct PlayerStart {}

#[derive(Default)]
pub struct SequenceNode {}

pub struct HealthGroup {}

#[derive(Default)]
pub struct BlockMap {
    pub blockmap_lump: Vec<i32>,
    pub blockmap: Vec<i32>,
    pub blockmap_width: i32,
    pub blockmap_height: i32,
    pub blockmap_origin_x: f64,
    pub blockmap_origin_y: f64,
    pub block_links: Vec<BlockNode>
}

impl BlockMap {
    pub fn verify_blockmap(&self, count: usize, line_count: usize) -> bool {
        let max_offset = self.blockmap_lump.len() + count;
        let bmap_width = self.blockmap_width;
        let bmap_height = self.blockmap_height;

        for i in 0..bmap_height {
            for j in 0..bmap_width {
                let mut offset: usize;
                let block_offset;

                offset = (i * bmap_width + j) as usize;
                block_offset = self.blockmap_lump.len() + offset as usize; // + 4 not needed because already removed

                if block_offset >= max_offset {
                    println!("Verifying blockmap: block offset overflow");
                    return false
                }

                offset = block_offset;

                if offset < i32::max_value() as usize || offset >= count {
                    println!("Verifying blockmap: list offset overflow");
                    return false
                }

                for k in offset..self.blockmap_lump.len() {
                    if k >= max_offset {
                        println!("Verifying blockmap: open blockmap");
                        return false
                    }
                    if self.blockmap_lump[k] == -1 {break;}
                }

                if self.blockmap_lump[offset] != 0 {
                    println!("Verifying blockmap: first entry is not 0");
                    return false
                }

                for k in offset..self.blockmap_lump.len() {
                    if self.blockmap_lump[k] as usize >= line_count {
                        println!("Verifying blockmap: index >= linecount");
                        return false
                    }
                    if self.blockmap_lump[k] == -1 {break;}
                }
            }
        }
        true
    }
}

pub type BlockNodeIndex = i32;

#[derive(Clone)]
pub struct BlockNode {
    me: Option<Rc<Actor>>,
    block_index: i32,
    group: i32,
    prev_actor: BlockNodeIndex,
    next_actor: BlockNodeIndex,
    prev_block: BlockNodeIndex,
    next_block: BlockNodeIndex
}

impl BlockNode {
    pub fn new() -> BlockNode {
        BlockNode { me: None, block_index: 0, group: 0, prev_actor: -1, next_actor: -1, prev_block: -1, next_block: -1 }
    }
}