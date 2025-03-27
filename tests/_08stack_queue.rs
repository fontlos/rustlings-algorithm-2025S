// 使用栈实现一个队列

#[derive(Debug)]
pub struct Queue<T> {
    in_stack: Vec<T>,   // 用于入队操作的栈
    out_stack: Vec<T>,  // 用于出队操作的栈
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    /// 入队操作 - O(1)时间复杂度
    pub fn enqueue(&mut self, elem: T) {
        self.in_stack.push(elem);
    }

    /// 出队操作 - 均摊O(1)时间复杂度
    pub fn dequeue(&mut self) -> Option<T> {
        // 如果out_stack为空，将in_stack的所有元素转移到out_stack
        if self.out_stack.is_empty() {
            while let Some(elem) = self.in_stack.pop() {
                self.out_stack.push(elem);
            }
        }
        self.out_stack.pop()
    }

    /// 查看队首元素 - 均摊O(1)时间复杂度
    pub fn peek(&mut self) -> Option<&T> {
        // 同样需要先确保out_stack有元素
        if self.out_stack.is_empty() {
            while let Some(elem) = self.in_stack.pop() {
                self.out_stack.push(elem);
            }
        }
        self.out_stack.last()
    }

    /// 返回队列大小 - O(1)时间复杂度
    pub fn size(&self) -> usize {
        self.in_stack.len() + self.out_stack.len()
    }

    /// 检查队列是否为空 - O(1)时间复杂度
    pub fn is_empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_queue() {
        let mut q = Queue::new();
        assert_eq!(q.dequeue(), None);
        assert!(q.is_empty());

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.size(), 3);
        assert_eq!(q.peek(), Some(&1));

        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));

        q.enqueue(4);
        q.enqueue(5);
        assert_eq!(q.size(), 3);
        assert_eq!(q.peek(), Some(&3));

        assert_eq!(q.dequeue(), Some(3));
        assert_eq!(q.dequeue(), Some(4));
        assert_eq!(q.dequeue(), Some(5));
        assert_eq!(q.dequeue(), None);
        assert!(q.is_empty());
    }
}