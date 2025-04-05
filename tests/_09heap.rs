// 实现一个基本的二叉堆

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;  // 新元素的索引

        // 上浮过程
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    // 找出当前节点的最小(或最大)子节点索引
    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right > self.count {
            left
        } else {
            if (self.comparator)(&self.items[left], &self.items[right]) {
                left
            } else {
                right
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    // 取出堆顶元素(索引1的元素)
    // 将最后一个元素移到堆顶
    // 执行"下沉"(sink)操作，恢复堆的性质
    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            return None;
        }

        // 取出堆顶元素
        let top = self.items.swap_remove(1);
        self.count -= 1;

        if self.count > 0 {
            // 下沉过程
            let mut idx = 1;
            while self.children_present(idx) {
                let child_idx = self.smallest_child_idx(idx);
                if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                    self.items.swap(idx, child_idx);
                    idx = child_idx;
                } else {
                    break;
                }
            }
        }

        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

/// 获取最小的K个元素
pub fn top_k_min<T: Ord + Default>(nums: Vec<T>, k: usize) -> Vec<T> {
    if k == 0 || nums.is_empty() {
        return vec![];
    }

    let mut heap = MaxHeap::new(); // 最大堆保留最小的 K 个元素

    for num in nums {
        if heap.len() < k {
            heap.add(num);
        } else {
            if &num < heap.items.get(1).unwrap() {
                heap.next(); // 移除当前堆顶
                heap.add(num);
            }
        }
    }

    heap.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }

    #[test]
    fn test_top_k_min() {
        let nums = vec![4, 1, 3, 12, 7, 5];
        // 由于 next 函数的行为会导致结果是反转的
        assert_eq!(top_k_min(nums.clone(), 3), vec![4, 3, 1]);
        assert_eq!(top_k_min(nums.clone(), 5), vec![7, 5, 4, 3, 1]);
        assert_eq!(top_k_min(nums.clone(), 0), vec![]);
    }

    #[test]
    fn test_edge_cases() {
        // 元素全部相同
        assert_eq!(top_k_min(vec![2, 2, 2], 2), vec![2, 2]);
        // K 大于数组长度
        assert_eq!(top_k_min(vec![1, 2], 5), vec![2, 1]);
    }
}
