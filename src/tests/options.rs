use crate::options::StarMapOptions;

const JUST_OVER_ONE: f32 = 1f32 + std::f32::EPSILON;
const JUST_UNDER_ZERO: f32 = -std::f32::EPSILON;
const ONE: f32 = 1f32;
const ZERO: f32 = 0f32;

#[test]
pub fn validate_with_defaults_should_not_err() {
    assert_eq!(Ok(()), StarMapOptions::defaults().validate());
}

// Height
#[test]
pub fn validate_when_height_greater_than_one_should_err() {
    let mut options = StarMapOptions::defaults();
    options.height = JUST_OVER_ONE;
    assert_ne!(Ok(()), options.validate());
}

#[test]
pub fn validate_when_height_less_than_zero_should_err() {
    let mut options = StarMapOptions::defaults();
    options.height = JUST_UNDER_ZERO;
    assert_ne!(Ok(()), options.validate());
}

#[test]
pub fn validate_when_height_in_range_should_not_err() {
    let mut options = StarMapOptions::defaults();
    options.height = ZERO;
    assert_eq!(Ok(()), options.validate());
    options.height = ONE;
    assert_eq!(Ok(()), options.validate());
}

// Depth
#[test]
pub fn validate_when_depth_greater_than_one_should_err() {
    let mut options = StarMapOptions::defaults();
    options.depth = JUST_OVER_ONE;
    assert_ne!(Ok(()), options.validate());
}

#[test]
pub fn validate_when_depth_less_than_zero_should_err() {
    let mut options = StarMapOptions::defaults();
    options.depth = JUST_UNDER_ZERO;
    assert_ne!(Ok(()), options.validate());
}

#[test]
pub fn validate_when_depth_in_range_should_not_err() {
    let mut options = StarMapOptions::defaults();
    options.depth = ZERO;
    assert_eq!(Ok(()), options.validate());
    options.depth = ONE;
    assert_eq!(Ok(()), options.validate());
}

// Core size
#[test]
pub fn validate_when_core_size_greater_than_one_should_err() {
    let mut options = StarMapOptions::defaults();
    options.core_size = JUST_OVER_ONE;
    assert_ne!(Ok(()), options.validate());
}

#[test]
pub fn validate_when_core_size_less_than_zero_should_err() {
    let mut options = StarMapOptions::defaults();
    options.core_size = JUST_UNDER_ZERO;
    assert_ne!(Ok(()), options.validate());
}

#[test]
pub fn validate_when_core_size_in_range_should_not_err() {
    let mut options = StarMapOptions::defaults();
    options.core_size = ZERO;
    assert_eq!(Ok(()), options.validate());
    options.core_size = ONE;
    assert_eq!(Ok(()), options.validate());
}

// Swirl Magnitude
#[test]
pub fn validate_when_swirl_magnitude_greater_than_one_should_err() {
    let mut options = StarMapOptions::defaults();
    options.swirl_magnitude = JUST_OVER_ONE;
    assert_ne!(Ok(()), options.validate());
}

#[test]
pub fn validate_when_swirl_magnitude_less_than_zero_should_err() {
    let mut options = StarMapOptions::defaults();
    options.swirl_magnitude = JUST_UNDER_ZERO;
    assert_ne!(Ok(()), options.validate());
}

#[test]
pub fn validate_when_swirl_magnitude_in_range_should_not_err() {
    let mut options = StarMapOptions::defaults();
    options.swirl_magnitude = ZERO;
    assert_eq!(Ok(()), options.validate());
    options.swirl_magnitude = ONE;
    assert_eq!(Ok(()), options.validate());
}

// Spread
#[test]
pub fn validate_when_spread_greater_than_one_should_err() {
    let mut options = StarMapOptions::defaults();
    options.spread = JUST_OVER_ONE;
    assert_ne!(Ok(()), options.validate())
}

#[test]
pub fn validate_when_spread_less_than_zero_should_err() {
    let mut options = StarMapOptions::defaults();
    options.spread = JUST_UNDER_ZERO;
    assert_ne!(Ok(()), options.validate())
}

#[test]
pub fn validate_when_spread_in_range_should_not_err() {
    let mut options = StarMapOptions::defaults();
    options.spread = ZERO;
    assert_eq!(Ok(()), options.validate());
    options.spread = ONE;
    assert_eq!(Ok(()), options.validate());
}
