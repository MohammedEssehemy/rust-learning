use std::cmp::Ordering;

pub fn find<E: Ord, T: AsRef<[E]>>(array: T, key: E) -> Option<usize> {
    let slice = array.as_ref();
    let mut start = 0;
    let mut end = slice.len().checked_sub(1)?;
    let mut index = None;
    while end >= start {
        let middle_index = (start + end) / 2;
        let middle_element = slice.get(middle_index)?;
        match key.cmp(middle_element) {
            Ordering::Equal => {
                index = Some(middle_index);
                break;
            }
            Ordering::Greater => {
                start = middle_index.checked_add(1)?;
            }
            Ordering::Less => {
                end = middle_index.checked_sub(1)?;
            }
        }
    }
    index
}

pub fn find_recursive<E: Ord, T: AsRef<[E]>>(array: T, key: E) -> Option<usize> {
    find_tail(array.as_ref(), key, 0)
}

fn find_tail<E: Ord>(array: &[E], key: E, offset: usize) -> Option<usize> {
    let mid = array.len() / 2;
    // if array is empty, array[..0].get(_) -> None
    // calculate offset and pass to next recursive call
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid + offset),
        Ordering::Less => find_tail(&array[..mid], key, offset),
        Ordering::Greater => find_tail(&array[mid + 1..], key, offset + mid + 1),
    }
}
