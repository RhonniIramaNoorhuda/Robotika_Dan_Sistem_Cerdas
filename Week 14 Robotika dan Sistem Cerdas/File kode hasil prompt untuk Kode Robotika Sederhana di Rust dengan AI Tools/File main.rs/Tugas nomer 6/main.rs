use rand::Rng; // Untuk menghasilkan angka acak.

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn manhattan_distance(p1: Position, p2: Position) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn main() {
    let start = Position { x: 0, y: 0 };
    let goal = Position { x: 4, y: 4 };
    let map = vec![
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let sensor_uncertainty = 0.2;

    println!("Peta:");
    for row in &map {
        println!("{:?}", row);
    }
    println!("Posisi awal: {:?}", start);
    println!("Tujuan: {:?}", goal);

    let mut current_position = start;

    while current_position != goal {
        let mut possible_moves = vec![
            Position {
                x: current_position.x,
                y: current_position.y - 1,
            },
            Position {
                x: current_position.x,
                y: current_position.y + 1,
            },
            Position {
                x: current_position.x - 1,
                y: current_position.y,
            },
            Position {
                x: current_position.x + 1,
                y: current_position.y,
            },
        ];

        possible_moves.retain(|pos| {
            pos.x >= 0
                && pos.y >= 0
                && pos.x < map.len() as i32
                && pos.y < map[0].len() as i32
                && map[pos.x as usize][pos.y as usize] == 0
        });

        let mut rng = rand::thread_rng();
        for move_pos in &mut possible_moves {
            let noise: f32 = rng.gen_range(-sensor_uncertainty..sensor_uncertainty);
            let noisy_distance = manhattan_distance(*move_pos, goal) as f32 * (1.0 + noise);
            move_pos.x += noisy_distance.round() as i32;
        }

        possible_moves.sort_by(|a, b| {
            manhattan_distance(*a, goal).cmp(&manhattan_distance(*b, goal))
        });

        if let Some(best_move) = possible_moves.first() {
            current_position = *best_move;
            println!("Robot bergerak ke: {:?}", current_position);
        } else {
            println!("Tidak ada langkah valid yang tersedia. Pergerakan berhenti.");
            break;
        }
    }

    if current_position == goal {
        println!("Robot telah mencapai tujuan: {:?}", goal);
    } else {
        println!("Robot gagal mencapai tujuan.");
    }
}
