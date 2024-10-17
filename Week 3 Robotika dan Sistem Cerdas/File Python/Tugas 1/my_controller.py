from controller import Robot

# Inisialisasi robot
robot = Robot()

# Waktu langkah simulasi (ms)
timestep = int(robot.getBasicTimeStep())

# Mendapatkan akses ke motor kiri dan kanan
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Mengatur mode motor ke kecepatan
left_motor.setPosition(float('inf'))  # Inf berarti tidak ada batasan posisi (untuk rotasi terus menerus)
right_motor.setPosition(float('inf'))

# Kecepatan konstan untuk maju
speed = 3.0

# Mengatur kecepatan motor untuk bergerak maju tanpa henti
left_motor.setVelocity(speed)
right_motor.setVelocity(speed)

# Loop simulasi
while robot.step(timestep) != -1:
    # Tidak ada kontrol umpan balik, hanya terus bergerak maju
    pass  # Open-loop control tidak memiliki penyesuaian
