use dotbc::{self, DotBC};
use tempdir::TempDir;

#[test]
fn test_archive_round_trip() {
     let dir = TempDir::new("dotbc-test").expect("create temp dir");
     let file = dir.path().join("foo.bc");

     let mut dotbc = DotBC::new();
     dotbc.put("hello.txt", &b"hello world"[..]);

     assert_eq!(dotbc.get("hello.txt").unwrap(), b"hello world");

     dotbc.save(&file);

     let dotbc = DotBC::open(&file).unwrap();
     assert_eq!(dotbc.get("hello.txt").unwrap(), b"hello world");
}
