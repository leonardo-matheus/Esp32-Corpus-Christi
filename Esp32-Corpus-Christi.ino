#include <Wire.h>
#include <Adafruit_MPU6050.h>
#include <Adafruit_Sensor.h>
#include <BleGamepad.h> // Biblioteca para emular um gamepad via BLE

// Sensor MPU6050
Adafruit_MPU6050 mpu;

// Gamepad BLE
BleGamepad bleGamepad;

void setup() {
  Serial.begin(115200);
  while (!Serial) delay(10); // Aguarda o monitor serial

  // Inicializa o MPU6050
  if (!mpu.begin()) {
    Serial.println("Falha ao encontrar o MPU6050!");
    while (1);
  }
  Serial.println("MPU6050 inicializado com sucesso!");

  // Configura o MPU6050
  mpu.setAccelerometerRange(MPU6050_RANGE_2_G);
  mpu.setGyroRange(MPU6050_RANGE_250_DEG);
  mpu.setFilterBandwidth(MPU6050_BAND_21_HZ);

  // Inicializa o gamepad BLE
  bleGamepad.begin();
  Serial.println("Gamepad BLE inicializado. Aguardando conexão...");
}

void loop() {
  // Verifica se o gamepad está conectado
  if (bleGamepad.isConnected()) {
    // Lê os dados do acelerômetro
    sensors_event_t accel, gyro, temp;
    mpu.getEvent(&accel, &gyro, &temp);

    // Mapeia os valores do acelerômetro para os eixos do gamepad

    //Definir OffSet ao iniciar
    int16_t xAxis = map(accel.acceleration.x * 100, -2000, 2000, -32768, 32767); // Eixo X
    int16_t yAxis = map(accel.acceleration.y * 100, -2000, 2000, -32768, 32767); // Eixo Y
    int16_t zAxis = map(accel.acceleration.z * 100, -2000, 2000, -32768, 32767); // Eixo Z

    //Configurar eixos de rotação XYZ

    // Define os eixos do gamepad
    bleGamepad.setAxes(xAxis, yAxis, zAxis);

    // Debug no Serial Monitor
    Serial.print("Acelerômetro (m/s^2): X=");
    Serial.print(accel.acceleration.x);
    Serial.print(", Y=");
    Serial.print(accel.acceleration.y);
    Serial.print(", Z=");
    Serial.println(accel.acceleration.z);

    Serial.print("Gamepad: X=");
    Serial.print(xAxis);
    Serial.print(", Y=");
    Serial.print(yAxis);
    Serial.print(", Z=");
    Serial.println(zAxis);
  }

  delay(100); // Ajuste o delay conforme necessário
}