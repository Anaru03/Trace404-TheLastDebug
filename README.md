```text
████████╗██████╗  █████╗  ██████╗███████╗
╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██╔════╝
   ██║   ██████╔╝███████║██║     █████╗
   ██║   ██╔══██╗██╔══██║██║     ██╔══╝
   ██║   ██║  ██║██║  ██║╚██████╗███████╗
   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚══════╝

              // 404
         THE LAST DEBUG
```

<h1 align="center">TRACE//404: The Last Debug</h1>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-DEA584?style=for-the-badge&logo=rust&logoColor=black" />
  <img src="https://img.shields.io/badge/Ray%20Tracing-CPU-00bcd4?style=for-the-badge" />
  <img src="https://img.shields.io/badge/status-en%20desarrollo-orange?style=for-the-badge" />
</p>

<p align="center">
  <b>Un diorama renderizado mediante ray tracing dentro de una sala de servidores abandonada.</b><br/>
  Una última estación sigue encendida. Algo quedó ejecutándose.
</p>

---

## Sobre el proyecto

**TRACE//404: The Last Debug** es un proyecto de Ray Tracing desarrollado desde cero en Rust para el curso de Gráficas por Computadora.

El proyecto busca construir un diorama inspirado en una estación de trabajo antigua dentro de una sala de servidores, combinando una estética tecnológica oscura con los diferentes conceptos de ray tracing desarrollados durante el curso.

La escena se construye progresivamente a partir de primitivas 3D y actualmente incluye una estación de trabajo con monitor, computadora, teclado, lámpara y racks de servidores.

---

## Estado actual

Actualmente el raytracer cuenta con:

- Generación de rayos primarios
- Vectores y operaciones matemáticas 3D
- Intersección con esferas
- Intersección con cubos
- Selección del impacto más cercano
- Framebuffer propio
- Cámara orbital interactiva
- Iluminación difusa
- Fuente de luz fija
- Escena compuesta por múltiples objetos
- Blockout inicial del diorama
- Renderizado interactivo mediante `minifb`

---

## Controles

| Tecla | Acción |
|:---:|---|
| `←` | Orbitar cámara hacia la izquierda |
| `→` | Orbitar cámara hacia la derecha |
| `↑` | Orbitar cámara hacia arriba |
| `↓` | Orbitar cámara hacia abajo |
| `Esc` | Salir |

---

## Ejecutar

```bash
cargo run
```

Para ejecutar con optimizaciones:

```bash
cargo run --release
```

---

<p align="center">
  <i>It's not a bug. It's a message.</i>
</p>