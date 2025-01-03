use std::{fs, iter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryBlock {
    File { file_id: usize },
    Free,
}
pub struct DiskMap {
    data: Vec<MemoryBlock>,
}

#[derive(Debug)]
struct FreeBlock {
    start_index: usize,
    size: usize,
}

impl DiskMap {
    ///Takes as input an explicit disk map.
    ///Compacts the amphipod's hard drive using the process they requested.
    pub fn build_and_compact(mut disk_map: Vec<MemoryBlock>) -> DiskMap {
        //Compressing the disk map
        let mut left_index = 0; //The current first occurence of free memory (i.e. '.') in the vector
        'Compress: for index in (0..disk_map.len()).rev() {
            //Note there is no situation in which we move a file to the right, so we only care when disk_map[index] != '.'
            if let MemoryBlock::File { .. } = disk_map[index] {
                //Find the next occurence of '.' in the vector
                left_index = loop {
                    if left_index < index {
                        if let MemoryBlock::Free = disk_map[left_index] {
                            break left_index;
                        }
                        left_index += 1;
                    } else {
                        //meaning there are no more '.' in the vector so compression is done
                        break 'Compress;
                    }
                };
                disk_map.swap(left_index, index);
                left_index += 1; //Since disk_map[left_index] != '.'
            }
        }

        //Note that after the for loop our disk_map vector is a sequence of digits and then a bunch of '.' .
        //Also note that left_index = the last index in the Vector that is not '.' .
        //while we could do disk_map.truncate(left_index + 1), we preserve the entire data.
        DiskMap { data: disk_map }
    }

    ///Returns the resulting filesystem checksum
    pub fn checksum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .take_while(|(_, entry)| **entry != MemoryBlock::Free)
            .map(|(index, entry)| {
                let MemoryBlock::File { file_id } = *entry else {
                    panic!("impossible as entry must be a File");
                };
                index * file_id
            })
            .sum()
    }

    ///Create a helper vector of where FreeBlocks are in the diskmap and what their size is
    fn find_free_blocks(disk_map: &[MemoryBlock]) -> Vec<FreeBlock> {
        let mut free_blocks: Vec<FreeBlock> = Vec::new();

        let mut new_free_block = true;
        let mut size = 0;
        let mut start_index = 0;

        //Note we chain a meaningless file to the end of the iterator so we guarantee our iterator ends with a file
        for (index, entry) in disk_map
            .iter()
            .chain(iter::once(&MemoryBlock::File { file_id: 42 }))
            .enumerate()
        {
            if let MemoryBlock::Free = *entry {
                if new_free_block {
                    start_index = index;
                    size = 1;
                    new_free_block = false;
                } else {
                    size += 1;
                }
            } else if !new_free_block {
                //meaning this is a file encountered after a free block.
                //So when we next encounter a '.' it will denote a new free_block
                new_free_block = true;

                //bank the old free block
                free_blocks.push(FreeBlock { start_index, size });
            }
        }

        free_blocks
    }

    ///Takes as input an explicit disk map.
    ///Compacts the amphipod's hard drive using the **NEW** process they requested.
    pub fn build_and_compact_part2(mut disk_map: Vec<MemoryBlock>) -> DiskMap {
        //First we create a helper vector of where FreeBlocks are and what their size is
        let mut free_blocks = DiskMap::find_free_blocks(&disk_map);

        //Compressing the disk map
        let mut right_index = if disk_map.is_empty() {
            panic!("The disk map is blank")
        } else {
            disk_map.len() - 1
        };

        'Compress: while right_index > 0 {
            if let MemoryBlock::File { file_id } = disk_map[right_index] {
                //Find the size of this file
                let mut file_size = 0;

                let current_file = MemoryBlock::File { file_id };

                while current_file == disk_map[right_index] {
                    file_size += 1;
                    if right_index == 0 {
                        break 'Compress;
                    }
                    right_index -= 1;
                }
                //Note right_index+1 is the index of where the file starts
                let file_start_index = right_index + 1;

                //Find the next occurence of free space of sufficent size in the vector **To the LEFT** of the file
                //and move the file there
                for free_block in free_blocks.iter_mut() {
                    //We must move the file left, and we know free_blocks is ordered so we can do short-circuting
                    if free_block.start_index > file_start_index {
                        break;
                    }

                    if free_block.size >= file_size {
                        //Move the file to the free space
                        for idx in 0..file_size {
                            disk_map.swap(free_block.start_index + idx, file_start_index + idx);
                        }

                        //Update this free block accordingly
                        free_block.size -= file_size;
                        free_block.start_index += file_size;

                        //Since we found a free space, no need to keep searching
                        break;
                    }
                }
            } else {
                right_index -= 1;
            }
        }

        DiskMap { data: disk_map }
    }

    ///Returns the resulting filesystem checksum
    pub fn checksum_part2(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, entry)| **entry != MemoryBlock::Free)
            .map(|(index, entry)| {
                let MemoryBlock::File { file_id } = *entry else {
                    panic!("impossible as entry must be a File");
                };
                index * file_id
            })
            .sum()
    }
}

fn solution_part1(file_path: &str) -> usize {
    //Note char_index can be a multiple digit number which is why we use MemoryBlock

    let mut explicit_disk_map: Vec<MemoryBlock> = Vec::new(); //An explict representation of the disk map.

    for (char_index, char) in fs::read_to_string(file_path)
        .expect("failed to open file")
        .chars()
        .enumerate()
    {
        let num_of_blocks = char.to_digit(10).expect("char should be a digit from 0-9") as usize;

        let block = if char_index % 2 == 0 {
            //Note the file_ID is simply char_index/2.
            MemoryBlock::File {
                file_id: (char_index / 2),
            }
        } else {
            MemoryBlock::Free
        };

        explicit_disk_map.extend(iter::repeat_n(block, num_of_blocks));
    }

    let disk_map = DiskMap::build_and_compact(explicit_disk_map);
    disk_map.checksum()
}

fn solution_part2(file_path: &str) -> usize {
    let mut explicit_disk_map: Vec<MemoryBlock> = Vec::new(); //An explict representation of the disk map.

    for (char_index, char) in fs::read_to_string(file_path)
        .expect("failed to open file")
        .chars()
        .enumerate()
    {
        let num_of_blocks = char.to_digit(10).expect("char should be a digit from 0-9") as usize;

        let block = if char_index % 2 == 0 {
            //Note the file_ID is simply char_index/2.
            MemoryBlock::File {
                file_id: (char_index / 2),
            }
        } else {
            MemoryBlock::Free
        };

        explicit_disk_map.extend(iter::repeat_n(block, num_of_blocks));
    }

    let disk_map = DiskMap::build_and_compact_part2(explicit_disk_map);
    disk_map.checksum_part2()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day9.txt"));
        dbg!(solution_part2("puzzle_inputs/day9.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day9example.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day9example.txt");
        assert_eq!(result, 2858);
    }
}
