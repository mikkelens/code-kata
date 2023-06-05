trait PushSort<T> {
    fn push_sort(&mut self, val: T);
}
impl<T : Ord> PushSort<T> for Vec<T> {
    fn push_sort(&mut self, val: T) {
        self.push(val);
        self.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rack: Vec<u32> = vec![];
        let empty: Vec<u32> = vec![];
        assert_eq!(empty, rack);
        
        rack.push_sort(20);
        assert_eq!(vec![20], rack);

        rack.push_sort(10);
        assert_eq!(vec![10, 20], rack);

        rack.push_sort(30);
        assert_eq!(vec![10, 20, 30], rack);
    }
}