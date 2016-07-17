use modulo_traits::text::{Point, SelectionRange};
use ::file::file_data::FileData;

#[test]
fn test_new_file_single_new_line_no_selections() {
    let file_data = FileData::new();
    assert!(file_data.selections.is_empty());
    assert_eq!(file_data.lines.len(), 1);
    assert_eq!(file_data.lines[0].text, String::new());
}

#[test]
fn test_load_file_from_string() {
    let mut file_data = FileData::new();
    file_data.load_from_string("test\ntest\ntest");
    assert_eq!(file_data.lines.len(), 3);
}

#[test]
fn test_clear_file_single_new_line_no_selections() {
    let mut file_data = FileData::new();
    file_data.load_from_string("test\ntest\ntest");
    file_data.clear_all_text();
    assert!(file_data.selections.is_empty());
    assert_eq!(file_data.lines.len(), 1);
    assert_eq!(file_data.lines[0].text, String::new());
}

#[test]
fn test_insert_text() {
    let mut file_data = FileData::new();
    file_data.add_selection(SelectionRange::new(Point::new(0, 0), None));
    file_data.replace_text("hi");
    assert_eq!(file_data.lines.len(), 1);
    assert_eq!(file_data.lines[0].text, String::from("hi"));
}

#[test]
fn test_replace_text() {
    let mut file_data = FileData::new();
    file_data.add_selection(SelectionRange::new(Point::new(0, 0), None));
    file_data.replace_text("hi\nhi");
    assert_eq!(file_data.lines.len(), 2);
    assert_eq!(file_data.selections.len(), 1);
    assert_eq!(file_data.lines[0].text, String::from("hi"));
    assert_eq!(file_data.lines[1].text, String::from("hi"));
    assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(1, 2), None));
    file_data.clear_all_selections();

    file_data.add_selection(SelectionRange::new(Point::new(0, 0), Some(Point::new(0, 2))));
    file_data.replace_text("no");
    assert_eq!(file_data.lines.len(), 2);
    assert_eq!(file_data.selections.len(), 1);
    assert_eq!(file_data.lines[0].text, String::from("no"));
    assert_eq!(file_data.lines[1].text, String::from("hi"));
    assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(0, 2), None));
    file_data.clear_all_selections();

    file_data.add_selection(SelectionRange::new(Point::new(0, 0), Some(Point::new(0,2))));
    file_data.add_selection(SelectionRange::new(Point::new(1, 0), Some(Point::new(1,2))));
    file_data.replace_text("so");
    assert_eq!(file_data.lines.len(), 2);
    assert_eq!(file_data.selections.len(), 2);
    assert_eq!(file_data.lines[0].text, String::from("so"));
    assert_eq!(file_data.lines[1].text, String::from("so"));
    assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(0, 2), None));
    assert_eq!(file_data.selections[1], SelectionRange::new(Point::new(1, 2), None));
    file_data.clear_all_selections();

    file_data.clear_all_text();
    assert!(file_data.selections.is_empty());
    assert_eq!(file_data.lines.len(), 1);
    assert_eq!(file_data.lines[0].text, String::new());
    file_data.load_from_string("no\nyes\nnot");
    file_data.add_selection(SelectionRange::new(Point::new(0, 1), Some(Point::new(0,2))));
    file_data.add_selection(SelectionRange::new(Point::new(2, 1), Some(Point::new(2,2))));
    file_data.replace_text("j");
    assert_eq!(file_data.lines.len(), 3);
    assert_eq!(file_data.selections.len(), 2);
    assert_eq!(file_data.lines[0].text, String::from("nj"));
    assert_eq!(file_data.lines[1].text, String::from("yes"));
    assert_eq!(file_data.lines[2].text, String::from("njt"));
    assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(0, 2), None));
    assert_eq!(file_data.selections[1], SelectionRange::new(Point::new(2, 2), None));
    file_data.clear_all_selections();

    file_data.add_selection(SelectionRange::new(Point::new(0, 0), Some(Point::new(1,3))));
    file_data.add_selection(SelectionRange::new(Point::new(2, 0), Some(Point::new(2,3))));
    file_data.replace_text("hi");
    assert_eq!(file_data.lines.len(), 2);
    assert_eq!(file_data.selections.len(), 2);
    assert_eq!(file_data.lines[0].text, String::from("hi"));
    assert_eq!(file_data.lines[1].text, String::from("hi"));
    assert_eq!(file_data.selections[0], SelectionRange::new(Point::new(0, 2), None));
    assert_eq!(file_data.selections[1], SelectionRange::new(Point::new(1, 2), None));
    file_data.clear_all_selections();
}
