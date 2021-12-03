pub fn part_one<'a>(lines : impl Iterator<Item = &'a str>) -> u32 {
    let mut lines = lines.peekable();

    let width = lines.peek().unwrap().len();
    let numbers = lines.map(|line| {
        u32::from_str_radix(line, 2).unwrap()
    });

    let mut count = 0u32;
    let mut positions = vec![0u32; width];

    for number in numbers {
        let mut number = number;
        let mut pow = 0;
        count += 1;

        while number > 0 {
            if number & 0x0001 != 0 {
                positions[pow] += 1;
            }

            number /= 2;
            pow += 1;
        }
    }

    let bits = positions.iter().map(|position_count| {
        if *position_count > count / 2 {
            return 1;
        } else {
            return 0;
        }
    });

    let mut gamma_rate = 0u32;
    let mut epsilon_rate = 0u32;
    let mut pow = 1u32;
    for bit in bits {
        gamma_rate += bit * pow;
        epsilon_rate += (bit + 1) % 2 * pow;
        pow *= 2;
    }

    return gamma_rate * epsilon_rate;
}

pub fn part_two<'a>(lines : impl Iterator<Item = &'a str>) -> u32 {
    panic!("Not implemented")
}