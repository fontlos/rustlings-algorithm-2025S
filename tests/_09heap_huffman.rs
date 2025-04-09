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

#[derive(Debug, Default)]
pub struct HuffmanNode {
    pub character: Option<char>,  // None表示内部节点
    pub frequency: usize,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.frequency.cmp(&other.frequency))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for HuffmanNode {}

use std::collections::HashMap;

pub fn build_huffman_tree(text: &str) -> Option<Box<HuffmanNode>> {
    if text.is_empty() {
        return None;
    }

    // 1. 统计字符频率
    let mut freq_map = HashMap::new();
    for c in text.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }

    // 2. 创建最小堆并插入所有字符节点
    let mut heap = MinHeap::new();
    for (character, frequency) in freq_map {
        heap.add(Box::new(HuffmanNode {
            character: Some(character),
            frequency,
            left: None,
            right: None,
        }));
    }

    // 3. 构建Huffman树
    while heap.len() > 1 {
        let left = heap.next().unwrap();
        let right = heap.next().unwrap();

        let merged = Box::new(HuffmanNode {
            character: None,
            frequency: left.frequency + right.frequency,
            left: Some(left),
            right: Some(right),
        });

        heap.add(merged);
    }

    heap.next()  // 返回最终的Huffman树根节点
}

pub fn build_huffman_codebook(root: &Box<HuffmanNode>) -> HashMap<char, String> {
    let mut codebook = HashMap::new();
    let mut stack = Vec::new();

    if let Some(ref c) = root.character {
        // 特殊情况：只有一个字符
        codebook.insert(*c, "0".to_string());
        return codebook;
    }

    stack.push((root, String::new()));

    while let Some((node, code)) = stack.pop() {
        if let Some(ref c) = node.character {
            codebook.insert(*c, code);
            continue;
        }

        if let Some(ref left) = node.left {
            stack.push((left, code.clone() + "0"));
        }

        if let Some(ref right) = node.right {
            stack.push((right, code + "1"));
        }
    }

    codebook
}

pub fn huffman_encode(text: &str) -> (Option<Box<HuffmanNode>>, String) {
    let tree = build_huffman_tree(text);
    if tree.is_none() {
        return (None, String::new());
    }

    let codebook = build_huffman_codebook(&tree.as_ref().unwrap());
    let mut encoded = String::new();

    for c in text.chars() {
        encoded.push_str(&codebook[&c]);
    }

    (tree, encoded)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_single_char() {
        let text = "aaaaa";
        let (tree, encoded) = huffman_encode(text);
        assert!(tree.is_some());
        assert_eq!(encoded, "00000");
    }

    #[test]
    fn test_huffman_encoding() {
        let text = "this is an example of a huffman tree";
        let (tree, encoded) = huffman_encode(text);
        assert!(tree.is_some());

        // 验证编码长度比原始文本短
        let original_bits = text.len() * 8;
        let encoded_bits = encoded.len();
        assert!(encoded_bits < original_bits);

        // 验证不同字符有不同的前缀编码
        let codebook = build_huffman_codebook(&tree.unwrap());
        let codes: Vec<&String> = codebook.values().collect();
        for i in 0..codes.len() {
            for j in i+1..codes.len() {
                assert!(!codes[i].starts_with(codes[j]));
                assert!(!codes[j].starts_with(codes[i]));
            }
        }
    }

    #[test]
    fn test_empty_input() {
        let (tree, encoded) = huffman_encode("");
        assert!(tree.is_none());
        assert!(encoded.is_empty());
    }
}
