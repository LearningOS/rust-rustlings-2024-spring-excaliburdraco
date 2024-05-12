/*
    double linked list reverse
    This problem requires you to reverse a doubly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}
/*定义一个双链表的节点，
其中包含指向下一个节点的指针next，
以及指向前一个节点的指针prev。
*/

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
/*为结构体Node实现一个构造函数new，
该构造函数接收一个参数t，并将其赋值给结构体Node的val字段。
 */
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}
/*定义一个双链表的结构体LinkedList，
lenth字段表示链表的长度，start和end字段分别指向链表的头节点和尾节点。
start和end字段与Node类型中的next和prev字段相对应。
Node类型是一个结构体，包含val字段表示节点的值，

*/

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
/*为结构体LinkedList实现一个默认构造函数default，
在default()内部调用new()函数创建一个新的LinkedList实例。
*/

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
    /*为结构体LinkedList实现一个构造函数new，*/

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));   
        // 传值路径add(obj)-> Node::new(obj)-> Box::new(Node::new(obj))-> node -> Box::into_raw(node)-> NonNull::new_unchecked(Box::into_raw(node))
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }
    /*为结构体LinkedList实现一个方法add，该方法接收一个参数obj，
    表示要添加到链表中的值。方法内部首先创建一个新的节点node，
    Node::new(obj)构造函数接收一个参数obj，表示要添加到链表中的值。Box::new(Node::new(obj))创建一个Box对象，
    并将Node::new(obj)的结果存储在Box对象中。
    将Node::new(obj)存在Box对象中是为了把node的内存分配在堆上。
    然后将node的next字段设置为None，
    将node的prev字段设置为self.end，
    将node的指针存储在node_ptr变量中，Box::into_raw(node)将Box对象转换为裸指针，NonNull::new_unchecked(Box::into_raw(node))将裸指针包装为NonNull类型的指针。
    Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) })将NonNull类型的指针存储在node_ptr变量中。
    match self.end 用来匹配self.end的值，如果self.end为None，则将node_ptr赋值给self.start，
    如果self.end为Some，则将node的指针存储在self.end的next字段中。
    self.end = node_ptr; 将node的指针存储在self.end中。这与存在next字段中不同，存在next字段中表示下一个节点，而存在self.end中表示尾节点。
    self.length += 1; 将链表的长度加1。
    */

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }
    /*为结构体LinkedList实现一个方法get，该方法接收一个参数index，表示要获取的节点的索引。
    返回一个Option<&T>类型的值，表示获取到的节点的值。
    方法内部调用get_ith_node方法，传入self.start和index参数。
    get_ith_node方法接收两个参数，第一个参数表示要获取的节点，第二个参数表示要获取的节点的索引。
    */

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    /* 为结构体LinkedList实现一个方法get_ith_node，该方法接收两个参数，
    第一个参数表示要获取的节点，第二个参数表示要获取的节点的索引。
    返回一个Option<&T>类型的值，表示获取到的节点的值。
    方法内部首先匹配node的值，如果node为None，则返回None，
    如果node为Some，则调用match语句来匹配index的值。
    如果index为0，则返回Some(unsafe { &(*next_ptr.as_ptr()).val })，
    其中unsafe { &(*next_ptr.as_ptr()).val }表示获取下一个节点的值。
    如果index不为0，则递归调用self.get_ith_node方法，传入unsafe { (*next_ptr.as_ptr()).next }和index - 1参数。

     */
    pub fn reverse(&mut self) {
        // TODO
        let mut current_ptr = self.start;
        while let Some(mut current) = current_ptr {
            // Swap the next and prev pointers
            unsafe {
                //方法一
                // let next_temp = current.as_ref().next;
                // (*current.as_ptr()).next = current.as_ref().prev;
                // (*current.as_ptr()).prev = next_temp;
                
                // 方法二
                std::mem::swap(&mut (*current.as_ptr()).next, &mut (*current.as_ptr()).prev);
            }
            // Move to the next node, which is actually the previous node
            current_ptr = unsafe { (*current.as_ptr()).prev };
        }
        // Swap the start and end pointers
        std::mem::swap(&mut self.start, &mut self.end);
    }
    /*  为结构体LinkedList实现一个方法reverse，该方法将链表反转。
    方法内部首先获取链表的第一个节点，并将其存储在current_ptr变量中。
    然后使用while循环遍历链表，每次循环都将current_ptr的值存储在current变量中。
    在每次循环中，调用unsafe { }块来交换current节点的next和prev指针。

    然后将current_ptr的值更新为current节点的prev指针，表示移动到下一个节点。
    最后，交换self.start和self.end的值，实现链表的反转。
     */
    
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
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
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
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}
