# Libs

Bibliotecas compartidas de CosmicOS.

| Crate | Qué es |
|---|---|
| `cosmic-core` | Tipos y utilidades comunes a los demás componentes |
| `cosmic-usb` | Transporte USB (sobre `rusb`) que comparten `cosmic-fastboot` y `cad`; incluye una parte en C (`csrc/`, compilada desde `build.rs`) |

## Compilar

Se compila desde la raíz del árbol, con el workspace de [Cosmic-Workspace](https://github.com/Open-Cosmic-Project/Cosmic-Workspace):

```sh
cargo check -p cosmic-core
```

## Licencia

GPL-3.0-or-later.
