mod trie;


fn main() {
    let mut root = trie::Trie::new();
    root.insert("hello".to_string());
    root.insert("hi".to_string());
    root.insert("hello, world!".to_string());
    root.insert("cat".to_string());
    root.insert("cattle".to_string());
    root.insert("to be".to_string());
    root.insert("to be or not to be".to_string());

    root.print();
}
