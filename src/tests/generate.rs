
#[cfg(test)]
mod generate_tests {
    use crate::options::StarMapOptions;
    use crate::generate::{generate_into_slice};
    use crate::generate;
    use crate::StarMapEntry;

    extern crate test;
    use test::Bencher;

    //use std::f32::consts::FRAC_PI_2;
    //use std::f32::consts::PI;

    #[bench]
    pub fn bench_generate1(b: &mut Bencher) {
        let mut x: Vec<StarMapEntry> = vec![
            StarMapEntry {
                x: 0f32,
                y: 0f32,
                z: 0f32,
                w: 0f32,
            };
            10000
        ];

        b.iter(|| {
            let mut defaults = StarMapOptions::defaults();
            defaults.object_count = 10000;
            generate_into_slice(&mut x, defaults);
        });
    }

    #[bench]
    pub fn bench_generate2(b: &mut Bencher) {
        let mut x: Vec<StarMapEntry> = vec![
            StarMapEntry {
                x: 0f32,
                y: 0f32,
                z: 0f32,
                w: 0f32,
            };
            10000
        ];

        b.iter(|| {
            let mut defaults = StarMapOptions::defaults();
            defaults.object_count = 10000;
            generate_into_slice(&mut x, defaults);
        });
    }

    #[bench]
    pub fn bench_generate3(b: &mut Bencher) {
        let mut x: Vec<StarMapEntry> = vec![
            StarMapEntry {
                x: 0f32,
                y: 0f32,
                z: 0f32,
                w: 0f32,
            };
            10000
        ];

        b.iter(|| {
            let mut defaults = StarMapOptions::defaults();
            defaults.object_count = 10000;
            generate_into_slice(&mut x, defaults);
        });
    }



    #[test]
    pub fn generate_should_return_vec_of_length_object_count()
    {
        const TEST_LEN: u32 = 300;
        let mut options = StarMapOptions::defaults();
        options.object_count = TEST_LEN;
        let x = generate(options);
        assert_eq!(TEST_LEN, x.unwrap().len() as u32);
    }

}