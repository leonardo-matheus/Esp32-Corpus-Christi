#include <Wire.h>
#include <Adafruit_MPU6050.h>
#include <BleGamepad.h>

Adafruit_MPU6050 mpu;
BleGamepad bleGamepad;

void setup() {
  Serial.begin(115200);
  Wire.begin(21, 22, 400000); // SDA, SCL, frequência I2C
  
  if (!mpu.begin()) {
    Serial.println("MPU6050 falhou!");
    while(1);
  }
  
  mpu.setAccelerometerRange(MPU6050_RANGE_2_G);
  
  // Inicializar o BLE Gamepad com configuração padrão
  bleGamepad.begin();
}

void loop() {
  // Remover a chamada para wdtFeed()
  
  if (bleGamepad.isConnected()) {
    sensors_event_t accel, gyro, temp;
    
    if(mpu.getEvent(&accel, &gyro, &temp)) {
      int16_t x = map(accel.acceleration.x * 100, -1960, 1960, -32768, 32767);
      int16_t y = map(accel.acceleration.y * 100, -1960, 1960, -32768, 32767);
      
      // Deadzone dinâmica
      x = abs(x) < 800 ? 0 : x;
      y = abs(y) < 800 ? 0 : y;
      
      bleGamepad.setAxes(x, y, 0, 0, 0, 0, 0, 0);
      Serial.printf("X: %d | Y: %d\n", x, y);
    } else {
      Serial.println("Erro na leitura do MPU!");
    }
  }
  delay(30); // Delay para evitar sobrecarga
}