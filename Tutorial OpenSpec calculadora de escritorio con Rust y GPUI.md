# Tutorial OpenSpec: calculadora de escritorio con Rust y GPUI

Guía práctica para aprender OpenSpec construyendo una calculadora de escritorio en Rust con GPUI, trabajando desde Zed. Basada en la documentación oficial de openspec.dev (v1.16.0 al momento de escribirla).

## 1. La idea en 30 segundos

OpenSpec es una capa ligera de acuerdo entre tú y tu agente de IA: primero acuerdan **qué** construir en archivos Markdown y después el agente escribe el código. Todo vive en tu repositorio, en la carpeta `openspec/`.

Cada cambio sigue el mismo ciclo de cinco pasos:

1. **Explore**: piensas el problema con el agente (no escribe código).
2. **Propose**: el agente redacta el plan en una carpeta de cambio.
3. **Review**: tú corriges el plan, mientras todavía son solo palabras.
4. **Apply**: el agente implementa tarea por tarea.
5. **Archive**: las specs del proyecto absorben lo que se construyó.

Lo que vas a construir aquí: una calculadora con un motor de cálculo testeable, una ventana GPUI y entrada por teclado, en **cambios pequeños** (uno por capacidad). Eso es lo que mantiene el consumo de tokens bajo.

## 2. Instalación

Opción recomendada (Linux/macOS), instalador independiente:

```
curl -fsSL https://openspec.dev/cli/install.sh | sh
```

Alternativa con npm (requiere Node 22.12.0 o superior):

```
npm install -g @fission-ai/openspec@latest
```

Comprueba que funciona:

```
openspec --version
```

Se instala una sola vez por máquina. Para refrescar los archivos generados en cada proyecto, ejecuta `openspec update` dentro de él.

## 3. Crear el proyecto Rust

```
cargo new calculadora
cd calculadora
git init
```

Sobre GPUI: es el framework de interfaz de Zed y sigue en desarrollo activo. Su forma de dependencia ha cambiado con el tiempo (dependencia git del repositorio de Zed, un crate `gpui` 0.2.x en crates.io y snapshots de terceros como `gpui-pre`). **No lo decidas ahora de memoria**: lo resolveremos en el paso de explore y quedará fijado en `design.md`.

## 4. Inicializar OpenSpec

En la raíz del proyecto:

```
openspec init
```

El asistente pregunta qué herramientas de IA usas. Para Zed, elige la entrada de **Zed Agent / Other (Universal)**. Según la documentación, Zed Agent comparte la carpeta `.agents/skills/` con el objetivo universal, así que no hay comandos propios de Zed: se usan *skills*.

También puedes saltarte el selector con:

```
openspec init --tools agents
```

Qué crea:

```
openspec/
├── config.yaml     ajustes y contexto para la IA
├── specs/          tus specs (vacío por ahora)
└── changes/        cambios en curso
    └── archive/    cambios terminados
.agents/skills/     los flujos de trabajo (explore, propose, apply...)
```

Haz commit de todo, igual que el resto del código. Reiniciar Zed después del init ayuda a que detecte las skills.

**Cómo invocar los flujos en Zed:** al escribir `/ope` en el panel del agente, Zed muestra las skills como comandos: `/openspec-explore`, `/openspec-propose`, `/openspec-apply-change`, `/openspec-update-change`, `/openspec-sync-specs` y `/openspec-archive-change`. Escríbelos seguidos de tu petición, por ejemplo `/openspec-propose add the calculator engine`. También funciona el lenguaje natural (*"Propose a change to add the calculator engine"*). Con el objetivo universal no se generan comandos `/opsx:...`.

| Comando | Cuándo usarlo |
| --- | --- |
| `/openspec-explore <idea>` | Pensar el problema antes de planificar (no escribe código) |
| `/openspec-propose <idea>` | Generar la carpeta del cambio: proposal, specs, design y tasks |
| `/openspec-update-change <cambio>` | Revisar el plan de un cambio ya propuesto |
| `/openspec-apply-change <cambio>` | Implementar las tareas de `tasks.md` (en sesión nueva) |
| `/openspec-sync-specs <cambio>` | Pasar las specs del cambio a `openspec/specs/` sin archivarlo |
| `/openspec-archive-change <cambio>` | Cerrar el cambio: fusiona las specs y lo mueve al archivo |

## 5. Configurar `config.yaml` (tokens, Rust y pruebas)

Es el archivo más rentable. Todo lo que pongas aquí llega al contexto del agente, así que **mantenlo corto**: las reglas largas pueden empeorar el resultado.

Tiene tres campos:

| Campo | Para qué sirve | Dónde se inyecta |
| --- | --- | --- |
| `context` | Hechos del proyecto que el agente siempre recibe | Todo: artefactos, apply, archive |
| `rules` | Reglas para un artefacto concreto | Solo al crearlo |
| `operations` | Cómo ejecutar `apply` y `archive` | Solo apply y archive |

Conserva las líneas que generó `init` (por ejemplo `schema: spec-driven`) y añade esto:

```yaml
context: |
  App de escritorio en Rust: calculadora con interfaz GPUI.
  La lógica de cálculo va en un módulo propio, sin depender de GPUI, para probarla con cargo test.
  Escribe todos los artefactos en español y sé breve.
rules:
  proposal:
    - Máximo 200 palabras
  design:
    - Solo decisiones no obvias; no repitas la propuesta
  tasks:
    - Cada tarea indica su verificación (cargo test, cargo clippy o comportamiento observable)
operations:
  apply:
    guidance:
      - Ejecuta cargo test y cargo clippy antes de marcar una tarea como hecha
      - No modifiques código fuera del alcance de la tarea
```

Esto responde directamente a tus objetivos: **pruebas** (cada tarea declara cómo se verifica y apply ejecuta los tests), **control** (alcance acotado por tarea) y **pocos tokens** (propuestas cortas, sin repetición).

Los cambios en `config.yaml` llegan al agente en la siguiente ejecución, sin reinstalar nada.

## 6. Cambio 1: el motor de cálculo

### 6.1 Explore

En el panel del agente de Zed:

```
/openspec-explore Help me explore a desktop calculator in Rust with GPUI. Compare how to depend on GPUI today and help me choose the first behavior to build. Keep the calculation engine independent from the UI.
```

Explore es un modo de pensar: el agente investiga, pregunta, compara opciones y cuestiona supuestos, pero no escribe código. Aquí decidirás cómo depender de GPUI y que el primer cambio sea solo el motor. Cuando la forma esté clara:

```
/openspec-propose the change we just discussed
```

Si el cambio ya lo tienes claro, salta explore y escribe directamente /openspec-propose add the calculator engine, o en lenguaje natural: *"Propose a change to add the calculator engine"*. Explore gasta tokens; úsalo cuando haya dudas reales, como la dependencia de GPUI.

### 6.2 Qué genera propose

```
openspec/changes/add-calculator-engine/
├── .openspec.yaml
├── proposal.md    por qué y qué cambia
├── specs/
│   └── calculator-engine/
│       └── spec.md    qué significa "terminado", como requisitos comprobables
├── design.md      solo si el cambio lo necesita
└── tasks.md       la lista de implementación
```

No hay código todavía. Propose se detiene en el plan.

### 6.3 Ejemplo de spec (delta)

La spec describe **comportamiento observable**, no nombres de funciones ni de librerías. Prueba rápida: si puedes cambiar la implementación sin cambiar lo que ve el usuario, no pertenece a la spec.

```
# Spec Delta

## Purpose

Evalúa operaciones aritméticas básicas y devuelve un resultado o un error claro.

## ADDED Requirements

### Requirement: Operaciones básicas
El sistema SHALL calcular suma, resta, multiplicación y división sobre números decimales.

#### Scenario: Suma simple
- **WHEN** se evalúa "2 + 3"
- **THEN** el resultado es 5

#### Scenario: Decimales
- **WHEN** se evalúa "0.5 * 4"
- **THEN** el resultado es 2

### Requirement: Precedencia de operadores
El sistema SHALL aplicar multiplicación y división antes que suma y resta.

#### Scenario: Multiplicación primero
- **WHEN** se evalúa "2 + 3 * 4"
- **THEN** el resultado es 14

### Requirement: División entre cero
El sistema SHALL devolver un error de división entre cero sin terminar el programa.

#### Scenario: División entre cero
- **WHEN** se evalúa "5 / 0"
- **THEN** se devuelve un error de división entre cero
```

Reglas de formato que importan:

- Los escenarios usan **exactamente cuatro `#`** (`####`). Con tres o con viñetas fallan en silencio.
- Cada requisito necesita al menos un escenario, con WHEN y THEN.
- Usa SHALL o MUST en los requisitos; evita "debería" o "podría".
- Un requisito = un comportamiento. Los casos límite van en escenarios.
- Un escenario es un caso de prueba potencial: de aquí salen tus tests.

### 6.4 Ejemplo de `tasks.md`

```
# Tasks

## 1. Motor de cálculo

- [ ] 1.1 Crear el módulo del motor, sin dependencia de GPUI, y verificar que cargo build compila
- [ ] 1.2 Implementar el evaluador con precedencia y verificar con tests de los escenarios "Suma simple", "Decimales" y "Multiplicación primero"
- [ ] 1.3 Manejar la división entre cero y verificar con el test del escenario correspondiente
```

OpenSpec pide que cada tarea indique cómo se verifica y que los tests se escriban en el mismo grupo que el código, no al final. Los checkboxes de `tasks.md` son el único registro de progreso: no hay estado oculto.

## 7. Revisar y corregir el plan

Este es tu momento de máximo control: corregir cuesta palabras, no código. Lee en este orden:

1. `proposal.md`: ¿es el problema correcto y del tamaño correcto?
2. `specs/`: la lectura más valiosa. ¿Aceptarías estos requisitos como "terminado"?
3. `tasks.md`: ¿cubren las specs y nada más?

Para arreglar algo tienes dos caminos: edita el archivo tú mismo (es Markdown plano) o dile al agente qué falta (por ejemplo, *"falta el caso de números negativos"*) y él revisa los artefactos; para eso existe /openspec-update-change.

Puedes validar el formato con:

```
openspec validate
```

Con `--strict` también falla ante advertencias (escenarios sin WHEN/THEN, descripciones de más de 500 caracteres).

## 8. Apply

Abre una **sesión nueva** del agente (la implementación funciona mejor con el contexto limpio) y pide:

```
/openspec-apply-change add-calculator-engine
```

El agente lee la carpeta del cambio y avanza por `tasks.md`, marcando cada tarea al terminarla. Gracias a tu `operations.apply`, ejecutará `cargo test` y `cargo clippy` antes de marcar.

- **¿Se cortó o se acabó el contexto?** Abre otra sesión y pide apply otra vez: retoma en la primera tarea sin marcar.
- **¿El plan estaba mal?** Corrige los artefactos y sigue con apply.

## 9. Archive

Cuando todas las casillas estén marcadas:

```
/openspec-archive-change add-calculator-engine
```

Archive hace dos cosas: fusiona los requisitos del cambio en `openspec/specs/` y mueve la carpeta del cambio a `openspec/changes/archive/` con prefijo de fecha. Después, `specs/` describe el sistema tal como está construido y `archive/` guarda el historial.

Haz commit de la carpeta del cambio junto con el código.

## 10. Los siguientes cambios

Repite el ciclo con cambios pequeños. Un plan posible:

| Cambio | Capacidad | Idea de prompt |
| --- | --- | --- |
| `add-calculator-window` | `calculator-ui` | */openspec-propose show a GPUI window with a display and a button grid wired to the engine* |
| `add-keyboard-input` | `keyboard-input` | */openspec-propose make digits, operators, Enter and Escape work from the keyboard* |
| `add-calculation-history` | `calculation-history` | */openspec-propose keep a list of past calculations in the window* |

Al modificar una capacidad existente, la spec usa `## MODIFIED Requirements` y debes copiar el bloque **completo** del requisito (desde `### Requirement:` hasta sus escenarios) y editarlo. Si solo añades comportamiento nuevo, usa `ADDED`. Un `MODIFIED` con contenido parcial pierde detalle al archivar.

## 11. Cómo gastar menos tokens

- **Un cambio = una capacidad pequeña.** Cambios grandes producen specs y tareas largas.
- **Sesión nueva para cada apply.** Evitas arrastrar toda la conversación de planificación.
- **`config.yaml` corto.** Todo lo que pongas se envía en cada ejecución.
- **Salta `design.md` cuando no haga falta.** El agente solo lo crea si hay decisiones de arquitectura, dependencias nuevas o ambigüedad.
- **No uses explore para cambios triviales.** Pide propose directo.
- **Lee specs de forma filtrada.** `openspec list --specs` lista las capacidades existentes y `openspec show <spec-id> --type spec --json --no-scenarios` devuelve solo propósito y requisitos, sin cargar el archivo completo.
- **Revisa tú mismo los artefactos.** Corregir una línea a mano es más barato que pedirle una regeneración.

Si usas un modelo local con contexto limitado, esto pesa aún más: tareas pequeñas y specs concisas son lo que mejor funciona.

## 12. Comandos de la CLI que usarás

| Comando | Para qué |
| --- | --- |
| `openspec init` | Añadir OpenSpec a un proyecto |
| `openspec update` | Refrescar skills y comandos generados (tras actualizar la CLI) |
| `openspec list` | Cambios en curso |
| `openspec list --specs` | Capacidades ya especificadas |
| `openspec show <id>` | Ver un cambio o una spec |
| `openspec validate [--strict]` | Comprobar el formato de los artefactos |
| `openspec config profile` | Elegir qué flujos se instalan y cómo |

El flujo **verify** (comprobar que la implementación coincide con la spec) no viene en el perfil por defecto; se añade con `openspec config profile` y luego `openspec update`. Para tu objetivo de tener pruebas, es una buena opción cuando el proyecto crezca.

## 13. Errores frecuentes

- **Escenarios con `###` o con viñetas:** deben ser `####`; si no, se ignoran sin avisar.
- **Spec fuera de su sitio:** debe ser `specs/<capacidad>/spec.md`. Un archivo como `specs/calculadora.md` no se fusiona al archivar.
- **Nombres de librerías en la spec:** GPUI va en `design.md`, no en los requisitos.
- **Capacidades con nombres de tarea:** usa `calculator-engine`, no `add-engine`.
- **No ves los flujos en Zed:** ejecuta `openspec update`, reinicia Zed y escribe /ope para comprobar que aparecen; si no, pide el flujo en lenguaje natural.
- **Cambio sin cambios de comportamiento** (refactor, tooling, documentación): marca `skip_specs: true` en el `.openspec.yaml` del cambio, porque `validate` rechaza un cambio sin deltas.

## 14. Hoja de referencia

```
# una vez por máquina
curl -fsSL https://openspec.dev/cli/install.sh | sh

# una vez por proyecto
openspec init --tools agents

# por cada cambio, en el agente de Zed
1. Explore  -> /openspec-explore <idea>           (opcional)
2. Propose  -> /openspec-propose <idea>
3. Revisa proposal.md, specs/, tasks.md y corrige
4. Apply    -> /openspec-apply-change <cambio>   (sesión nueva)
5. Archive  -> /openspec-archive-change <cambio>
```

Documentación oficial: openspec.dev/docs (Installation, Set up your project, Quickstart, Project configuration, Supported tools).
