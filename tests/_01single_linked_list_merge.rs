// 合并两个有序单链表

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    // 指向下一个节点的指针
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    // 指向链表第一个节点的指针
    start: Option<NonNull<Node<T>>>,
    // 指向链表最后一个节点的指针
    end: Option<NonNull<Node<T>>>,
}

// 确保我们的数据是可以比较的
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

// 确保我们的数据是可以比较的
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        // 创建一个新的节点, 并用 Box 分配堆内存
        let mut node = Box::new(Node::new(obj));
        // 确保新节点的 next 指针为 None
        node.next = None;
        // 将 Box 转换为原始指针, 再包装成 NonNull 指针
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        // 检查链表是否为空
        match self.end {
            // 如果链表为空, 新节点就是首节点
            None => self.start = node_ptr,
            // 否则将当前尾节点的 next 指向新节点
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        // 更新尾指针为新节点
        self.end = node_ptr;
        // 增加链表长度
        self.length += 1;
    }

    // 公开方法, 调用私有递归方法
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    // 递归查找第 index 个节点
    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        // 检查当前节点指针
        match node {
            // 如果节点不存在返回 None
            None => None,
            // 如果节点存在
            Some(next_ptr) => match index {
                // 看看是否是第 index 个节点
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                // 否则继续递归查找下一个节点
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // pub fn merge_old(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    // // 合并要求我们的数据是可比较的
    // where
    //     T: std::cmp::PartialOrd,
    // {
    //     let mut merged = LinkedList::new();

    //     // 封装获取节点值和移动节点的逻辑, 最小化 unsafe
    //     let get_val = |node_ptr: NonNull<Node<T>>| unsafe { &(*node_ptr.as_ptr()).val };
    //     let take_next = |node_ptr: NonNull<Node<T>>| unsafe {
    //         let next = (*node_ptr.as_ptr()).next;
    //         let val = std::ptr::read(&(*node_ptr.as_ptr()).val);
    //         (val, next)
    //     };

    //     // 获取 a, b 的头指针作为开始
    //     let mut a_ptr = list_a.start;
    //     let mut b_ptr = list_b.start;

    //     // 开始循环
    //     while let (Some(a_node), Some(b_node)) = (a_ptr, b_ptr) {
    //         // 获取 a, b 当前指针的值作比较, 选择先推入哪一个, 同时更新对应的指针
    //         if get_val(a_node) <= get_val(b_node) {
    //             // 这里直接获取值
    //             let (val, next) = take_next(a_node);
    //             // 使用提供的函数
    //             merged.add(val);
    //             a_ptr = next;
    //         } else {
    //             let (val, next) = take_next(b_node);
    //             merged.add(val);
    //             b_ptr = next;
    //         }
    //     }

    //     // 处理剩余节点
    //     let mut process_remaining = |mut ptr| {
    //         while let Some(node) = ptr {
    //             let (val, next) = take_next(node);
    //             merged.add(val);
    //             ptr = next;
    //         }
    //     };

    //     // 循环 a, b 剩余的内容
    //     process_remaining(a_ptr);
    //     process_remaining(b_ptr);

    //     merged
    // }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: std::cmp::PartialOrd,
    {
        //TODO
        let mut merged = LinkedList::new();

        let mut add_node = |node: NonNull<Node<T>>| {
            // 断开原链接, 防止节点同时属于两个链表, take 将 node 的 next 设为 none 并取出 next, 这是一个原子操作
            let next = unsafe { (*node.as_ptr()).next.take() };
            // SAFETY:
            // merged.end 如果是 Some, 必定是之前添加的合法节点
            match merged.end {
                None => merged.start = Some(node),
                Some(end) => unsafe { (*end.as_ptr()).next = Some(node) },
            }
            merged.end = Some(node);
            merged.length += 1;
            next
        };

        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;

        // 主合并循环
        while let (Some(a), Some(b)) = (a_ptr, b_ptr) {
            if unsafe { (*a.as_ptr()).val <= (*b.as_ptr()).val } {
                a_ptr = add_node(a)
            } else {
                b_ptr = add_node(b)
            };
        }

        // 处理剩余节点
        while let Some(curr) = a_ptr {
            a_ptr = add_node(curr);
        }
        while let Some(curr) = b_ptr {
            b_ptr = add_node(curr);
        }

        merged
    }

    pub fn find(&self, value: &T) -> Option<NonNull<Node<T>>>
    where
        T: std::cmp::PartialOrd,
    {
        let mut current = self.start;
        while let Some(node) = current {
            unsafe {
                if &(*node.as_ptr()).val == value {
                    return Some(node);
                }
                current = (*node.as_ptr()).next;
            }
        }
        None
    }

    /// 更新指定节点的值
    pub fn update(&mut self, node: NonNull<Node<T>>, new_value: T) {
        unsafe {
            (*node.as_ptr()).val = new_value;
        }
    }

    /// 删除指定节点
    pub fn remove(&mut self, target: NonNull<Node<T>>) -> Option<T> {
        unsafe {
            // 处理头节点特殊情况
            if Some(target) == self.start {
                let node = Box::from_raw(target.as_ptr());
                self.start = node.next;
                if self.end == Some(target) {
                    self.end = None;
                }
                self.length -= 1;
                return Some(node.val);
            }

            // 查找前驱节点
            let mut prev = None;
            let mut current = self.start;
            while let Some(node) = current {
                if (*node.as_ptr()).next == Some(target) {
                    prev = Some(node);
                    break;
                }
                current = (*node.as_ptr()).next;
            }

            if let Some(prev_node) = prev {
                let target_node = Box::from_raw(target.as_ptr());
                (*prev_node.as_ptr()).next = target_node.next;
                // 更新尾指针
                if Some(target) == self.end {
                    self.end = Some(prev_node);
                }
                self.length -= 1;
                Some(target_node.val)
            } else {
                None
            }
        }
    }

    /// 在指定节点后插入新值
    pub fn insert_after(&mut self, node: NonNull<Node<T>>, value: T) {
        unsafe {
            let new_node = Box::new(Node {
                val: value,
                next: (*node.as_ptr()).next,
            });
            let new_node_ptr = NonNull::new(Box::into_raw(new_node));
            (*node.as_ptr()).next = new_node_ptr;
            // 更新尾指针
            if self.end == Some(node) {
                self.end = new_node_ptr;
            }
            self.length += 1;
        }
    }

    /// 在链表头部插入
    pub fn push_front(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.start;
        let new_node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };
        if self.end.is_none() {
            self.end = Some(new_node_ptr);
        }
        self.start = Some(new_node_ptr);
        self.length += 1;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_crud() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        // 查找
        let node = list.find(&2).unwrap();

        // 更新
        list.update(node, 4);
        assert_eq!(list.get(1), Some(&4));

        // 插入
        list.insert_after(node, 5);
        assert_eq!(list.length, 4);
        assert_eq!(list.get(2), Some(&5));

        // 删除
        let val = list.remove(node).unwrap();
        assert_eq!(val, 4);
        assert_eq!(list.length, 3);

        // 头部插入
        list.push_front(0);
        assert_eq!(list.get(0), Some(&0));
        assert_eq!(list.length, 4);
    }
}
