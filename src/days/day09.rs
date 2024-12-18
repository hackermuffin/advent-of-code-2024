use core::panic;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Block {
    File(u32, u8),
    Empty(u8),
}

impl Block {
    fn len(&self) -> u32 {
        *match self {
            Self::File(_, len) => len,
            Self::Empty(len) => len,
        } as u32
    }
}

#[derive(Debug)]
struct DiskMap {
    data: Vec<Block>,
}

impl DiskMap {
    fn new(input: &str) -> Self {
        let mut file = false;
        let mut id = 0;
        DiskMap {
            data: input
                .trim()
                .chars()
                .map(|char| {
                    file = !file;
                    let len = str::parse(&char.to_string()).expect("Invalid block length: {char}");
                    if file {
                        let res = Block::File(id, len);
                        id += 1;
                        res
                    } else {
                        Block::Empty(len)
                    }
                })
                .collect(),
        }
    }

    fn raw_blocks(&self) -> Vec<Option<u32>> {
        let mut arr = Vec::new();
        for block in self.data.iter() {
            let val = match block {
                Block::File(id, _) => Some(*id),
                Block::Empty(_) => None,
            };
            for _ in 0..block.len() {
                arr.push(val);
            }
        }
        arr
    }

    fn rearrange_blocks(&self) -> Vec<Option<u32>> {
        let arr = self.raw_blocks();
        let mut dequeue = arr.clone().into_iter().flatten().collect::<VecDeque<_>>();
        let mut res = Vec::new();

        for i in 0..arr.len() {
            let block = arr.get(i).unwrap();
            match block {
                Some(_) => res.push(dequeue.pop_front()),
                None => res.push(dequeue.pop_back()),
            }
        }

        res
    }

    fn rearrange_files(&mut self) {
        fn insert(vec: &mut Vec<Block>, block: Block) -> Option<()> {
            let Block::File(_, insert_size) = block else {
                panic!("Cannot insert empty")
            };
            let (index, empty) = vec.iter_mut().enumerate().find(|(_, b)| match b {
                Block::Empty(size) => *size >= insert_size,
                _ => false,
            })?;

            let empty = std::mem::replace(empty, block);
            let Block::Empty(empty_size) = empty else {
                panic!("Empty block is not empty")
            };

            let diff = empty_size - insert_size;
            if diff > 0 {
                let new = Block::Empty(diff);
                vec.insert(index + 1, new);
            }

            Some(())
        }

        let inserts = self
            .data
            .clone()
            .into_iter()
            .filter(|x| matches!(x, Block::File(_, _)))
            .rev();

        let mut src = &mut self.data;

        for block in inserts {
            if let Block::File(insert_id, insert_size) = block {
                // Replace file with empty before insert
                let i = src
                    .iter()
                    .position(|block| match block {
                        Block::File(id, _) => insert_id == *id,
                        _ => false,
                    })
                    .expect("Unable to find id {insert_id} in vector");
                let block = std::mem::replace(src.get_mut(i).unwrap(), Block::Empty(insert_size));

                // Insert element, worst case inserts at old position
                insert(&mut src, block);
            }
        }
    }
}

fn checksum(blocks: &[Option<u32>]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .map(|(i, id)| match id {
            Some(id) => i * *id as usize,
            None => 0,
        } as u64)
        .sum()
}

pub fn run(input: String) {
    let mut map = DiskMap::new(&input);

    // Pt 1
    let new = map.rearrange_blocks();
    let sum = checksum(&new);
    println!("{sum}");

    // Pt 2
    map.rearrange_files();
    let sum = checksum(&map.raw_blocks());
    println!("{sum}");
}
