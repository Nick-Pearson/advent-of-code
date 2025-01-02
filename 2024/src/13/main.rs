fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Prize {
    x: usize,
    y: usize,
}

struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(parse_claw_machine)
        .filter_map(|machine| cheapest_way_to_win(&machine))
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(parse_claw_machine)
        .map(|machines| ClawMachine {
            button_a: machines.button_a,
            button_b: machines.button_b,
            prize: Prize {
                x: machines.prize.x + 10000000000000_usize,
                y: machines.prize.y + 10000000000000_usize,
            },
        })
        .filter_map(|machine| cheapest_way_to_win(&machine))
        .sum()
}

fn cheapest_way_to_win(machine: &ClawMachine) -> Option<usize> {
    // solve a system of linear equations
    let xa = machine.button_a.x as i128;
    let xb = machine.button_b.x as i128;
    let ya = machine.button_a.y as i128;
    let yb = machine.button_b.y as i128;

    let determinant = (xa * yb) - (xb * ya);
    if determinant == 0 {
        return None;
    }

    let prize_x = machine.prize.x as i128;
    let prize_y = machine.prize.y as i128;
    let n = yb * prize_x - xb * prize_y;
    let m = xa * prize_y - ya * prize_x;
    let num_a_presses = n / determinant;
    let num_b_presses = m / determinant;
    if num_a_presses < 0 || num_b_presses < 0 || n % determinant != 0 || m % determinant != 0 {
        return None;
    }
    Some((num_a_presses as usize * 3) + num_b_presses as usize)
}

fn parse_claw_machine(input: &str) -> ClawMachine {
    let mut lines = input.lines();
    let button_a = parse_button(lines.next().unwrap());
    let button_b = parse_button(lines.next().unwrap());
    let prize = parse_prize(lines.next().unwrap());
    ClawMachine {
        button_a,
        button_b,
        prize,
    }
}

fn parse_button(input: &str) -> Button {
    let x = input.split(',').next().unwrap()[11..].parse().unwrap();
    let y = input.split(',').nth(1).unwrap()[2..].parse().unwrap();
    Button { x, y }
}

fn parse_prize(input: &str) -> Prize {
    let x = input.split(',').next().unwrap()[9..].parse().unwrap();
    let y = input.split(',').nth(1).unwrap()[3..].parse().unwrap();
    Prize { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(part_one(input), 480);
    }

    #[test]
    fn test_part_two() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(part_two(input), 875318608908);
    }
}
