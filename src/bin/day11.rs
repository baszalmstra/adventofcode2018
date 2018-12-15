fn power_level(x: i32, y: i32, grid_serial: i32) -> i32 {
    let rack_id = x + 10;
    let value = (rack_id * y + grid_serial) * rack_id;
    let digit = (value % 1000 - value % 100) / 100;
    digit - 5
}

fn compute_power_levels(width: i32, height: i32, grid_serial: i32) -> Vec<i32> {
    let mut power_levels = Vec::new();
    power_levels.resize((width * height) as usize, 0);
    for y in 0..height {
        for x in 0..width {
            power_levels[(y * 300 + x) as usize] = power_level(x, y, grid_serial);
        }
    }
    power_levels
}

fn compute_power_level_blocks(
    width: i32,
    height: i32,
    block_size: i32,
    power_level_grid: &Vec<i32>,
) -> Vec<i32> {
    let mut power_level_blocks = Vec::new();
    power_level_blocks.resize(
        ((width - block_size + 1) * (height - block_size + 1)) as usize,
        0,
    );

    for y in 0..height - (block_size - 1) {
        for x in 0..width - (block_size - 1) {
            let mut total = 0;
            for by in 0..block_size {
                for bx in 0..block_size {
                    total += power_level_grid[((y + by) * width + (x + bx)) as usize];
                }
            }
            power_level_blocks[(y * (width - block_size + 1) + x) as usize] = total;
        }
    }

    power_level_blocks
}

fn find_max_block(grid_serial: i32) -> (usize, usize) {
    let blocks =
        compute_power_level_blocks(300, 300, 3, &compute_power_levels(300, 300, grid_serial));
    let block_grid_size = 298;
    blocks
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(i, _)| (i % block_grid_size as usize, i / block_grid_size as usize))
        .unwrap()
}

fn find_max(grid_serial: i32) -> (i32, i32, i32) {
    let width = 300;
    let height = 300;
    let mut block_size = 3;
    let power_levels = compute_power_levels(width, height, grid_serial);
    let mut blocks = compute_power_level_blocks(width, height, block_size, &power_levels);
    let stride = 298;

    let mut max_location = (0, 0, 0);
    let mut max_value = 0;
    loop {
        // Find the max
        for y in 0..height - (block_size - 1) {
            for x in 0..width - (block_size - 1) {
                if blocks[(y * stride + x) as usize] > max_value {
                    max_value = blocks[(y * stride + x) as usize];
                    max_location = (x, y, block_size);
                }
            }
        }

        // Increase the block size
        block_size = block_size + 1;
        if block_size > 300 {
            return max_location;
        }

        // Increase the blocks range
        for y in 0..height - (block_size - 1) {
            for x in 0..width - (block_size - 1) {
                for by in 0..block_size - 1 {
                    blocks[(y * stride + x) as usize] +=
                        power_levels[((y + by) * 300 + x + block_size - 1) as usize]
                }

                for bx in 0..block_size {
                    blocks[(y * stride + x) as usize] +=
                        power_levels[((y + block_size - 1) * 300 + x + bx) as usize]
                }
            }
        }
    }
}

fn main() {
    let input = 5535;

    let result1 = find_max_block(input);
    println!("Result 1: {},{}", result1.0, result1.1);

    let result2 = find_max(input);
    println!("Result 2: {},{},{}", result2.0, result2.1, result2.2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn power_levels() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn max_blocks() {
        assert_eq!(find_max_block(18), (33, 45));
        assert_eq!(find_max_block(42), (21, 61));
    }

    #[test]
    fn max_blocks_dynamic() {
        assert_eq!(find_max(18), (90, 269, 16));
        assert_eq!(find_max(42), (232, 251, 12));
    }
}
