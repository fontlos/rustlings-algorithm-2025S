#![feature(linked_list_cursors)]

use std::collections::LinkedList;

const C1: u64 = 0x87c37b91114253d5;
const C2: u64 = 0x4cf5ad432745937f;
const R1: u32 = 31;
const R2: u32 = 27;
const M: u64 = 0x52dce729;

pub struct HashMap<K, V> {
    buckets: Vec<LinkedList<(K, V)>>,
    size: usize,
    capacity: usize,
    load_factor: f32,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone
{
    pub fn new() -> Self {
        let initial_capacity = 16;
        HashMap {
            buckets: vec![LinkedList::new(); initial_capacity],
            size: 0,
            capacity: initial_capacity,
            load_factor: 0.75,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        HashMap {
            buckets: vec![LinkedList::new(); capacity],
            size: 0,
            capacity,
            load_factor: 0.75,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // 检查是否需要扩容
        if (self.size as f32) >= (self.capacity as f32 * self.load_factor) {
            self.resize();
        }

        let hash = Self::hash(&key);
        let index = (hash as usize) % self.capacity;

        // 检查是否已存在相同的 key
        for entry in self.buckets[index].iter_mut() {
            if entry.0 == key {
                let old_value = std::mem::replace(&mut entry.1, value);
                return Some(old_value);
            }
        }

        // 插入新键值对
        self.buckets[index].push_back((key, value));
        self.size += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = Self::hash(key);
        let index = (hash as usize) % self.capacity;

        for entry in self.buckets[index].iter() {
            if &entry.0 == key {
                return Some(&entry.1);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let hash = Self::hash(key);
        let index = (hash as usize) % self.capacity;

        for entry in self.buckets[index].iter_mut() {
            if &entry.0 == key {
                return Some(&mut entry.1);
            }
        }
        None
    }

    // pub fn remove(&mut self, key: &K) -> Option<V> {
    //     let hash = Self::hash(key);
    //     let index = (hash as usize) % self.capacity;

    //     let prev_link = &mut self.buckets[index];
    //     while !prev_link.is_empty() {
    //         if prev_link.front().unwrap().0 == *key {
    //             // 找到目标节点, 直接移除
    //             self.size -= 1;
    //             return Some(prev_link.pop_front().unwrap().1);
    //         }
    //         // 移动到下一个节点
    //         let mut split_list = prev_link.split_off(1);
    //         std::mem::swap(prev_link, &mut split_list);
    //     }
    //     None
    // }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let hash = Self::hash(key);
        let index = (hash as usize) % self.capacity;

        let mut cursor = self.buckets[index].cursor_front_mut();
        while let Some(entry) = cursor.current() {
            if entry.0 == *key {
                self.size -= 1;
                return Some(cursor.remove_current().unwrap().1);
            }
            cursor.move_next();
        }

        None
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn hash(key: &K) -> u64 {
        use std::hash::Hasher;
        let mut hasher = Murmur3Hasher::new(0);
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_buckets = vec![LinkedList::new(); new_capacity];

        for bucket in self.buckets.drain(..) {
            for (key, value) in bucket {
                let hash = Self::hash(&key);
                let index = (hash as usize) % new_capacity;
                new_buckets[index].push_back((key, value));
            }
        }

        self.buckets = new_buckets;
        self.capacity = new_capacity;
    }
}

struct Murmur3Hasher {
    seed: u64,
    buffer: Vec<u8>,
}

impl Murmur3Hasher {
    fn new(seed: u64) -> Self {
        Murmur3Hasher {
            seed,
            buffer: Vec::new(),
        }
    }
}

impl std::hash::Hasher for Murmur3Hasher {
    fn finish(&self) -> u64 {
        murmurhash3_x64_128(&self.buffer, self.seed)
    }

    fn write(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }
}

/// 64 位 MurmurHash3 实现
fn murmurhash3_x64_128(data: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let mut h1 = seed;
    let mut h2 = seed;

    // 处理 16 字节块
    let chunks = data.chunks_exact(16);
    let remainder = chunks.remainder();

    for chunk in chunks {
        let mut k1 = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
        let mut k2 = u64::from_le_bytes(chunk[8..16].try_into().unwrap());

        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(R1);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;

        h1 = h1.rotate_left(R2);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(M).wrapping_add(0x52dce729);

        k2 = k2.wrapping_mul(C2);
        k2 = k2.rotate_left(R2);
        k2 = k2.wrapping_mul(C1);
        h2 ^= k2;

        h2 = h2.rotate_left(R1);
        h2 = h2.wrapping_add(h1);
        h2 = h2.wrapping_mul(M).wrapping_add(0x38495ab5);
    }

    // 处理尾部数据
    if !remainder.is_empty() {
        let mut k1 = 0;
        let mut k2 = 0;

        match remainder.len() {
            15 => k2 ^= (remainder[14] as u64) << 48,
            14 => k2 ^= (remainder[13] as u64) << 40,
            13 => k2 ^= (remainder[12] as u64) << 32,
            12 => k2 ^= (remainder[11] as u64) << 24,
            11 => k2 ^= (remainder[10] as u64) << 16,
            10 => k2 ^= (remainder[9] as u64) << 8,
            9 => k2 ^= remainder[8] as u64,
            _ => (),
        }

        if !remainder.is_empty() {
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(R2);
            k2 = k2.wrapping_mul(C1);
            h2 ^= k2;
        }

        match remainder.len() {
            8 => k1 ^= (remainder[7] as u64) << 56,
            7 => k1 ^= (remainder[6] as u64) << 48,
            6 => k1 ^= (remainder[5] as u64) << 40,
            5 => k1 ^= (remainder[4] as u64) << 32,
            4 => k1 ^= (remainder[3] as u64) << 24,
            3 => k1 ^= (remainder[2] as u64) << 16,
            2 => k1 ^= (remainder[1] as u64) << 8,
            1 => k1 ^= remainder[0] as u64,
            _ => (),
        }

        if !remainder.is_empty() {
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(R1);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
    }

    // 最终混合
    h1 ^= len as u64;
    h2 ^= len as u64;

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    h1 = fmix64(h1);
    h2 = fmix64(h2);

    h1.wrapping_add(h2)
}

/// 64 位最终混合函数
fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
    k ^= k >> 33;
    k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_map() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());

        map.insert("key1", "value1");
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
        assert_eq!(map.get(&"key1"), Some(&"value1"));
        assert!(map.contains_key(&"key1"));

        map.insert("key2", "value2");
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&"key2"), Some(&"value2"));

        assert_eq!(map.remove(&"key1"), Some("value1"));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"key1"), None);

        *map.get_mut(&"key2").unwrap() = "new_value";
        assert_eq!(map.get(&"key2"), Some(&"new_value"));
    }
}
