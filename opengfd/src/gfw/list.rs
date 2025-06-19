#![allow(dead_code)]
use allocator_api2::alloc::{ Allocator, Global };
use std::{
    alloc::Layout,
    fmt::{ Debug, Display },
    marker::PhantomData,
    ops::{ Index, IndexMut },
    ptr::NonNull
};

#[repr(C)]
pub struct ListItem<T, A = Global>
where A: Allocator + Clone
{
    next: Option<NonNull<Self>>,
    prev: Option<NonNull<Self>>,
    data: T,
    _allocator: A
}

/// A double ended linked list for storing elements. Most of this API was adapted from the C++ list
/// implementation in https://github.com/rirurin/cpp-types/blob/main/src/msvc/list.rs
impl<T, A> ListItem<T, A>
where A: Allocator + Clone
{
    fn new(data: T, alloc: A) -> NonNull<Self> {
        let new = alloc.allocate(Layout::new::<Self>()).unwrap().as_ptr() as *mut Self;
        let new_edit: &mut Self = unsafe { &mut *new };
        new_edit.next = None;
        new_edit.prev = None;
        unsafe { std::ptr::write(&raw mut new_edit.data, data); }
        new_edit._allocator = alloc;
        unsafe { NonNull::new_unchecked(new) }
    }
    pub fn get_next(&self) -> Option<&Self> {
        unsafe { self.next.map(|f| f.as_ref()) }   
    }
    pub fn get_prev(&self) -> Option<&Self> {
        unsafe { self.prev.map(|f| f.as_ref()) }   
    }
    pub fn get_next_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.next.map(|mut f| f.as_mut()) }
    }
    pub fn get_prev_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.prev.map(|mut f| f.as_mut()) }
    }
    pub fn get_next_ptr(&mut self) -> Option<NonNull<Self>> { self.next }
    pub fn get_prev_ptr(&mut self) -> Option<NonNull<Self>> { self.prev }
    pub fn value(&self) -> &T { &self.data }
    pub fn value_mut(&mut self) -> &mut T { &mut self.data }
}

#[repr(C)]
pub struct List<T, A = Global>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    head: Option<NonNull<ListItem<T, A>>>,
    tail: Option<NonNull<ListItem<T, A>>>,
    count: u32,
    _allocator: A
}

impl<T> List<T, Global> {
    pub fn new() -> Self { Self::new_in(Global) }
    pub fn new_with_vtable(vtable: *const u8) -> Self { Self::new_in_with_vtable(vtable, Global) }
    pub fn from_vec(vec: Vec<T>) -> Self { Self::from_vec_in(vec, Global) }
    pub fn from_vec_and_vtable(vtable: *const u8, vec: Vec<T>) -> Self { Self::from_vec_in_and_vtable(vtable, vec, Global) }
}

impl<T, A> List<T, A>
where A: Allocator + Clone
{
    pub fn new_in(alloc: A) -> Self {
        Self::new_in_with_vtable(std::ptr::null(), alloc)
    }
    pub fn new_in_with_vtable(vtable: *const u8, alloc: A) -> Self {
        assert!(std::mem::size_of::<A>() == 0, "Allocator must be zero-sized!");
        Self {
            _cpp_vtable: vtable,
            head: None,
            tail: None,
            count: 0,
            _allocator: alloc
        }
    }

    pub fn len(&self) -> usize { self.count as usize }

    pub fn first(&self) -> Option<&ListItem<T, A>> {
        unsafe { self.head.map(|f| f.as_ref()) }
    }
    pub fn last(&self) -> Option<&ListItem<T, A>> {
        unsafe { self.tail.map(|f| f.as_ref()) }
    }
    pub fn first_mut(&mut self) -> Option<&mut ListItem<T, A>> {
        unsafe { self.head.map(|mut f| f.as_mut()) }
    }
    pub fn last_mut(&mut self) -> Option<&mut ListItem<T, A>> {
        unsafe { self.tail.map(|mut f| f.as_mut()) }
    }

    pub fn get(&self, index: usize) -> Option<&ListItem<T, A>> {
        if index >= self.len() { return None; } 
        Some(self.get_unchecked(index))
    }

    pub fn get_unchecked(&self, index: usize) -> &ListItem<T, A> {
        match index <= (self.len() - 1) / 2 {
            true => self.get_from_start(index),
            false => self.get_from_end(self.len() - 1 - index)
        }
    }

    fn get_from_start(&self, index: usize) -> &ListItem<T, A> {
        let mut curr = self.first().unwrap();
        for _ in 0..index { curr = curr.get_next().unwrap(); }
        curr
    }

    fn get_from_end(&self, from_end: usize) -> &ListItem<T, A> {
        let mut curr = self.last().unwrap();
        for _ in 0..from_end { curr = curr.get_prev().unwrap(); }
        curr
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut ListItem<T, A>> {
        if index >= self.len() { return None; }
        Some(self.get_unchecked_mut(index))
    }

    pub fn get_unchecked_mut(&mut self, index: usize) -> &mut ListItem<T, A> {
        match index <= (self.len() - 1) / 2 {
            true => self.get_from_start_mut(index),
            false => self.get_from_end_mut(self.len() - 1 - index)
        }
    }

    fn get_from_start_mut(&mut self, index: usize) -> &mut ListItem<T, A> {
        let mut curr = self.first_mut().unwrap();
        for _ in 0..index { curr = curr.get_next_mut().unwrap(); }
        curr
    }

    fn get_from_end_mut(&mut self, from_end: usize) -> &mut ListItem<T, A> {
        let mut curr = self.last_mut().unwrap();
        for _ in 0..from_end { curr = curr.get_prev_mut().unwrap(); }
        curr
    }

    pub fn iter(&self) -> ListIterator<'_, T, A> { self.into_iter() }
    pub fn iter_mut(&mut self) -> ListIteratorMut<'_, T, A> { self.into_iter() }

    pub fn from_vec_in(vec: Vec<T>, alloc: A) -> Self {
        Self::from_vec_in_and_vtable(std::ptr::null(), vec, alloc)
    }

    pub fn from_vec_in_and_vtable(vtable: *const u8, vec: Vec<T>, alloc: A) -> Self {
        assert!(std::mem::size_of::<A>() == 0, "Allocator must be zero-sized!");
        let mut new = List::new_in_with_vtable(vtable, alloc);
        for el in vec { new.push(el) }
        new
    }

    pub fn index_of_by_predicate<F>(&self, cb: F) -> Option<usize>
    where F: Fn(&T) -> bool
    {
        for (i, v) in self.iter().enumerate() {
            if cb(v) { return Some(i)}
        }
        None
    }

    pub fn contains_by_predicate<F>(&self, cb: F) -> bool
    where F: Fn(&T) -> bool { self.find_by_predicate(cb).is_some() }

    pub fn find_by_predicate<F>(&self, cb: F) -> Option<&T>
    where F: Fn(&T) -> bool
    {
        for v in self { if cb(v) { return Some(v) } }
        None
    }

    /// Inserts a new element after the given index. This method will panic if trying to insert out
    /// of bounds.
    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len(), "Tried to insert value out of bounds");
        if index == self.len() { self.add_tail(value); }
        else {
            let after = unsafe { &mut *(&raw const *self.get(index).unwrap() as *mut ListItem<T, A>) };
            unsafe { self.insert_after_unchecked(after, value); }
        }
    }

    pub(super) unsafe fn insert_after_unchecked(&mut self, prev: &mut ListItem<T, A>, val: T) {
        let allocator = self._allocator.clone();
        let mut new = ListItem::new(val, allocator);
        let new_mut = unsafe { new.as_mut() };
        // link
        let next = prev.next;
        prev.next = Some(new);
        new_mut.prev = Some(unsafe{NonNull::new_unchecked(&raw mut *prev)});
        new_mut.next = next;
        if let Some(mut n) = next { unsafe { n.as_mut() }.prev = Some(new); }
        self.count += 1;
    }

    pub fn push(&mut self, value: T) { self.add_tail(value) }
    pub fn pop(&mut self) -> Option<T> { if self.len() > 0 { Some(self.remove(self.len() - 1)) } else { None } }

    /// Inserts a new element at the beginning
    pub fn add_tail(&mut self, value: T) {
        let allocator = self._allocator.clone();
        let mut new = ListItem::new(value, allocator);
        let new_mut = unsafe { new.as_mut() };
        // link
        match self.last_mut() {
            Some(v) => {
                v.next = Some(new);
                new_mut.prev = Some(unsafe{NonNull::new_unchecked(&raw mut *v)});
                self.tail = Some(new);
            },
            None => {
                self.head = Some(new);
                self.tail = Some(new);
            }
        }
        self.count += 1;
    }
    /// Inserts a new element at the end
    pub fn add_head(&mut self, value: T) {
        let allocator = self._allocator.clone();
        let mut new = ListItem::new(value, allocator);
        let new_mut = unsafe { new.as_mut() };
        // link
        match self.first_mut() {
            Some(v) => {
                v.prev = Some(new);
                new_mut.next = Some(unsafe{NonNull::new_unchecked(&raw mut *v)});
                self.head = Some(new);
            },
            None => {
                self.head = Some(new);
                self.tail = Some(new);
            }
        }
        self.count += 1;
    }
    /// Removes an element at the given index
    pub fn remove(&mut self, index: usize) -> T {
        assert!(self.len() > index, "Tried to remove value out of bounds");
        let el = self.get_unchecked_mut(index);
        let el_ptr = &raw mut *el;
        unsafe { self.remove_unchecked(el_ptr) }
    }

    unsafe fn unlink(&mut self, p_element: *mut ListItem<T, A>) {
        let element = &mut* p_element;
        let prev = element.get_prev_mut().map(|v| NonNull::new_unchecked(&raw mut *v));
        let next = element.get_next_mut().map(|v| NonNull::new_unchecked(&raw mut *v));
        match prev {
            Some(mut v) => {
                unsafe { v.as_mut() }.next = next;
                match next {
                    Some(mut n) => unsafe { n.as_mut() }.prev = prev,
                    None => self.tail = prev
                }
            },
            None => { // head entry
                self.head = next;
                match next {
                    Some(mut v) => unsafe { v.as_mut() }.prev = None,
                    None => self.tail = None
                }
            }
        }
    }

    pub(super) unsafe fn remove_unchecked(&mut self, p_element: *mut ListItem<T, A>) -> T {
        self.count -= 1;
        self.unlink(p_element);
        let element = &mut *p_element;
        let val_out = std::ptr::read(element.value());
        std::ptr::drop_in_place(element);
        val_out
    }
}

impl<T, A> List<T, A>
where T: PartialEq,
      A: Allocator + Clone
{
    pub fn index_of(&self, val: T) -> Option<usize> {
        for (i, v) in self.iter().enumerate() {
            if *v == val { return Some(i)}
        }
        None
    }

    pub fn find(&self, val: T) -> Option<&T> {
        for v in self {
            if *v == val { return Some(v)}
        }
        None
    }

    pub fn find_mut(&mut self, val: T) -> Option<&mut T> {
        for v in self {
            if *v == val { return Some(v)}
        }
        None
    }

    pub fn contains(&self, val: T) -> bool { self.find(val).is_some() }
}

impl<T, A> Index<usize> for List<T, A>
where A: Allocator + Clone
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get_unchecked(index).value()
    }
}

impl<T, A> IndexMut<usize> for List<T, A>
where A: Allocator + Clone
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_unchecked_mut(index).value_mut()
    }
}

impl<'a, T, A> IntoIterator for &'a List<T, A>
where A: Allocator + Clone
{
    type Item = &'a T;
    type IntoIter = ListIterator<'a, T, A>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            curr: self.first(),
            curr_rev: self.last(),
            _type_marker: PhantomData::<T>,
            _alloc_marker: PhantomData::<A>
        }
    }
}

impl<'a, T, A> IntoIterator for &'a mut List<T, A>
where A: Allocator + Clone
{
    type Item = &'a mut T;
    type IntoIter = ListIteratorMut<'a, T, A>;
    fn into_iter(self) -> Self::IntoIter {
        // Rust isn't aware that we can safely split borrows here, since implementors of
        // DoubleEndedIterator can't allow the forward and back iterators to cross over
        let curr = self.first().map(|v| unsafe { &mut *(&raw const *v as *mut ListItem<T, A>) });
        let curr_rev = self.last_mut();
        Self::IntoIter {
            curr, curr_rev,
            _type_marker: PhantomData::<T>,
            _alloc_marker: PhantomData::<A>
        }
    }
}

pub struct ListIterator<'a, T, A>
where A: Allocator + Clone
{
    curr: Option<&'a ListItem<T, A>>,
    curr_rev: Option<&'a ListItem<T, A>>,
    _type_marker: std::marker::PhantomData<T>,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, T: 'a, A> ListIterator<'a, T, A>
where A: Allocator + Clone
{
    fn collided(&self) -> bool {
        let fwd_ptr = match &self.curr { Some(v) => &raw const **v, None => std::ptr::null() as *const ListItem<T, A> };
        let bck_ptr = match &self.curr_rev { Some(v) => &raw const **v, None => std::ptr::null() as *const ListItem<T, A> };
        std::ptr::eq(fwd_ptr, bck_ptr)
    }
}

impl<'a, T: 'a, A> Iterator for ListIterator<'a, T, A>
where A: Allocator + Clone
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = match self.collided() {
                false => v.get_next(),
                true => None
            };
            v.value()
        })
    }
}

impl<'a, T: 'a, A> DoubleEndedIterator for ListIterator<'a, T, A>
where A: Allocator + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.curr_rev.take().map(|v| {
            self.curr_rev = match self.collided() {
                false => v.get_prev(),
                true => None
            };
            v.value()
        })
    }
}

pub struct ListIteratorMut<'a, T, A>
where A: Allocator + Clone
{
    curr: Option<&'a mut ListItem<T, A>>,
    curr_rev: Option<&'a mut ListItem<T, A>>,
    _type_marker: std::marker::PhantomData<T>,
    _alloc_marker: std::marker::PhantomData<A>
}

impl<'a, T: 'a, A> ListIteratorMut<'a, T, A>
where A: Allocator + Clone
{
    fn collided(&self) -> bool {
        let fwd_ptr = match &self.curr { Some(v) => &raw const **v, None => std::ptr::null() as *const ListItem<T, A> };
        let bck_ptr = match &self.curr_rev { Some(v) => &raw const **v, None => std::ptr::null() as *const ListItem<T, A> };
        std::ptr::eq(fwd_ptr, bck_ptr)
    }
}

impl<'a, T: 'a, A> Iterator for ListIteratorMut<'a, T, A>
where A: Allocator + Clone
{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = match self.collided() {
                false => unsafe { v.get_next_ptr().map(|mut f| f.as_mut()) },
                true => None
            };
            v.value_mut()
        })
    }
}

impl<'a, T: 'a, A> DoubleEndedIterator for ListIteratorMut<'a, T, A>
where A: Allocator + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.curr_rev.take().map(|v| {
            self.curr_rev = match self.collided() {
                false => unsafe { v.get_next_ptr().map(|mut f| f.as_mut()) }
                true => None
            };
            v.value_mut()
        })
    }
}

impl<T, A> Display for List<T, A>
where T: Display,
      A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::from("List [ ");
        for (i, v) in self.iter().enumerate() {
            buf.push_str(&format!("{}", v));
            if i < self.len() - 1 { buf.push_str(", ") }
        }
        buf.push_str(" ]");
        write!(f, "{}", &buf)
    }
}

impl<T, A> Debug for List<T, A>
where T: Debug,
      A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::from("List [ ");
        for (i, v) in self.iter().enumerate() {
            buf.push_str(&format!("{:?}", v));
            if i < self.len() - 1 { buf.push_str(", ") }
        }
        buf.push_str(" ]");
        write!(f, "{}", &buf)
    }
}


impl<T, A> Drop for List<T, A>
where A: Allocator + Clone
{
    fn drop(&mut self) {
        let mut curr_node = self.first();
        while let Some(v) = curr_node {
            curr_node = v.get_next(); 
            unsafe {
                std::ptr::drop_in_place(&raw const *v.value() as *mut T);
                std::ptr::drop_in_place(&raw const *v as *mut ListItem<T, A>) 
            };
        }
    }
}

impl<T, A> From<List<T, A>> for Vec<T>
where A: Allocator + Clone,
{
    fn from(mut value: List<T, A>) -> Self {
        let old_len = value.len();
        let mut vec: Vec<T> = Vec::with_capacity(old_len);
        unsafe { vec.set_len(old_len); }
        for i in 0..old_len {
            let val = value.pop().unwrap();
            unsafe { vec.as_mut_ptr().add(old_len-i-1).write(val); }
        }        
        vec
    }
}

#[cfg(test)]
pub mod tests {
    use allocator_api2::alloc::{ Allocator, Global };
    use super::List;

    use std::{
        fmt::{ Debug, Display },
        error::Error
    };
    type TestReturn = Result<(), Box<dyn Error>>;

impl<T, A> List<T, A>
where T: Display + PartialEq,
      A: Allocator + Clone
{
    pub(crate) fn check_list_iterator(&self, values: &[T]) {
        for (i, v) in self.iter().enumerate() {
            assert!(values[i] == *v, 
                "Index {} should have item {} instead of {}", 
                i, values[i], *v);
        }
    }

    pub(crate) fn check_list_iterator_reverse(&self, values: &[T]) {
        for (i, v) in self.iter().rev().enumerate() {
            assert!(values[i] == *v, 
                "Index {} should have item {} instead of {}",
                i, values[i], *v);
        }
    }
}

impl<T, A> List<T, A>
where T: Debug + PartialEq,
      A: Allocator + Clone
{
    pub(crate) fn check_list_iterator_debug(&self, values: &[T]) {
        for (i, v) in self.iter().enumerate() {
            assert!(values[i] == *v, 
                "Index {} should have item {:?} instead of {:?}", 
                i, values[i], *v);
        }
    }

    pub(crate) fn check_list_iterator_reverse_debug(&self, values: &[T]) {
        for (i, v) in self.iter().rev().enumerate() {
            assert!(values[i] == *v, 
                "Index {} should have item {:?} instead of {:?}",
                i, values[i], *v);
        }
    }
}

impl<T, A> List<T, A>
where T: Debug,
      A: Allocator + Clone
{
    pub(crate) fn check_list_iterator_delegate<F, V>(&self, cb: F, expected: &[V])
    where F: Fn(&T, &V) -> bool,
          V: Debug
    {
        for (i, v) in self.iter().enumerate() {
            assert!(cb(v, &expected[i]), "Index {} should contain element {:?} instead of {:?}",
            i, expected[i], *v);
        }
    }
}
    #[test]
    pub fn create_blank_list() -> TestReturn {
        let list: List<u32, Global> = List::new();
        assert!(list.len() == 0, "List should be blank");
        assert!(list.first().is_none(), "First element should be blank");
        Ok(())
    }
    #[test]
    pub fn list_push_pop() -> TestReturn {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        for i in 0..list.len() {
            // println!("{}: {}", i, list[i]);
            assert!(list[i] == i + 1, "Element at {} should have item {} instead of {}", i, i + 1, list[i]);
        }
        assert!(list.pop() == Some(3), "Element should be 3");
        assert!(list.pop() == Some(2), "Element should be 2");
        assert!(list.pop() == Some(1), "Element should be 1");
        assert!(list.pop() == None, "List should be empty");
        Ok(())
    }

    #[test]
    pub fn list_iterator() -> TestReturn {
        // Note that impl Iterator is only implemented for single ended iterators, since we don't
        // have a double ended linked list, so check backwards traversal on list_insertion
        let mut list = List::new();
        for i in 5..10 { list.push(i) }
        for (i, v) in list.iter().enumerate() {
            assert!(*v == i + 5, "Element {} should have item {} instead of {}", i, i + 5, *v);
        }
        for (i, v) in list.iter_mut().enumerate() {
            *v *= 2;
            assert!(*v == (i + 5) * 2, "Element {} should have item {} instead of {}", i, (i + 5) * 2, *v);
        }
        Ok(())
    }

    #[test]
    pub fn rust_list_conversion() -> TestReturn {
        let rust_list = vec!["a", "b", "c", "d", "e"];
        let list = List::from_vec(rust_list.clone());
        for (i, v) in list.iter().enumerate() {
            assert!(*v == rust_list[i], "Element {} should have {} instead of {}", i, rust_list[i], *v);
        }
        let list_out: Vec<&str> = list.into();
        for (a, b) in list_out.iter().zip(rust_list.iter()) {
            assert!(*a == *b, "Out list doesn't equal in list: {} != {}", *a, *b);
        }
        Ok(())
    }

    #[test]
    pub fn list_find() -> TestReturn { 
        let list = List::from_vec(vec![20, 30, 15, 5, 40, 25]);
        assert!(!list.contains(10), "List doesn't contain 10, but was found anyway");
        assert!(list.contains(30), "List contains 30, but wasn't found");
        assert!(list.index_of(40) == Some(4), "40 should be the fifth element");
        assert!(list.index_of(10) == None, "10 is not in the list");
        assert!(list.index_of_by_predicate(|f| f * 2 == 10) == Some(3), "Fourth element should be found (5)");
        assert!(*list.find_by_predicate(|f| f * 2 == 10).unwrap() == 5, "Should have found foruth element (5)");
        Ok(())
    }

    #[test]
    pub fn list_removal() -> TestReturn {
        let mut list = List::new();
        for i in 0..10 { list.push(i * 2) }
        assert!(list.len() == 10, "List length should be to, got {} instead", list.len());
        // delete at start
        assert!(list.remove(0) == 0, "The first element removed should be 0");
        // delete at end
        assert!(list.remove(8) == 18, "The last element removed should be 18");
        // delete inside
        assert!(list.remove(3) == 8, "The third element removed should be 8");
        assert!(list.len() == 7, "List length should be to, got {} instead", list.len());
        let expected = [2, 4, 6, 10, 12, 14, 16];
        for (i, v) in list.iter().enumerate() {
            assert!(*v == expected[i], "Element {} should be {} instead of {}", i, expected[i], *v);
        }
        // check backwards
        let expected_rev: Vec<i32> = expected.into_iter().rev().collect();
        for (i, v) in list.iter().rev().enumerate() {
            assert!(*v == expected_rev[i], "Element {} in reverse iterator should be {} instead of {}",
            i, expected_rev[i], *v);
        }
        Ok(())
    }
}
