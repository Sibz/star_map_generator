mod options;

use crate::options::StarMapOptions;

#[test]
pub fn generate_should_return_vec_of_length_object_count() {
    const TEST_LEN: u32 = 3;
    let mut options = StarMapOptions::defaults();
    options.object_count = TEST_LEN;
    let x = super::generate(options);
    assert_eq!(TEST_LEN, x.unwrap().len() as u32);
}
