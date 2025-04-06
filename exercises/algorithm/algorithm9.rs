/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::os::unix::process::parent_id;

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
        self.items.push(value);
        self.count+=1;
        self.heapify_up(self.count);
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

    fn smallest_child_idx(&self, idx: usize) -> Option<usize> {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 如果左子节点不存在，直接返回右子节点索引
        if right >= self.count {
            return if left < self.count { Some(left) } else { None };
        }

        // 比较左右子节点，返回较小的子节点
        if  (self.comparator)(&self.items[left], &self.items[right]) {
            Some(left)
        } else {
            Some(right)
        }
    }
    fn heapify_up(&mut self, idx: usize) {
        let mut idx = idx;
        while idx != 1 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }
    fn heapify_down(&mut self, idx: usize) {
        let mut idx = idx;
        while let Some(child_idx) = self.smallest_child_idx(idx) {
            if (self.comparator)(&self.items[child_idx],&self.items[idx]){
                self.items.swap(idx, child_idx);
                idx = child_idx;
            }else{
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.items.swap(1, self.count ); // 将堆顶元素与最后一个元素交换
        self.count -= 1;
        self.heapify_down(1); // 下沉操作，恢复堆的性质
        Some(self.items.pop().unwrap()) // 返回堆顶元素
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

    fn next(&mut self) -> Option<T> {
        self.pop()
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
}