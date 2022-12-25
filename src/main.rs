//!       Rust Christmas Tree
//!                *
//!               ***
//!              *****
//!             *******
//!            *********
//!           ***********
//!          *************
//!         ***************
//!        *****************
//!       *******************
//!      *********************
//!     ***********************
//!    *************************
//!   ***************************
//!  *****************************
//! *******************************
//!               ||
//!               ||

fn main() {
    const CROWN_WIDTH: usize = 31;
    const CROWN_HEIGHT: usize = 16;
    const CROWN_CHAR: &str = "*";
    const CROWN_FACTOR: usize = (CROWN_WIDTH - 1) / 2;
    const SPACE_CHAR: &str = " ";
    const TRUNK_HEIGH: usize = 2;
    const TRUNK_CHAR: &str = "|";

    // Crown
    for n in 0..CROWN_HEIGHT {
        let crown = match n {
            0 => {
                let padding_left = &SPACE_CHAR.repeat(CROWN_FACTOR);
                [padding_left, CROWN_CHAR].concat()
            }
            _ => {
                let start = CROWN_FACTOR - n;
                let padding_left = SPACE_CHAR.repeat(start);
                let end = CROWN_WIDTH - (start * 2);
                let points = CROWN_CHAR.repeat(end);
                [padding_left, points].concat()
            }
        };

        println!("{}", crown);
    }

    // Trunk
    for _ in 0..TRUNK_HEIGH {
        let padding_left = SPACE_CHAR.repeat(CROWN_FACTOR - 1);
        let trunk = TRUNK_CHAR.repeat(2);
        println!("{}{}", padding_left, trunk);
    }
}
