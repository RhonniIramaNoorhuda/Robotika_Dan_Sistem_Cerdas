from controller import Robot

# Inisialisasi robot
robot = Robot()

# Waktu langkah simulasi (ms)
timestep = int(robot.getBasicTimeStep())

# Mendapatkan akses ke motor kiri dan kanan
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Mengatur mode motor ke kecepatan
left_motor.setPosition(float('inf'))
right_motor.setPosition(float('inf'))

# Kecepatan roda
left_speed = 2.0  # Kecepatan roda kiri lebih lambat
right_speed = 5.0  # Kecepatan roda kanan lebih cepat

# Mengatur kecepatan motor
left_motor.setVelocity(left_speed)
right_motor.setVelocity(right_speed)

# Loop simulasi
while robot.step(timestep) != -1:
    # Robot akan bergerak melingkar karena perbedaan kecepatan roda
    pass  # Tidak ada umpan balik atau perubahan kecepatan
