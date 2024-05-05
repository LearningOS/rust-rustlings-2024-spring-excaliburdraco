/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,q
}
//定义一个链表，其中包含一个头节点，以及一个尾节点，尾结点类型为Option<NonNull<Node<T>>>，NonNull<Node<T>>是一个智能指针，指向一个Node<T>类型的节点.

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
//impl Node<T>的new方法，该方法创建一个新的Node<T>类型的节点，并将val设置为t，t表示一个泛型参数，表示节点的值，next设置为None.


#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}
/*定义一个linked list，作用是存储一系列的节点。
 ，其中包含一个length字段，表示链表的长度，一个start字段，表示链表的起始节点，一个end字段，表示链表的结束节点。
start和end字段都是Option<NonNull<Node<T>>>类型的，表示链表的起始节点和结束节点，NonNull<Node<T>>是一个智能指针，指向一个Node<T>类型的节点。 
 */

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
/*imp LinkedList<T>的default方法，该方法返回一个默认的LinkedList<T>实例，即一个空链表。
那么空链表的length字段为0，start和end字段为None。
Default单独impl，是因为它是一个关联函数，关联函数的作用是定义一个类型T的默认值。
 */

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
    /*imp LinkedList<T>的new方法，该方法创建一个新的LinkedList<T>实例，并初始化其length字段为0，start和end字段为None。
    new方法返回一个新的LinkedList<T>实例，与Default方法返回的默认实例相同。
    default方法在外面定义，因为它是一个关联函数，关联函数的作用是定义一个类型T的默认值。
    new方法的作用是创建一个新的LinkedList<T>实例，并初始化其length字段为0，start和end字段为None.
    */

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));  //创建一个新的Node<T>类型的节点，并将obj设置为其值字段，并将next字段设置为None.
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }
    /*imp LinkedList<T>的add方法，该方法将一个元素添加到链表的末尾。
    首先，创建一个新的Node<T>类型的节点，并将obj设置为其val字段，并将next字段设置为None。
    然后，创建node_ptr，是一个Option<NonNull<Node<T>>>类型的智能指针，指向一个Node<T>类型的节点。 将node包装在一个Box中，并将其转换为NonNull<Node<T>>类型的智能指针。
    接下来，match判断链表是否为空，将新创建的节点设置为链表的起始节点或结束节点。
    如果链表不为空，则采用unsafe的方式，将链表的结束节点的next字段设置为新创建的节点。
    最后，将链表的长度加1。
    */

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }
    /*imp LinkedList<T>的get方法，接收一个索引值作为参数，返回链表中指定索引位置的元素的引用。
    首先，调用get_ith_node方法，传入链表的起始节点和索引值。
    返回值是Option<&T>类型的，表示可能存在也可能不存在的元素的引用。
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
    /*get_ith_node方法接收两个参数：一个链表的起始节点和一个索引值。
    
    首先，检查链表是否为空，如果为空，则返回None。
    如果链表不为空，则检查索引值是否为0，如果是0，则返回链表的起始节点的val字段的引用。
    否则，递归调用get_ith_node方法，传入链表的下一个节点和索引值减1。
    */

	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{
		//TODO
 
		Self {
            length: 0,
            start: None,
            end: None,
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
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}