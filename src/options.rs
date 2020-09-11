macro_rules! name_of {
    ($name:ident in $ty:ty) => {{
        #[allow(dead_code)]
        fn dummy(v: $ty)
        {
            let _ = &v.$name;
        }
        stringify!($name)
    }};

    ($name:ident) => {{
        let _ = &$name;
        stringify!($name)
    }};
}

#[repr(C)]
pub struct StarMapOptions
{
    pub seed: u64,
    pub object_count: u32,
    pub centre_distribution: f32,
    pub height: f32,
    pub height_distribution: f32,
    pub depth: f32,
    pub depth_distribution: f32,
    pub core_size: f32,
    pub swirl_magnitude: f32,
    pub val4_0_max: u8,
    pub val4_1_max: u8,
    pub val4_2_max: u8,
    pub val4_3_max: u8,
}

impl StarMapOptions
{
    pub fn defaults() -> StarMapOptions
    {
        StarMapOptions {
            seed: 1337,
            object_count: 10000,
            centre_distribution: 0.6,
            height: 1f32,
            height_distribution: 0.75,
            depth: 1f32,
            depth_distribution: 0.75,
            core_size: 0.1,
            swirl_magnitude: 0.75,
            val4_0_max: 255,
            val4_1_max: 255,
            val4_2_max: 255,
            val4_3_max: 255,
        }
    }

    pub fn validate(&self) -> Result<(), String>
    {
        let mut err_message = String::new();

        add_error_if_not_zero_to_one(
            self.centre_distribution,
            &mut err_message,
            name_of!(centre_distribution in StarMapOptions),
        );

        add_error_if_not_zero_to_one(
            self.height,
            &mut err_message,
            name_of!(height in StarMapOptions),
        );

        add_error_if_not_zero_to_one(
            self.height_distribution,
            &mut err_message,
            name_of!(height_distribution in StarMapOptions),
        );

        add_error_if_not_zero_to_one(
            self.depth,
            &mut err_message,
            name_of!(depth in StarMapOptions),
        );

        add_error_if_not_zero_to_one(
            self.depth_distribution,
            &mut err_message,
            name_of!(depth_distribution in StarMapOptions),
        );

        add_error_if_not_zero_to_one(
            self.core_size,
            &mut err_message,
            name_of!(core_size in StarMapOptions),
        );

        add_error_if_not_zero_to_one(
            self.swirl_magnitude,
            &mut err_message,
            name_of!(swirl_magnitude in StarMapOptions),
        );
        if err_message.len() > 0 {
            Err(String::from(err_message.trim()))
        } else {
            Ok(())
        }
    }
}

fn add_error_if_not_zero_to_one(f: f32, err_message: &mut String, var_name: &str)
{
    if f < 0f32 || f > 1f32 {
        err_message.push_str(&format!(
            "{} must be between 0 and 1 (inclusive).\n",
            var_name
        ));
    }
}
