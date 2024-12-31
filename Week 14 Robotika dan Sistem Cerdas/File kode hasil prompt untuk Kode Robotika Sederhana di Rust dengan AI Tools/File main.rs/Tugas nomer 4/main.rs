use std::collections::BinaryHeap; // Mengimpor BinaryHeap untuk mengimplementasikan antrean prioritas.

// Struktur untuk merepresentasikan tugas.
#[derive(Eq, PartialEq)] // Menambahkan trait Eq dan PartialEq untuk memungkinkan perbandingan.
struct Task {
    priority: i32, // Prioritas tugas (angka lebih besar = prioritas lebih tinggi).
    description: String, // Deskripsi tugas.
}

// Mengimplementasikan trait Ord untuk menentukan urutan prioritas.
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Membandingkan tugas berdasarkan prioritas (dibalik agar BinaryHeap menjadi max-heap).
        other.priority.cmp(&self.priority)
    }
}

// Mengimplementasikan trait PartialOrd untuk perbandingan parsial.
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Membuat antrean prioritas menggunakan BinaryHeap.
    let mut task_queue: BinaryHeap<Task> = BinaryHeap::new();

    // Menambahkan tugas ke antrean dengan prioritas tertentu.
    task_queue.push(Task {
        priority: 3,
        description: String::from("Periksa sensor robot."),
    });
    task_queue.push(Task {
        priority: 5,
        description: String::from("Isi ulang baterai robot."),
    });
    task_queue.push(Task {
        priority: 1,
        description: String::from("Bersihkan area kerja."),
    });
    task_queue.push(Task {
        priority: 4,
        description: String::from("Kalibrasi lengan robot."),
    });

    println!("Robot memulai menyelesaikan tugas berdasarkan prioritas:");

    // Menyelesaikan tugas berdasarkan prioritas tertinggi.
    while let Some(task) = task_queue.pop() {
        println!(
            "Menyelesaikan tugas: '{}' dengan prioritas {}",
            task.description, task.priority
        );
    }

    println!("Semua tugas telah diselesaikan!");
}
