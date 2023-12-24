#[derive(Debug, Clone)]
enum Item {
    Symbol(char),
    PartNumber(u32),
}

#[derive(Debug, Clone)]
struct ItemCoord {
    item: Item,
    start_x: i32,
    end_x: i32,
    y: i32,
}

fn main() {
    let input = include_str!("input.txt");
    let items: Vec<ItemCoord> = input
        .lines()
        .enumerate()
        .map(|line| process_line(line.1, line.0 as i32))
        .flat_map(|v| v.into_iter())
        .collect();
    dbg(&filter_part_one(&items));
    let part_one: u32 = filter_part_one(&items)
        .iter()
        .map(|i| match i.item {
            Item::Symbol(_) => 0,
            Item::PartNumber(num) => num,
        })
        .sum();
    println!("Part one: {}", part_one);
}

fn dbg(items: &Vec<ItemCoord>) {
    let mut y = 0;
    let mut x = 0;
    for i in items {
        while i.y > y {
            println!("");
            x = 0;
            y = y + 1;
        }
        while i.start_x > x {
            print!(".");
            x = x + 1;
        }

        match i.item {
            Item::Symbol(c) => print!("{}", c),
            Item::PartNumber(num) => print!("{}", num),
        }
        x = i.end_x + 1;
    };
    println!("");
}

fn filter_part_one(items: &Vec<ItemCoord>) -> Vec<ItemCoord> {
    items
        .iter()
        .filter(|i| match i.item {
            Item::PartNumber(_) => is_adjacent_to_symbol(items, i),
            _ => false,
        })
        .cloned()
        .collect()
}

fn is_adjacent_to_symbol(items: &Vec<ItemCoord>, i: &ItemCoord) -> bool {
    let mut coords_to_check = Vec::new();
    coords_to_check.push((i.start_x - 1, i.y));
    coords_to_check.push((i.end_x + 1, i.y));
    for x in i.start_x - 1..=i.end_x + 1 {
        coords_to_check.push((x, i.y - 1));
        coords_to_check.push((x, i.y + 1));
    }

    coords_to_check
        .into_iter()
        .filter_map(|(x, y)| find_item(items, x, y))
        .find(|x| match x.item {
            Item::Symbol(_) => true,
            Item::PartNumber(_) => false,
        })
        .is_some()
}

fn find_item(items: &Vec<ItemCoord>, x: i32, y: i32) -> Option<ItemCoord> {
    let key = x + (y * 10000);
    let idx = items.binary_search_by_key(&key, |item| item.start_x + item.y * 10000);
    idx.ok().map(|i| items[i].clone())
}

fn process_line(line: &str, y_coord: i32) -> Vec<ItemCoord> {
    let mut result = Vec::new();
    let mut current_part = None;
    let mut start_x = None;
    for (x_coord, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            current_part = Some(current_part.map_or(0, |x| x * 10) + c.to_digit(10).unwrap());
            start_x.get_or_insert(x_coord as i32);
        } else {
            if current_part.is_some() {
                result.push(ItemCoord {
                    item: Item::PartNumber(current_part.take().unwrap()),
                    start_x: start_x.take().unwrap(),
                    end_x: x_coord as i32 - 1,
                    y: y_coord,
                })
            }
            if c != '.' {
                result.push(ItemCoord {
                    item: Item::Symbol(c),
                    start_x: x_coord as i32,
                    end_x: x_coord as i32,
                    y: y_coord,
                })
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let items: Vec<ItemCoord> = input
            .lines()
            .enumerate()
            .map(|line| process_line(line.1, line.0 as i32))
            .flat_map(|v| v.into_iter())
            .collect();
        let part_one: u32 = filter_part_one(&items)
            .iter()
            .map(|i| match i.item {
                Item::Symbol(_) => 0,
                Item::PartNumber(num) => num,
            })
            .sum();
        assert_eq!(4361, part_one)
    }
}
