use std::sync::mpsc; // Untuk komunikasi antar-thread menggunakan channel.
use std::thread; // Untuk membuat thread.
use std::time::Duration; // Untuk mengatur jeda waktu dalam simulasi.

// Enum untuk jenis event yang dideteksi oleh robot.
enum Event {
    ObstacleDetected((i32, i32)), // Rintangan baru ditemukan di koordinat tertentu.
    GoalChanged((i32, i32)),      // Titik tujuan diubah ke koordinat tertentu.
    NoEvent,                      // Tidak ada event.
}

// Fungsi simulasi untuk mendeteksi event di lingkungan.
fn simulate_environment(tx: mpsc::Sender<Event>) {
    let events = vec![
        Event::ObstacleDetected((2, 2)),
        Event::GoalChanged((4, 4)),
        Event::NoEvent,
        Event::ObstacleDetected((3, 3)),
    ];

    for event in events {
        thread::sleep(Duration::from_secs(2)); // Simulasi jeda waktu sebelum event berikutnya.
        tx.send(event).unwrap(); // Mengirim event ke robot melalui channel.
    }
}

// Fungsi utama untuk menjalankan sistem robotik berbasis event-driven.
fn main() {
    // Channel untuk komunikasi antara lingkungan dan robot.
    let (tx, rx) = mpsc::channel();

    // Thread untuk mensimulasikan lingkungan.
    thread::spawn(move || simulate_environment(tx));

    // Titik awal robot dan tujuan awal.
    let mut robot_position = (0, 0);
    let mut goal = (5, 5);

    println!("Robot memulai di posisi: {:?}", robot_position);

    loop {
        // Menerima event dari lingkungan.
        match rx.recv() {
            Ok(Event::ObstacleDetected(coord)) => {
                println!("Rintangan baru terdeteksi di {:?}", coord);
                println!("Robot menghitung ulang jalur untuk menghindari rintangan.");
                // Tambahkan logika untuk menghitung ulang jalur jika diperlukan.
            }
            Ok(Event::GoalChanged(new_goal)) => {
                println!("Tujuan diubah ke {:?}", new_goal);
                goal = new_goal;
                println!("Robot bergerak menuju tujuan baru.");
            }
            Ok(Event::NoEvent) => {
                println!("Tidak ada perubahan di lingkungan.");
            }
            Err(_) => {
                println!("Lingkungan berhenti mengirim event. Robot selesai.");
                break;
            }
        }

        // Logika untuk memindahkan robot menuju tujuan jika tidak ada rintangan.
        if robot_position != goal {
            if robot_position.0 < goal.0 {
                robot_position.0 += 1;
            } else if robot_position.0 > goal.0 {
                robot_position.0 -= 1;
            }

            if robot_position.1 < goal.1 {
                robot_position.1 += 1;
            } else if robot_position.1 > goal.1 {
                robot_position.1 -= 1;
            }

            println!("Robot bergerak ke posisi: {:?}", robot_position);
        } else {
            println!("Robot telah mencapai tujuan: {:?}", goal);
        }

        thread::sleep(Duration::from_secs(1)); // Simulasi pergerakan dengan jeda waktu.
    }
}
