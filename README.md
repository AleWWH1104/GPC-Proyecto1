# Raycaster Laberinto

Proyecto en Rust que renderiza un laberinto estilo “raycasting”: paredes, sprites, colisiones, pantalla de éxito y audio (música de fondo y sonidos por proximidad).

---
## ¿Cómo funciona?
- **Raycasting por columnas**: cada rayo calcula la distancia perpendicular a la pared y se dibuja un “slice” escalado.
- **Sprites**: Algunos con animacion.
- **Colisiones**: el jugador no puede atravesar paredes del laberinto.
- **Audio**:
  - Sonidos de proximidad ligados a sprites.
  - Música de fondo que cambia cuando entras a zonas especiales.
- **Pantalla de éxito**: al llegar a la zona meta.

## Estructura 

- `main.rs` → loop principal, entrada del jugador, estados de juego (Playing/Success).
- `framebuffer.rs` → manejo del buffer de píxeles y z-buffer.
- `caster.rs` → raycasting y proyección de paredes.
- `maze.rs` → definición del mapa del laberinto.
- `player.rs` → posición, ángulo y movimiento con colisiones.
- `sprites.rs` → renderizado de sprites con chequeo de profundidad.
- `textures.rs` → carga y caché de texturas/sprites.
- `audio.rs` → sistema de audio (música y sonidos de proximidad).

## Requisitos

- **Rust** (toolchain estable).
- **raylib** y dependencias C (compilador + CMake).
  - En Linux:  
    ```bash
    sudo apt install build-essential cmake pkg-config \
      libasound2-dev libx11-dev libxrandr-dev libxi-dev \
      libxcursor-dev libxinerama-dev libgl1-mesa-dev
    ```
  - En macOS:  
    ```bash
    brew install raylib
    ```
  - En Windows: Visual Studio Build Tools + CMake, y asegúrate de usar el toolchain `msvc`.

### Ejecutar
Dentro de la raíz del proyecto (donde está `Cargo.toml`):

```bash
cargo run
```

## Video demo del juego
https://youtu.be/LncLoEckh_Y

