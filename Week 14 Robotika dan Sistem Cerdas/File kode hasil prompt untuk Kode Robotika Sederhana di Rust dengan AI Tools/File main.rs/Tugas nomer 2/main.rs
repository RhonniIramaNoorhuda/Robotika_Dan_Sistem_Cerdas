use std::io; // Mengimpor modul input/output.

fn main() {
    // Inisialisasi posisi awal robot pada (0, 0).
    let mut position = (0, 0);

    println!("Robot Positioning Program");
    println!("Robot dimulai pada posisi (0, 0).");

    loop {
        // Menampilkan posisi saat ini.
        println!("Posisi robot saat ini: ({}, {})", position.0, position.1);

        // Menanyakan arah pergerakan kepada pengguna.
        println!("Masukkan arah pergerakan (up, down, left, right) atau 'exit' untuk keluar:");

        // Membaca input dari pengguna.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");
        
        // Menghapus karakter newline dari input.
        let input = input.trim().to_lowercase();

        // Mengecek input untuk menentukan tindakan.
        match input.as_str() {
            "up" => {
                position.1 += 1; // Menambah nilai y untuk bergerak ke atas.
                println!("Robot bergerak ke atas.");
            }
            "down" => {
                position.1 -= 1; // Mengurangi nilai y untuk bergerak ke bawah.
                println!("Robot bergerak ke bawah.");
            }
            "left" => {
                position.0 -= 1; // Mengurangi nilai x untuk bergerak ke kiri.
                println!("Robot bergerak ke kiri.");
            }
            "right" => {
                position.0 += 1; // Menambah nilai x untuk bergerak ke kanan.
                println!("Robot bergerak ke kanan.");
            }
            "exit" => {
                println!("Program selesai. Posisi akhir robot: ({}, {})", position.0, position.1);
                break; // Keluar dari loop.
            }
            _ => {
                println!("Perintah tidak dikenal. Gunakan 'up', 'down', 'left', 'right', atau 'exit'.");
            }
        }
    }
}
