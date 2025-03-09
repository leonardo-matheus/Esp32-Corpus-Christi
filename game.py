import asyncio
from bleak import BleakClient, BleakScanner
import pygame
import struct  # Para interpretar dados binários do BLE Gamepad

# Endereço MAC do ESP32 BLE Gamepad
ESP32_BLE_MAC = "E0:E2:E6:63:23:96"

# Configurações da janela do jogo (representando 3x3m)
WINDOW_WIDTH, WINDOW_HEIGHT = 900, 900  # Pixels na janela do jogo (900x900)
CENTER_X, CENTER_Y = WINDOW_WIDTH // 2, WINDOW_HEIGHT // 2  # Posição inicial do pincel
BRUSH_COLOR = (255, 0, 0)  # Cor do pincel (vermelho)
BRUSH_SIZE = 20  # Tamanho do pincel
FPS = 60  # Atualização do jogo (frames por segundo)

# UUID da característica BLE que recebe os dados (padrão no ESP32 BLE Gamepad)
BLE_CHARACTERISTIC_UUID = "2a4d"  # UUID para os dados do controle

# Classe do pincel virtual
class Pincel:
    def __init__(self):
        self.x = CENTER_X
        self.y = CENTER_Y
        self.painting = False  # Quando o gatilho é pressionado

    def move(self, delta_x, delta_y):
        """Move o pincel dentro da área delimitada"""
        self.x += delta_x
        self.y += delta_y
        self.x = max(0, min(WINDOW_WIDTH, self.x))  # Restringe ao limite da janela
        self.y = max(0, min(WINDOW_HEIGHT, self.y))

    def start_painting(self):
        """Inicia a pintura"""
        self.painting = True

    def stop_painting(self):
        """Para a pintura"""
        self.painting = False


async def receber_dados_ble(client, pincel):
    """Processa os valores recebidos do BLE Gamepad e movimenta o pincel"""
    while True:
        try:
            # Lê dados binários do BLE Gamepad
            data = await client.read_gatt_char(BLE_CHARACTERISTIC_UUID)

            # Decodifica os valores (6 bytes: [X, Y, botão])
            if len(data) >= 6:
                x_axis, y_axis, button_state, _ = struct.unpack("hhbb", data[:6])

                # Conversão dos valores para a escala da janela (900x900 pixels)
                delta_x = int((x_axis / 32767.0) * (WINDOW_WIDTH / 3))
                delta_y = int((y_axis / 32767.0) * (WINDOW_HEIGHT / 3))

                pincel.move(delta_x, delta_y)

                # Atualiza o estado do botão ("gatilho")
                if button_state & 0x01:
                    pincel.start_painting()
                else:
                    pincel.stop_painting()

            await asyncio.sleep(0.02)  # Intervalo entre leituras (~50 Hz)
        except Exception as e:
            print(f"Erro ao processar dados BLE: {e}")


async def jogo():
    """Função principal do jogo (integrada ao BLE)"""
    # Inicializa o cliente BLE
    async with BleakClient(ESP32_BLE_MAC) as client:
        print("Conectado ao ESP32 BLE Gamepad!")

        # Inicializa o Pygame
        pygame.init()
        screen = pygame.display.set_mode((WINDOW_WIDTH, WINDOW_HEIGHT))
        pygame.display.set_caption("Tapete Corpus Christi - Pincel Virtual")
        clock = pygame.time.Clock()
        painting_surface = pygame.Surface((WINDOW_WIDTH, WINDOW_HEIGHT), pygame.SRCALPHA)

        pincel = Pincel()

        # Inicia a tarefa BLE para ler dados do ESP32
        ble_task = asyncio.create_task(receber_dados_ble(client, pincel))

        running = True
        while running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False

            # Processa o movimento e a pintura do pincel
            if pincel.painting:
                pygame.draw.circle(painting_surface, BRUSH_COLOR, (pincel.x, pincel.y), BRUSH_SIZE)

            # Renderiza o jogo
            screen.fill((255, 255, 255))  # Fundo branco
            screen.blit(painting_surface, (0, 0))  # Pintura acumulada
            pygame.draw.circle(screen, BRUSH_COLOR, (pincel.x, pincel.y), 10)  # Mostra o pincel

            pygame.display.flip()
            clock.tick(FPS)  # Controla a taxa de quadros

        # Finaliza tarefas
        ble_task.cancel()
        pygame.quit()


if __name__ == "__main__":
    asyncio.run(jogo())