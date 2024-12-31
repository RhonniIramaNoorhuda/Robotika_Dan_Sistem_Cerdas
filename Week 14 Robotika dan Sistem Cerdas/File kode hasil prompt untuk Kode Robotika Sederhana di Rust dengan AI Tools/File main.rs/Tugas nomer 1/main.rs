use std::collections::VecDeque; // Mengimpor VecDeque untuk digunakan dalam algoritma BFS.

fn main() {
    // Matriks 2D yang merepresentasikan area.
    // 0: jalan bebas, 1: rintangan.
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    // Titik awal (0, 0) dan tujuan (4, 4).
    let start = (0, 0);
    let goal = (4, 4);

    // Memanggil fungsi untuk mencari jalur.
    if let Some(path) = find_path(grid, start, goal) {
        println!("Jalur ditemukan: {:?}", path);
    } else {
        println!("Tidak ada jalur yang ditemukan.");
    }
}

// Fungsi untuk mencari jalur menggunakan algoritma BFS.
fn find_path(grid: Vec<Vec<i32>>, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len(); // Jumlah baris dalam matriks.
    let cols = grid[0].len(); // Jumlah kolom dalam matriks.

    // Memeriksa apakah titik awal atau tujuan berada di rintangan.
    if grid[start.0][start.1] == 1 || grid[goal.0][goal.1] == 1 {
        return None;
    }

    // Vektor untuk merepresentasikan 4 arah (atas, bawah, kiri, kanan).
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Matriks untuk melacak apakah suatu sel telah dikunjungi.
    let mut visited = vec![vec![false; cols]; rows];
    visited[start.0][start.1] = true; // Menandai titik awal sebagai sudah dikunjungi.

    // Queue untuk BFS yang berisi posisi saat ini dan jalur yang ditempuh.
    let mut queue = VecDeque::new();
    queue.push_back((start, vec![start])); // Memulai BFS dengan titik awal.

    while let Some((current, path)) = queue.pop_front() {
        // Jika mencapai tujuan, kembalikan jalur.
        if current == goal {
            return Some(path);
        }

        // Mengeksplorasi semua arah.
        for direction in &directions {
            let next_row = current.0 as isize + direction.0;
            let next_col = current.1 as isize + direction.1;

            // Memastikan indeks valid dan sel dapat dilalui.
            if next_row >= 0
                && next_row < rows as isize
                && next_col >= 0
                && next_col < cols as isize
                && grid[next_row as usize][next_col as usize] == 0
                && !visited[next_row as usize][next_col as usize]
            {
                // Menandai sel sebagai sudah dikunjungi.
                visited[next_row as usize][next_col as usize] = true;

                // Menambahkan sel berikutnya ke dalam queue dengan jalur yang diperbarui.
                let mut new_path = path.clone();
                new_path.push((next_row as usize, next_col as usize));
                queue.push_back(((next_row as usize, next_col as usize), new_path));
            }
        }
    }

    // Jika tidak ada jalur yang ditemukan.
    None
}
