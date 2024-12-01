use vec_map::vec_map;

extern crate vec_map;

fn main() {
    let mut my_map = vec_map![1 => "foo", 42 => "bar"];
    my_map.insert(3, "baz");
    assert_eq!(my_map.get(1).copied(), Some("foo"));
    assert_eq!(my_map.get(2).copied(), None);
    assert_eq!(my_map.get(3).copied(), Some("baz"));
    assert_eq!(my_map.get(42).copied(), Some("bar"));
}
