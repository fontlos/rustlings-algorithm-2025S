// 合并两个有序单链表, 函数式方案

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node<T> {
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

    // pub fn iter(&self) -> LinkedListIter<'_, T> {
    //     LinkedListIter {
    //         current: self.start,
    //         _marker: std::marker::PhantomData,
    //     }
    // }

    // pub fn iter_mut(&mut self) -> LinkedListIterMut<T> {
    //     LinkedListIterMut {
    //         current: self.start,
    //         _marker: std::marker::PhantomData,
    //     }
    // }

    fn into_iter(self) -> LinkedListIntoIter<T> {
        LinkedListIntoIter {
            current: self.start,
        }
    }

    fn append_node(&mut self, node: NonNull<Node<T>>) {
        unsafe{ (*node.as_ptr()).next = None };
        match self.end {
            None => self.start = Some(node),
            Some(end) => unsafe{ (*end.as_ptr()).next = Some(node) },
        }
        self.end = Some(node);
        self.length += 1;
    }

    pub fn merge(&mut self, other: LinkedList<T>) where T: PartialOrd {
        let mut other_iter = other.into_iter();
        let mut current = &mut self.start;

        while let Some(self_node) = current.as_ref().map(|n| unsafe { &*n.as_ptr() }) {
            if let Some(other_node_ptr) = other_iter.current {
                unsafe {
                    if (*other_node_ptr.as_ptr()).val <= self_node.val {
                        // 插入 other 的节点到当前节点前
                        let other_node = other_iter.next().unwrap();
                        (*other_node.as_ptr()).next = *current;
                        *current = Some(other_node);
                        self.length += 1;
                        continue;
                    }
                }
            }
            current = unsafe { &mut (*current.unwrap().as_ptr()).next };
        }

        // 处理剩余的 other 节点
        for node in other_iter {
            self.append_node(node);
        }
    }
}

// 不可变迭代器
pub struct LinkedListIter<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _marker: std::marker::PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            let node_ref = &*node.as_ptr();
            self.current = node_ref.next;
            &node_ref.val
        })
    }
}

// 消耗迭代器
pub struct LinkedListIntoIter<T> {
    current: Option<NonNull<Node<T>>>,
}

impl<T> Iterator for LinkedListIntoIter<T> {
    type Item = NonNull<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            let next = (*node.as_ptr()).next;
            (*node.as_ptr()).next = None; // 断开连接
            self.current = next;
            node
        })
    }
}

// 可变迭代器
pub struct LinkedListIterMut<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _marker: std::marker::PhantomData<&'a mut Node<T>>,
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            let node_ref = &mut *node.as_ptr();
            self.current = node_ref.next;
            &mut node_ref.val
        })
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
        list_a.merge(list_b);
        println!("merged List is {}", list_a);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_a.get(i as i32).unwrap());
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
        list_a.merge(list_b);
        println!("merged List is {}", list_a);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_a.get(i as i32).unwrap());
        }
    }
}
