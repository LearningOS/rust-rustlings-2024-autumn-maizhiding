use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
// 移除未使用的导入: use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let node = Box::new(Node::new(obj));
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = node_ptr;
            },
        }

        if let Some(node_ptr_inner) = node_ptr {
            unsafe {
                (*node_ptr_inner.as_ptr()).prev = self.end;
            }
        }

        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn reverse(&mut self) {
        let mut current = self.start;
        // 移除未使用的 new_end 变量
        // let mut new_end = self.start; // 移除此行

        while let Some(mut node_ptr) = current {
            // 将 node_ptr 声明为可变
            unsafe {
                let node = node_ptr.as_mut();

                // 交换 next 和 prev
                let temp = node.next;
                node.next = node.prev;
                node.prev = temp;

                // 继续遍历原来的 next（现在是 prev）
                current = node.prev;

                // 更新 new_end 为当前节点（移除此行，因为 new_end 不再使用）
                // new_end = Some(node_ptr); // 移除此行
            }
        }

        // 交换 start 和 end
        std::mem::swap(&mut self.start, &mut self.end);
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(node_ptr) = current {
            unsafe {
                let node = Box::from_raw(node_ptr.as_ptr());
                current = node.next;
                // `node` 会在这里被销毁，自动释放内存
            }
        }
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
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for &item in &original_vec {
            list.add(item);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for &item in &original_vec {
            list.add(item);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}
