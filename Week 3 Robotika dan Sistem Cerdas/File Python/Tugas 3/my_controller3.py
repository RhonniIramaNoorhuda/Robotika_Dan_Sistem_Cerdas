from controller import Robot

# Inisialisasi robot
robot = Robot()

# Waktu langkah simulasi (ms)
timestep = int(robot.getBasicTimeStep())

# Mendapatkan akses ke motor kiri dan kanan
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Mengatur mode motor ke kecepatan
left_motor.setPosition(float('inf'))  # Tidak ada batasan posisi
right_motor.setPosition(float('inf'))

# Mendapatkan akses ke sensor proximity
proximity_sensors = []
sensor_names = ['ps0', 'ps7']  # Sensor proximity depan, ps0 di kiri dan ps7 di kanan
for name in sensor_names:
    sensor = robot.getDevice(name)
    sensor.enable(timestep)  # Aktifkan sensor proximity
    proximity_sensors.append(sensor)

# Kecepatan default untuk bergerak maju
forward_speed = 3.0

# Loop simulasi
while robot.step(timestep) != -1:
    # Membaca nilai sensor proximity
    sensor_values = [sensor.getValue() for sensor in proximity_sensors]
    
    # Jika salah satu sensor mendeteksi objek dekat, berhenti
    if sensor_values[0] > 80.0 or sensor_values[1] > 80.0:  # Nilai ambang batas deteksi objek
        left_motor.setVelocity(0)  # Hentikan motor kiri
        right_motor.setVelocity(0)  # Hentikan motor kanan
    else:
        # Jika tidak ada objek yang dekat, terus maju
        left_motor.setVelocity(forward_speed)
        right_motor.setVelocity(forward_speed)
