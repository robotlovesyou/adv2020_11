fn main() {
    let (start, row_len) = read_seats(include_str!("../input.txt").lines());
    let stable = step_to_stable(start, row_len, 4, count_occupied_neighbours);
    println!("There are {} occupied seats", count_occupied(&stable));

    let (start, row_len) = read_seats(include_str!("../input.txt").lines());
    let stable = step_to_stable(start, row_len, 5, count_occupied_neighbours_far);
    println!("There are {} occupied seats", count_occupied(&stable));
}

fn read_seats<'a>(input: impl Iterator<Item = &'a str>) -> (Vec<char>, usize) {
    let mut out: Vec<char> = Vec::new();
    let mut row_len = 0;
    for line in input {
        row_len = usize::max(row_len, line.len());
        out.append(&mut line.chars().collect::<Vec<char>>());
    }
    (out, row_len)
}

fn offset(x: usize, y: usize, row_len: usize) -> usize {
    y * row_len + x
}

fn seat_at(x: usize, y: usize, seats: &[char], row_len: usize) -> char {
    seats[offset(x, y, row_len)]
}

fn is_occupied(seat: char) -> bool {
    seat == '#'
}

fn count_occupied_neighbours(x: usize, y: usize, seats: &[char], row_len: usize) -> usize {
    let mut occupied = 0;
    // row above
    if y > 0 {
        if x > 0 && is_occupied(seat_at(x - 1, y - 1, seats, row_len)) {
            occupied += 1;
        }
        if is_occupied(seat_at(x, y - 1, seats, row_len)) {
            occupied += 1;
        }
        if x < row_len - 1 && is_occupied(seat_at(x + 1, y - 1, seats, row_len)) {
            occupied += 1;
        }
    }
    // same row
    if x > 0 && is_occupied(seat_at(x - 1, y, seats, row_len)) {
        occupied += 1;
    }
    if x < row_len - 1 && is_occupied(seat_at(x + 1, y, seats, row_len)) {
        occupied += 1;
    }
    // row below
    if y < seats.len() / row_len - 1 {
        if x > 0 && is_occupied(seat_at(x - 1, y + 1, seats, row_len)) {
            occupied += 1;
        }
        if is_occupied(seat_at(x, y + 1, seats, row_len)) {
            occupied += 1;
        }
        if x < row_len - 1 && is_occupied(seat_at(x + 1, y + 1, seats, row_len)) {
            occupied += 1;
        }
    }
    occupied
}

fn search(x: usize, y: usize, step_x: i64, step_y: i64, seats: &[char], row_len: usize) -> bool {
    let mut current_x = x as i64 + step_x;
    let mut current_y = y as i64 + step_y;
    while current_x >= 0
        && current_x < (row_len as i64)
        && current_y >= 0
        && (current_y as usize) < seats.len() / row_len
    {
        match seat_at(current_x as usize, current_y as usize, seats, row_len) {
            'L' => return false,
            '#' => return true,
            _ => {
                current_x += step_x;
                current_y += step_y;
                continue;
            }
        }
    }
    false
}

fn count_occupied_neighbours_far(x: usize, y: usize, seats: &[char], row_len: usize) -> usize {
    let mut occupied = 0;
    if search(x, y, -1, -1, seats, row_len) {
        occupied += 1;
    }
    if search(x, y, 0, -1, seats, row_len) {
        occupied += 1;
    }
    if search(x, y, 1, -1, seats, row_len) {
        occupied += 1;
    }
    if search(x, y, -1, 0, seats, row_len) {
        occupied += 1;
    }
    if search(x, y, 1, 0, seats, row_len) {
        occupied += 1;
    }
    if search(x, y, -1, 1, seats, row_len) {
        occupied += 1;
    }
    if search(x, y, 0, 1, seats, row_len) {
        occupied += 1;
    }
    if search(x, y, 1, 1, seats, row_len) {
        occupied += 1;
    }
    occupied
}

type CountOccupiedNeighbours = fn(usize, usize, &[char], usize) -> usize;

fn step(
    current: Vec<char>,
    mut next: Vec<char>,
    row_len: usize,
    tolerance: usize,
    count: CountOccupiedNeighbours,
) -> (Vec<char>, Vec<char>) {
    let rows = current.len() / row_len;
    for y in 0..rows {
        for x in 0..row_len {
            match seat_at(x, y, &current, row_len) {
                'L' => {
                    if count(x, y, &current, row_len) == 0 {
                        next[offset(x, y, row_len)] = '#';
                    } else {
                        next[offset(x, y, row_len)] = 'L';
                    }
                }
                '#' => {
                    if count(x, y, &current, row_len) >= tolerance {
                        next[offset(x, y, row_len)] = 'L';
                    } else {
                        next[offset(x, y, row_len)] = '#';
                    }
                }
                _ => (),
            }
        }
    }
    (next, current)
}

fn step_to_stable(
    mut current: Vec<char>,
    row_len: usize,
    tolerance: usize,
    count: CountOccupiedNeighbours,
) -> Vec<char> {
    let mut buffer = current.clone();
    loop {
        let (new_current, new_buffer) = step(current, buffer, row_len, tolerance, count);
        current = new_current;
        buffer = new_buffer;
        if current == buffer {
            return current;
        }
    }
}

fn count_occupied(seats: &[char]) -> usize {
    seats.iter().fold(
        0,
        |count, seat| if *seat == '#' { count + 1 } else { count },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const EXAMPLE_START: &str = indoc! {"\
    L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL"};

    const EXAMPLE_STEP_1: &str = indoc! {"\
    #.##.##.##
    #######.##
    #.#.#..#..
    ####.##.##
    #.##.##.##
    #.#####.##
    ..#.#.....
    ##########
    #.######.#
    #.#####.##"};

    const EXAMPLE_STEP_2: &str = indoc! {"\
    #.LL.L#.##
    #LLLLLL.L#
    L.L.L..L..
    #LLL.LL.L#
    #.LL.LL.LL
    #.LLLL#.##
    ..L.L.....
    #LLLLLLLL#
    #.LLLLLL.L
    #.#LLLL.##"};

    const EXAMPLE_FAR_ALL: &str = indoc! {"\
    .......#.
    ...#.....
    .#.......
    .........
    ..#L....#
    ....#....
    .........
    #........
    ...#....."};

    const EXAMPLE_FAR_NONE: &str = indoc! {"\
    .##.##.
    #.#.#.#
    ##...##
    ...L...
    ##...##
    #.#.#.#
    .##.##."};

    #[test]
    fn it_gets_the_correct_seat_offset() {
        assert_eq!(0, offset(0, 0, 10));
        assert_eq!(35, offset(5, 3, 10));
    }

    #[test]
    fn it_gets_the_correct_seat_state() {
        let (seats, row_len) = read_seats(EXAMPLE_START.lines());
        assert_eq!('L', seat_at(0, 0, &seats, row_len));
        assert_eq!('.', seat_at(7, 9, &seats, row_len));
    }

    #[test]
    fn it_counts_occupied_neighbours() {
        let (all_empty, row_len) = read_seats(EXAMPLE_START.lines());
        assert_eq!(0, count_occupied_neighbours(0, 0, &all_empty, row_len));
        assert_eq!(0, count_occupied_neighbours(3, 4, &all_empty, row_len));
        assert_eq!(0, count_occupied_neighbours(9, 9, &all_empty, row_len));

        let (all_full, _) = read_seats(EXAMPLE_STEP_1.lines());
        assert_eq!(2, count_occupied_neighbours(0, 0, &all_full, row_len));
        assert_eq!(6, count_occupied_neighbours(3, 4, &all_full, row_len));
        assert_eq!(2, count_occupied_neighbours(9, 9, &all_full, row_len));
    }

    #[test]
    fn it_steps_correctly() {
        let (mut current, row_len) = read_seats(EXAMPLE_START.lines());
        let mut next = current.clone();
        let (expected_next, _) = read_seats(EXAMPLE_STEP_1.lines());
        let mut stepped = step(current, next, row_len, 4, count_occupied_neighbours);
        // stepped.0 is the new current and stepped.1 is the new buffer
        current = stepped.0;
        next = stepped.1;
        assert_eq!(current, expected_next);

        let (expected_next, _) = read_seats(EXAMPLE_STEP_2.lines());
        stepped = step(current, next, row_len, 4, count_occupied_neighbours);
        current = stepped.0;
        assert_eq!(current, expected_next);
    }

    #[test]
    fn it_correctly_steps_to_stable() {
        let (current, row_len) = read_seats(EXAMPLE_START.lines());
        let stable = step_to_stable(current, row_len, 4, count_occupied_neighbours);
        assert_eq!(37, count_occupied(&stable));
    }

    #[test]
    fn it_correctly_counts_far_occupied_seats() {
        let (seats, row_len) = read_seats(EXAMPLE_FAR_ALL.lines());
        assert_eq!(8, count_occupied_neighbours_far(3, 4, &seats, row_len));

        let (seats, row_len) = read_seats(EXAMPLE_FAR_NONE.lines());
        assert_eq!(0, count_occupied_neighbours_far(3, 3, &seats, row_len));
    }
}
