#[cfg(test)]
mod tests {
    use crate::LruCache;

    #[test]
    fn test_new() {
        let cache: LruCache<i32, i32> = LruCache::new(3);
        assert_eq!(cache.capacity, 3);
    }

    #[test]
    fn test_put() {
        let mut cache: LruCache<i32, i32> = LruCache::new(3);
        cache.put(1, 10);
        assert_eq!(cache.map.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut cache: LruCache<i32, i32> = LruCache::new(3);
        cache.put(1, 10);
        let value = cache.get(&1);
        assert_eq!(value, Some(&10));
    }

    #[test]
    fn test_overflow() {
        let mut cache: LruCache<i32, i32> = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert_eq!(cache.map.len(), 2);
        assert_eq!(cache.get(&1), None);
    }
}
