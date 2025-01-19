#include <Wire.h>
#include <Adafruit_MPU6050.h>
#include <Adafruit_Sensor.h>

// sensor
Adafruit_MPU6050 mpu;

void setup() {
  Serial.begin(115200); // serial
  while (!Serial) delay(10); // Aguarda o monitor

  // Inicia o MPU6050
  if (!mpu.begin()) {
    Serial.println("Falha ao encontrar o MPU6050!");
    while (1);
  }

  Serial.println("MPU6050 inicializado com sucesso!");
  mpu.setAccelerometerRange(MPU6050_RANGE_2_G);
  mpu.setGyroRange(MPU6050_RANGE_250_DEG);
  mpu.setFilterBandwidth(MPU6050_BAND_21_HZ);
}

void loop() {
  // Lê o sensor
  sensors_event_t accel, gyro, temp;
  mpu.getEvent(&accel, &gyro, &temp);

  // Printa
  Serial.println("========== Dados do MPU6050 ==========");
  Serial.print("Acelerômetro (m/s^2): X=");
  Serial.print(accel.acceleration.x);
  Serial.print(", Y=");
  Serial.print(accel.acceleration.y);
  Serial.print(", Z=");
  Serial.println(accel.acceleration.z);

  Serial.print("Giroscópio (rad/s): X=");
  Serial.print(gyro.gyro.x);
  Serial.print(", Y=");
  Serial.print(gyro.gyro.y);
  Serial.print(", Z=");
  Serial.println(gyro.gyro.z);

  Serial.print("Temperatura (C): ");
  Serial.println(temp.temperature);

  delay(1000); // Espera 1 seg
}

