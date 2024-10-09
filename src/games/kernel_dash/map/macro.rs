#[macro_export]
macro_rules! create_map {
    ($($($n:literal) +);+ $(;)?) => {
        create_map!{#columns; $($($n) +);+}

        #[allow(dead_code)]
        pub const MAP_HEIGHT: usize = create_map!(#lines; $($($n) +);+);

        #[allow(dead_code)]
        pub const MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = create_map!(#gen; $($($n) +);+);
    };

    (#columns; $($n:literal) +; $($_:tt)*) => { create_map!{#columns-gen; [0]; $($n)*} };
    (#columns-gen; [$count:expr]; ) => {
        #[allow(dead_code)]
        pub const MAP_WIDTH: usize = ($count / 8) + 1;

        #[allow(dead_code)]
        pub const MAP_DATA_WIDTH: usize = $count;
     };
    (#columns-gen; [$count:expr]; $_:literal $($n:tt)*) => { create_map!{#columns-gen; [$count + 1]; $($n)*} };

    (#lines; $($($n:literal) +);+) => { create_map!(#lines-gen; [1]; $($($n) +); +) };
    (#lines-gen; [$count:expr]; ) => { $count };
    (#lines-gen; [$count:expr]; ; $($n:tt)*) => { create_map!(#lines-gen; [$count + 1]; $($n)*) };
    (#lines-gen; [$count:expr]; $_:literal $($n:tt)*) => { create_map!(#lines-gen; [$count]; $($n)*) };

    (#gen; $($($n:literal) +);+) => {
        $crate::games::kernel_dash::map::r#macro::build_map::<MAP_WIDTH, MAP_HEIGHT, MAP_DATA_WIDTH>([$([$($n),+]),+])
    };
}

pub use create_map;

pub(super) const fn build_map<const W: usize, const H: usize, const D: usize>(
    data: [[u8; D]; H],
) -> [[u8; W]; H] {
    let mut out = [[0; W]; H];

    let mut y = 0;
    while y < H {
        let mut x = 0;
        while x < W {
            let x_idx = x * 8;
            let mut idx = 0;
            while idx < 8 {
                if idx + x_idx < D {
                    let bit = data[y][idx + x_idx];
                    out[y][x] += bit << (7 - idx);
                } else {
                    out[y][x] = 0;
                }

                idx += 1;
            }
            x += 1;
        }

        y += 1;
    }

    out
}
