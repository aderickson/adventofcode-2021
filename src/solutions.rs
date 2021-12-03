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
    let mut lines = lines.peekable();

    let width = lines.peek().unwrap().len();
    let numbers : Vec<u32> = lines.map(|line| {
        u32::from_str_radix(line, 2).unwrap()
    }).collect();

    let mut numbers_oxy = numbers.clone();
    let mut numbers_co2 = numbers.clone();

    let mut power = 2u32.pow(width as u32 - 1);
    while numbers_oxy.len() > 1 && power > 0 {
        let mut has_one = Vec::<u32>::new();
        let mut has_zero = Vec::<u32>::new();
        for number in numbers_oxy {
            if number & power == power {
                has_one.push(number);
            } else {
                has_zero.push(number);
            }
        }
        
        if has_one.len() >= has_zero.len() {
            numbers_oxy = has_one;
        } else {
            numbers_oxy = has_zero;
        }

        power /= 2;
    }

    let mut power = 2u32.pow(width as u32 - 1);
    while numbers_co2.len() > 1 && power > 0 {
        let mut has_one = Vec::<u32>::new();
        let mut has_zero = Vec::<u32>::new();
        for number in numbers_co2 {
            if number & power == power {
                has_one.push(number);
            } else {
                has_zero.push(number);
            }
        }
        
        if has_zero.len() <= has_one.len() {
            numbers_co2 = has_zero;
        } else {
            numbers_co2 = has_one;
        }

        power /= 2;
    }

    if numbers_oxy.len() != 1 || numbers_co2.len() != 1 {
        panic!("Did not reduce list down to one number");
    }

    let oxy = numbers_oxy[0];
    let co2 = numbers_co2[0];

    return oxy * co2;
}