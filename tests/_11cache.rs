#![feature(linked_list_cursors)]

use std::collections::{HashMap, LinkedList};
use std::hash::Hash;

pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: LinkedList<K>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        LRUCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            order: LinkedList::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // 将访问的 key 移动到链表尾部 (表示最近使用)
            self.move_to_back(key);
            self.map.get(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // 更新现有值
            self.map.insert(key.clone(), value);
            self.move_to_back(&key);
        } else {
            if self.map.len() >= self.capacity {
                // 移除最久未使用的元素（链表头部）
                if let Some(oldest_key) = self.order.pop_front() {
                    self.map.remove(&oldest_key);
                }
            }
            // 插入新值
            self.map.insert(key.clone(), value);
            self.order.push_back(key);
        }
    }
    // 简单的实现: 先移除再添加到尾部
    // 注意: 对于大型链表这显然不是最高效的方式, 但对于演示目的足够
    // fn move_to_back(&mut self, key: &K) {
    //     // 创建新链表, 将目标 key 放到最后
    //     let mut new_order = LinkedList::new();
    //     let mut found = false;
    //     // 将旧链表中的元素转移到新链表
    //     while let Some(k) = self.order.pop_front() {
    //         if &k == key {
    //             found = true;
    //         } else {
    //             new_order.push_back(k);
    //         }
    //     }
    //     // 如果找到 key, 将其放到链表尾部
    //     if found {
    //         new_order.push_back(key.clone());
    //     }
    //     // 将其他元素放回原链表
    //     while let Some(k) = new_order.pop_front() {
    //         self.order.push_back(k);
    //     }
    // }

    fn move_to_back(&mut self, key: &K) {
        let mut cursor = self.order.cursor_front_mut();
        while let Some(k) = cursor.current() {
            if k == key {
                cursor.remove_current();
                self.order.push_back(key.clone());
                break;
            }
            cursor.move_next();
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_basic() {
        let mut cache = LRUCache::new(2);

        assert_eq!(cache.get(&"a"), None);

        cache.put("a", 1);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"a"), Some(&1));

        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), Some(&2));
    }

    #[test]
    fn test_lru_cache_eviction() {
        let mut cache = LRUCache::new(2);

        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3); // 这会使得键 "a" 被移除

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_lru_cache_update() {
        let mut cache = LRUCache::new(2);

        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), Some(&1)); // 访问 "a" 使其成为最近使用的

        cache.put("c", 3); // 这会使得键 "b" 被移除，而不是 "a"
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_lru_cache_complex() {
        let mut cache = LRUCache::new(3);

        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);
        assert_eq!(cache.get(&"a"), Some(&1)); // a -> c, b, a

        cache.put("d", 4); // 移除 b (最久未使用)
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"c"), Some(&3));
        assert_eq!(cache.get(&"d"), Some(&4));

        cache.put("e", 5); // 移除 a (虽然a被访问过, 但之后 c 和 d 也被访问过)
        assert_eq!(cache.get(&"a"), None);
    }
}
