#include <Wire.h>
#include <Adafruit_MPU6050.h>
#include <Adafruit_Sensor.h>
#include <BleGamepad.h> // Biblioteca para emular um gamepad via BLE

// Sensor MPU6050
Adafruit_MPU6050 mpu;

// BLE Gamepad
BleGamepad bleGamepad;

void setup() {
  // Inicializa a comunicação serial para debug
  Serial.begin(115200);
  while (!Serial) delay(10); // Aguarda o monitor serial

  // Inicializa o BLE Gamepad
  Serial.println("Inicializando Gamepad BLE...");
  bleGamepad.begin();
  Serial.println("Gamepad BLE inicializado! Aguardando conexão...");

  // Inicializa o sensor MPU6050
  Serial.println("Inicializando MPU6050...");
  if (!mpu.begin()) {
    Serial.println("Falha ao inicializar MPU6050!");
    while (1); // Trava aqui se houver falha no MPU6050
  }
  Serial.println("MPU6050 inicializado!");

  // Configurações do MPU6050
  mpu.setAccelerometerRange(MPU6050_RANGE_2_G);  // Configura a faixa do acelerômetro
  mpu.setGyroRange(MPU6050_RANGE_250_DEG);       // Configura a faixa do giroscópio
  mpu.setFilterBandwidth(MPU6050_BAND_21_HZ);    // Redução de ruídos no sensor

  Serial.println("Setup concluído! Aguardando conexão BLE...");
}

void loop() {
  // Verifica se o Gamepad BLE está conectado
  if (bleGamepad.isConnected()) {
    // Lê os dados do sensor MPU6050
    sensors_event_t accel, gyro, temp;
    mpu.getEvent(&accel, &gyro, &temp);

    // Mapeia os valores do acelerômetro para os eixos do Gamepad (-32768 a 32767)
    int16_t xAxis = map(accel.acceleration.x * 100, -2000, 2000, -32768, 32767);
    int16_t yAxis = map(accel.acceleration.y * 100, -2000, 2000, -32768, 32767);
    int16_t zAxis = map(accel.acceleration.z * 100, -2000, 2000, -32768, 32767);

    // Define os eixos no Gamepad
    bleGamepad.setAxes(xAxis, yAxis, zAxis);

    // Debug no monitor serial
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
  } else {
    Serial.println("Aguardando conexão BLE...");
  }

  delay(100); // Ajuste o delay conforme necessário
}