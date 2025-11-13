# Proyecto 2: Raytracing - Diorama

Proyecto de raytracing desarrollado en Rust usando raylib. Renderiza un diorama estilo Minecraft con texturas, iluminación dinámica, refracción, reflexión y materiales emisivos.

## Video de Demostración

[Ver video del proyecto en Canva](https://www.canva.com/design/DAG4mob_Ji0/robC7aFQmwByAuiILKAX0Q/edit?utm_content=DAG4mob_Ji0&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton)

## Controles

### Cámara
- **IZQUIERDA/DERECHA**: Rotar el mundo
- **ARRIBA/ABAJO**: Mirar arriba/abajo
- **Q/E**: Acercar/Alejar zoom
- **CLIC DERECHO + ARRASTRAR**: Orbitar cámara
- **RUEDA DEL MOUSE**: Zoom

### General
- **ESC**: Salir del programa

## Cómo Ejecutar

### Requisitos Previos
- Rust (versión 1.70 o superior)
- Cargo
- Windows (el proyecto fue desarrollado para Windows)

### Instalación y Ejecución

1. Clona el repositorio:
```bash
git clone https://github.com/miafuentes30/Proyecto-2-Raytracing.git
cd Proyecto-2-Raytracing
```

2. Compila y ejecuta en modo release (recomendado para mejor rendimiento):
```bash
cargo run --release
```

O solo compila:
```bash
cargo build --release
```

Y ejecuta el binario generado:
```bash
.\target\release\project.exe
```

##  Características Implementadas

### Rendering
- Raytracing CPU con paralelización (rayon)
- Resolución: 640×480 (optimizada para rendimiento)
- Profundidad de recursión: 2 niveles

### Iluminación
- **Day/Night Cycle**: Sol animado que cambia de posición y color
- **Multi-light Shading**: Sol + 2 luces de relleno + materiales emisivos
- **Shadow Rays**: Sombras duras para todas las luces
- **Materiales Emisivos**: Antorcha de fuego que emite luz

### Materiales (9 diferentes)
1. **Grass** - Pasto con textura
2. **Wood** - Madera del tronco
3. **Brick** - Ladrillos de la casa
4. **Woodhouse** - Madera del techo
5. **Stone** - Piedras dispersas
6. **Water** - Agua con reflexión y transparencia (animada)
7. **Glass** - Vidrio de ventanas con refracción
8. **Fire** - Fuego emisivo (animado)
9. **Steve** - Textura del personaje

### Efectos Físicos
- **Refracción**: Implementada con Ley de Snell y aproximación de Fresnel
- **Reflexión**: En agua y vidrio
- **Texturas Animadas**: Agua y fuego con animación de ondas

### Modelos 3D
- **Carga de OBJ**: Parser completo con soporte para vértices, UVs y caras
- **Steve.obj**: Modelo de personaje
- **Intersección Möller-Trumbore**: Para triángulos

### Entorno
- **Skybox**: Cubemap de 6 caras con texturas
- **Rotación del Mundo**: Control manual del diorama
- **Cámara Orbital**: Control completo de yaw, pitch y distancia

## Estructura del Proyecto

```
Proyecto-2-Raytracing/
├── src/
│   ├── main.rs          # Loop principal y lógica de raytracing
│   ├── camera.rs        # Cámara y generación de rayos
│   ├── ray.rs           # Estructura de rayo
│   ├── cube.rs          # Intersección AABB y UVs
│   ├── mesh.rs          # Parser OBJ e intersección de triángulos
│   ├── material.rs      # Materiales y propiedades físicas
│   ├── light.rs         # Fuentes de luz
│   ├── color.rs         # Manejo de colores
│   ├── texture.rs       # Carga y muestreo de texturas
│   └── skybox.rs        # Cubemap del cielo
├── assets/
│   ├── textures/        # Texturas PNG (grass, wood, brick, etc.)
│   ├── skybox/          # 6 caras del cubemap
│   └── models/          # Archivos OBJ (Steve, sphere)
├── Cargo.toml
└── README.md
```

##  Dependencias

- **raylib**: 5.5.1 - Ventana y manejo de input
- **rayon**: 1.8 - Paralelización del rendering
- **image**: 0.24 - Carga de texturas PNG

## Escena

El diorama incluye:
- Piso de 12×12 bloques de pasto
- Casa de ladrillos con techo de madera y ventanas de vidrio
- Árbol con tronco y hojas
- Estanque de agua (con reflexión y animación)
- Antorcha de fuego emisiva
- 6 rocas dispersas
- Modelo de Steve frente a la casa
