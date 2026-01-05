
# DSY-RS Series Low Voltage Servo Drive – Parameter List (Chapter 7)
Merged Markdown with explicit numeric-to-value mappings for enumerated parameters (e.g., baud-rate codes).



## Parameter calculation

Address (PXX.YY) = XX * 256 + YY

E.G. : P18.01 = 0x1201

For U16 sending : [value & 0x00ff; (value & 0xff00) >> 8]

---

## P00 – Basic Control Parameters

| Code | Name | Range / Mapping | Min Unit | Default |
|---|---|---|---:|---:|
| P00.00 | Control mode selection | `0 = Position` ; `1 = Speed` ; `2 = Torque` | 1 | 0 |
| P00.01 | Direction of rotation | `0 = CCW is forward` ; `1 = CW is forward` | 1 | 0 |
| P00.02 | Pulse output forward-direction definition | `0 = CCW forward (OA leads OB)` ; `1 = CW forward (OA lags OB)` | 1 | 0 |
| P00.04 | Rigidity level setting | 0–31 | 1 | 11 |
| P00.05 | Inertia ratio | 0–3000 | 0.01 | 100 |
| P00.06 | Absolute value system selection | `0 = Incremental position` ; `1 = Absolute position (linear)` ; `2 = Absolute position (rotation)` | 1 | 0 |
| P00.07 | System maximum speed | 0–10000 rpm | 1 rpm | 4500 |
| P00.10 | Servo OFF stop mode | `0 = Freewheel stop` ; `1 = Stop at zero speed (decel by P05.06)` | 1 | 1 |
| P00.11 | Fault No.1 stop mode selection | `0 = Freewheel stop` ; `1 = Reserved` | 1 | 0 |
| P00.12 | Fault No.2 stop mode selection | `0 = Freewheel stop` ; `1 = Stop at zero speed (decel by P05.06)` | 1 | 1 |
| P00.13 | Stop mode when overtravel | `0 = Freewheel` ; `1 = Decel then servo-lock` ; `2 = Decel then freewheel` | 1 | 1 |
| P00.14 | Brake output ON delay after command | 0–10000 ms | 1 ms | 200 |
| P00.15 | Brake output OFF delay | 10–10000 ms | 1 ms | 200 |
| P00.16 | Speed threshold for brake output OFF (running) | 0–1000 rpm | 1 rpm | 50 |
| P00.17 | No.1 fault in running: delay between servo OFF and brake OFF | 0–10000 ms | 1 ms | 500 |
| P00.18 | Energy consumption resistor setting | `0 = Built-in` ; `1 = External, natural cooling` ; `2 = External, forced air` ; `3 = None (capacitance absorption, brake tube closed)` | 1 | 0 |
| P00.19 | External resistor power capacity | 1–65535 W | 1 W | Model |
| P00.20 | External resistance value | 1–1000 Ω | 1 Ω | Model |
| P00.21 | External resistance heating time constant | 1000–65535 ms | 1 ms | Model |
| P00.22 | Braking start voltage | 0–1000 V | 1 V | Model |
| P00.37 | Pulse increment threshold | 0–200 | 1 | 1 |
| P00.38 | Continuous pulseless reception cycle number | 1–200 | 1 | 3 |

---

## P01 – Servo Motor Parameters

| Code | Name | Range / Mapping | Min Unit | Default |
|---|---|---|---:|---:|
| P01.00 | Motor model code | 0–65535 | 1 | 101 |
| P01.01 | Motor power line phase sequence direction | `0 = CCW` ; `1 = CW` | 1 | 0 |
| P01.02 | Rated voltage | 1–1000 V | 1 | 48 |
| P01.03 | Rated power | 0–65535 kW | 0.01 kW | — |
| P01.04 | Rated current | 1–10000 A | 0.01 A | — |
| P01.05 | Rated torque | 0–65535 Nm | 0.01 Nm | — |
| P01.08 | Max speed | 0–9000 rpm | 1 rpm | — |
| P01.09 | Rotor inertia | 0–10000 | 0.01 kg·cm² | — |
| P01.10 | Pole pairs (PMSM) | 1–50 | 1 | — |
| P01.11 | Stator resistance Rs | 1–65535 | 0.001 Ω | — |
| P01.12 | Q-axis inductance Lq | 1–65535 | 0.01 mH | — |
| P01.13 | D-axis inductance Ld | 1–65535 | 0.01 mH | — |
| P01.14 | Back EMF | 1–65535 | 0.01 mV/rpm | — |
| P01.15 | Torque factor | 1–65535 | 0.001 Nm/A | — |
| P01.18 | Encoder selection | `0 = 2500-line` ; `1 = 17-bit incremental` ; `2 = 17-bit absolute` ; `3 = 23-bit incremental` ; `4 = 23-bit absolute` | — | — |
| P01.20 | Encoder resolution | 1–1,073,741,824 | 1 | — |
| P01.22 | Z electrical angle | 0–3600 | 0.1° | — |
| P01.23 | U rising-edge electrical angle | 0–3600 | 0.1° | — |
| P01.24 | FPGA upload motor model | 0–65535 | — | Read-only |

---

## P02 – Digital Terminal I/O Parameters

| Code | Name | Range / Mapping | Default |
|---|---|---|---:|
| P02.00 | FunINL unassigned state (HEX) | 0–0xFFFF (Bit0=FunIN.1 … Bit15=FunIN.16) | 0 |
| P02.01 | DI1 terminal function selection | `0 = None` ; `1–45 = FunIN.1–45` | 1 |
| P02.02 | DI2 terminal function selection | `0 = None` ; `1–45 = FunIN.1–45` | 2 |
| P02.03 | DI3 terminal function selection | `0 = None` ; `1–45 = FunIN.1–45` | 10 |
| P02.10 | FunINH unassigned state (HEX) | 0–0xFFFF (Bit0=FunIN.17 … Bit15=FunIN.32) | 0 |
| P02.11 | DI1 terminal logic selection | `0=Low` ; `1=High` ; `2=Rising` ; `3=Falling` ; `4=Both edges` | 0 |
| P02.12 | DI2 terminal logic selection | `0=Low` ; `1=High` ; `2=Rising` ; `3=Falling` ; `4=Both edges` | 0 |
| P02.13 | DI3 terminal logic selection | `0=Low` ; `1=High` ; `2=Rising` ; `3=Falling` ; `4=Both edges` | 0 |
| P02.21 | DO1 terminal function selection | `0 = None` ; `1–25 = FunOUT.1–25` | 1 |
| P02.22 | DO2 terminal function selection | Must be `11 = Lock release signal output` | 11 |
| P02.31 | DO1 terminal logic (polarity invert) | `0 = NO (conduct when active)` ; `1 = NC (open when active)` | 0 |
| P02.32 | DO2 terminal logic (polarity invert) | `0 = NO` ; `1 = NC` | 0 |

---

## P04 – Position Control Parameters

| Code | Name | Range / Mapping | Unit | Default |
|---|---|---|---|---:|
| P04.00 | Main position command A source | `0=Low-speed pulse` ; `1=High-speed pulse` ; `2=Step amount` ; `4=Multi-segment position` ; `5=Communication` | — | 0 |
| P04.02 | Step amount | −9999…9999 | unit | 50 |
| P04.03 | Position command smoothing filter | 0–65535 | 0.1 ms | 0 |
| P04.04 | Position command FIR filter | 0–1280 | 0.1 ms | 0 |
| P04.05 | Units required for one revolution (32-bit, PTP only) | 16–1,073,741,824 | unit/turn | 0 |
| P04.07 | Electronic gear 1 numerator (32-bit) | 1–1,073,741,824 | 1 | Motor resolution |
| P04.09 | Electronic gear 1 denominator (32-bit) | 1–1,073,741,824 | 1 | 10000 |
| P04.11 | Electronic gear 2 numerator (32-bit) | 1–1,073,741,824 | 1 | Motor resolution |
| P04.13 | Electronic gear 2 denominator (32-bit) | 1–1,073,741,824 | 1 | 10000 |
| P04.21 | Pulse shape | `0=Pulse+Dir, +logic` ; `1=Dir+Pulse, −logic` ; `2=A/B quad, +logic` ; `3=A/B quad, −logic` ; `4=CCW/CW, +logic` ; `5=CCW/CW, −logic` | — | 0 |
| P04.22 | Position deviation clear | `0=Clear on fault or servo OFF` ; `1=Clear only on fault` ; `2=Clear by DI (PERR-CLR)` | — | 0 |
| P04.23 | COIN output condition | `0=|dev|<range` ; `1=|dev|<range AND filtered cmd=0` ; `2=|dev|<range AND cmd=0` | — | 0 |
| P04.24 | Positioning completion range | 1–65535 | pulse | motor-dependent |
| P04.25 | Positioning close range | 1–65535 | pulse | 65535 |

---

## P05 – Speed Control Parameters

| Code | Name | Range / Mapping | Unit | Default |
|---|---|---|---|---:|
| P05.00 | Main speed command A source | `0=Digit value (P05.03)` ; `1=Reserved` ; `2=Reserved` | — | 0 |
| P05.01 | Auxiliary speed command B source | `0=Digit value (P05.03)` ; `1=Reserved` ; `2=Reserved` ; `3=Multi-speed command` | — | 3 |
| P05.02 | Speed command selection | `0=Main A` ; `2=Aux B` ; `3=A/B switching` | — | 0 |
| P05.03 | Speed command keyboard setting | −9000…9000 | rpm | 200 |
| P05.04 | Jog speed setting | 0–9000 | rpm | 200 |
| P05.05 | Acceleration time | 0–10000 | ms | 50 |
| P05.06 | Deceleration time | 0–10000 | ms | 50 |
| P05.07 | Speed limit selection | `0=Use P05.08/P05.09 (default)` | — | 0 |
| P05.08 | Forward speed limit | 0–9000 | rpm | 6000 |
| P05.09 | Backward speed limit | 0–9000 | rpm | 6000 |
| P05.14 | Speed direction selection | `0=Unchanged` ; `1=Reversed` ; `2=By DI func 25` ; `3=By DI func 40/41` | — | 2 |
| P05.15 | Zero fixed speed value | 0–6000 | rpm | 10 |
| P05.16 | Motor running signal speed threshold | 0–1000 | rpm | 20 |
| P05.17 | Speed uniform signal width | 0–100 | rpm | 10 |
| P05.18 | Speed reaches specified value | 0–6000 | rpm | 1000 |
| P05.20 | Zero-speed judgment threshold | 0–6000 | rpm | 10 |

---

## P06 – Torque Control Parameters

| Code | Name | Range / Mapping | Unit | Default |
|---|---|---|---|---:|
| P06.00 | Main torque command A source | `0=Digit (P06.05)` ; `1=Reserved` | — | 0 |
| P06.02 | Torque command selection | `0=A` ; `1=B` ; `2=A+B` ; `3=A/B switch` | — | 0 |
| P06.04 | Torque command filter time (torque mode) | 0–65535 | 0.01 ms | 0 |
| P06.05 | Torque command keyboard setting | −3000…3000 (rated torque basis) | 0.1% | 0 |
| P06.06 | Torque limit source | `0=Internal ± limit` ; `1=External ± limit (P_CL/N_CL)` | — | 0 |
| P06.08 | Forward internal torque limit | 0–5000 | 0.1% | 3000 |
| P06.09 | Backward internal torque limit | 0–5000 | 0.1% | 3000 |
| P06.10 | Forward external torque limit | 0–5000 | 0.1% | 3000 |
| P06.11 | Backward external torque limit | 0–5000 | 0.1% | 3000 |
| P06.13 | Speed limit source (torque control) | `0=Internal (P06.15/P06.16)` ; `1=Reserved` | — | 0 |
| P06.15 | Positive speed limit in torque mode | 0–9000 | rpm | 3000 |
| P06.16 | Negative speed limit in torque mode | 0–9000 | rpm | 3000 |
| P06.21 | Multi-segment torque command 1 | −3000…3000 | 0.1% | 0 |
| P06.22 | Multi-segment torque command 2 | −3000…3000 | 0.1% | 0 |
| P06.23 | Multi-segment torque command 3 | −3000…3000 | 0.1% | 0 |

---

## P07 – Gain Parameters (selected)

| Code | Name | Range / Mapping | Unit | Default |
|---|---|---|---|---:|
| P07.00 | Position loop gain 1 | 10–20000 | 0.1 Hz | 320 |
| P07.01 | Speed loop gain 1 | 10–20000 | 0.1 Hz | 180 |
| P07.02 | Speed loop integral time 1 | 15–512 | 0.01 ms | 3100 |
| P07.03 | Speed detection filter 1 | 0–200 | 0.01 ms | 20 |
| P07.05 | Position loop gain 2 | 10–20000 | 0.1 Hz | 380 |
| P07.06 | Speed loop gain 2 | 10–20000 | 0.1 Hz | 180 |
| P07.10 | GAINSWITCH action select | `0=PI/P switch (gain fixed group 1)` ; `1=Gain1/Gain2 switching` | — | 0 |
| P07.11 | Gain switching mode | `0=Gain1 fixed` ; `1=Gain2 fixed` ; `2=By DI` ; `3=Torque cmd>` ; `4=Speed cmd change>` ; `5=Speed cmd>` ; `6=Pos dev>` ; `7=Receive pos cmd` ; `8=Not positioned` ; `9=Actual speed>` ; `10=Receive pos cmd AND actual speed` ; `11=Speed loop PDFF` ; `13=Improved PI` | — | 0 |

---

## P08 – Advanced Adjustment Parameters (selected)

| Code | Name | Range / Mapping | Unit | Default |
|---|---|---|---|---:|
| P08.00 | Adaptive filter mode | 0–5 | — | 0 |
| P08.02 | 1st notch filter frequency | 10–4000 | Hz | 4000 |
| P08.03 | 1st notch filter width | 0–8 | — | 8 |
| P08.04 | 1st notch filter depth | 0–100 | — | 50 |
| P08.15 | Damping filter switch | `0=OFF` ; `1=ON` | — | 0 |
| P08.17 | Damping filter selection | `0=Filter A` ; `1=Filter B` | — | 1 |
| P08.23 | Inertia identification mode | `0=Offline triangle (+/−)` ; `1=Offline JOG` | — | 0 |
| P08.26 | HF vibration suppression switch | `0=OFF` ; `1=ON` | — | 0 |
| P08.33 | Anti-disturbance compensation | `0=OFF` ; `1=ON` | — | 0 |
| P08.39 | Momentary speed compensation | `0=OFF` ; `1=ON` | — | 0 |
| P08.45 | Model compensation switch | `0=Off` ; `1=Rigid model` ; `2=2nd-order vector model` | — | 0 |

---

## P09 – Failure & Protection (selected)

| Code | Name | Range / Mapping | Unit | Default |
|---|---|---|---|---:|
| P09.02 | Undervoltage detection delay | 100–20000 | 0.1 ms | 700 |
| P09.04 | Out-of-control protection | `0=Open` ; `1=Off` | — | 0 |
| P09.05 | Overload warning value | 1–100 | % | 90 |
| P09.06 | Motor overload factor | 10–300 | % | 100 |
| P09.07 | Undervoltage protection point | 50–100 (100=default) | % | 100 |
| P09.08 | Overspeed fault point | 50–120 (100=max speed) | % | 120 |
| P09.09 | Position deviation excessive threshold (32-bit) | 1–1,073,741,824 | pulse | motor-dependent |
| P09.24 | Locked-rotor over-temp enable | `0=Disable` ; `1=Enable` | — | 0 |
| P09.25 | Overload protection enable | `0=Motor overload + avg-load overload` ; `1=Motor overload only` ; `2=Avg-load only` ; `3=Disable both` | — | 0 |

---

## P10 – Communication Parameters (with numeric → value mapping)

| Code | Name | Range / Mapping | Default |
|---|---|---:|---:|
| P10.00 | Communication address | 0–247 (0 = broadcast) | 1 |
| P10.02 | Modbus baud rate setting | `0=2400` ; `1=4800` ; `2=9600` ; `3=19200` ; `4=38400` ; `5=57600` ; `6=115200` | 6 |
| P10.03 | Modbus data format | `0=No parity, 2 stop` ; `1=Even, 1 stop` ; `2=Odd, 1 stop` ; `3=No parity, 1 stop` | 0 |
| P10.04 | Write comm params to EEPROM | `0=No` ; `1=Yes (except P11 & P18)` | 0 |
| P10.05 | RS232 baud rate setting | `0=2400` ; `1=4800` ; `2=9600` ; `3=19200` ; `4=38400` ; `5=57600` ; `6=115200` | 6 |
| P10.06 | RS485 address source | `0=DIP switch` ; `1=Host setting` | 0 |

---

## P11 – Auxiliary Function Parameters

| Code | Name | Range / Mapping | Default |
|---|---|---|---:|
| P11.01 | Fault reset | `0=None` ; `1=Reset` | 0 |
| P11.02 | Soft reset | `0=None` ; `1=Reset` | 0 |
| P11.03 | Inertia recognition function | Enter to execute | 0 |
| P11.06 | Absolute encoder reset | `0=None` ; `1=Clear warnings/errors` ; `2=Reset multi-turn data` | 0 |
| P11.07 | Absolute system soft limit set | `0=None` ; `1=Set current pos as negative limit` ; `2=Set current pos as positive limit` | 0 |
| P11.09 | System initialization | `0=None` ; `1=Factory reset (except P01 & P17)` ; `2=Clear fault record` | 0 |
| P11.10 | Forced DIDO enable | `0=None` ; `1=Force DI` ; `2=Force DO` ; `3=Force DI&DO` | 0 |
| P11.11 | Set DI forced input | 0–0x01FF | 511 |
| P11.12 | Set DO forced output | 0–0x001F | 0 |
| P11.13 | Emergency stop settings | `0=None` ; `1=Emergency stop` | 0 |

---

## P12 – Keyboard Display Parameters (selected)

| Code | Name | Range / Mapping | Default |
|---|---|---|---:|
| P12.00 | LED warning display selection | `0=Show warnings` ; `1=Do not show warnings` | 0 |
| P12.01 | Default display settings | 0–100 | 1 |
| P12.03 | Speed display filter time | 0–10000 (0.1 ms) | 50 |
| P12.11 | Non-standard version number | Display only | — |
| P12.12 | Software version number | Display only | — |
| P12.13 | FPGA version number | Display only | — |
| P12.14 | Product series code | Display only | — |

---


# DSY‑RS Series Servo Drive – Parameter Tables
## Groups: P13, P14, P16, P18

---

## P13 – Multi‑Segment Position Parameters

### Common Settings
| Code | Name | Range | Unit | Default |
|-----|------|-------|------|---------|
| P13.00 | Operation mode | 0: Single, 1: Cycle, 2: DI switch | — | 1 |
| P13.01 | Start segment | 1–16 | — | 1 |
| P13.02 | End segment | 1–16 | — | 16 |
| P13.03 | Interrupt handling | 0: Continue, 1: Restart | — | 0 |
| P13.04 | Wait time unit | 0: ms, 1: s | — | 0 |
| P13.05 | Position mode | 0: Incremental, 1: Absolute | — | 0 |

### Segment Parameters (Segments 1–16)
Each segment has the same structure.

| Segment | Displacement (32‑bit) | Max Speed (rpm) | Acc/Dec Time (ms) | Wait Time (ms/s) |
|--------|----------------------|-----------------|-------------------|------------------|
| 1 | P13.08 | P13.10 | P13.11 | P13.12 |
| 2 | P13.13 | P13.15 | P13.16 | P13.17 |
| 3 | P13.18 | P13.20 | P13.21 | P13.22 |
| 4 | P13.23 | P13.25 | P13.26 | P13.27 |
| 5 | P13.28 | P13.30 | P13.31 | P13.32 |
| 6 | P13.33 | P13.35 | P13.36 | P13.37 |
| 7 | P13.38 | P13.40 | P13.41 | P13.42 |
| 8 | P13.43 | P13.45 | P13.46 | P13.47 |
| 9 | P13.48 | P13.50 | P13.51 | P13.52 |
|10 | P13.53 | P13.55 | P13.56 | P13.57 |
|11 | P13.58 | P13.60 | P13.61 | P13.62 |
|12 | P13.63 | P13.65 | P13.66 | P13.67 |
|13 | P13.68 | P13.70 | P13.71 | P13.72 |
|14 | P13.73 | P13.75 | P13.76 | P13.77 |
|15 | P13.78 | P13.80 | P13.81 | P13.82 |
|16 | P13.83 | P13.85 | P13.86 | P13.87 |

Displacement range: −1,073,741,824 ~ +1,073,741,824 (1 unit)  
Speed range: 0–9000 rpm

---

## P14 – Multi‑Speed Parameters

### Common Settings
| Code | Name | Range | Default |
|-----|------|-------|---------|
| P14.00 | Operation mode | 0: Single, 1: Cycle, 2: DI | 1 |
| P14.01 | End segment | 1–16 | 16 |
| P14.02 | Time unit | 0: s, 1: min | 0 |
| P14.03–06 | Acc/Dec time 1–4 | 0–10000 ms | 0 |

### Speed Segments (1–16)

| Segment | Speed (rpm) | Run Time | Acc/Dec Select |
|--------|-------------|----------|----------------|
| 1 | P14.07 | P14.08 | P14.09 |
| 2 | P14.10 | P14.11 | P14.12 |
| 3 | P14.13 | P14.14 | P14.15 |
| 4 | P14.16 | P14.17 | P14.18 |
| 5 | P14.19 | P14.20 | P14.21 |
| 6 | P14.22 | P14.23 | P14.24 |
| 7 | P14.25 | P14.26 | P14.27 |
| 8 | P14.28 | P14.29 | P14.30 |
| 9 | P14.31 | P14.32 | P14.33 |
|10 | P14.34 | P14.35 | P14.36 |
|11 | P14.37 | P14.38 | P14.39 |
|12 | P14.40 | P14.41 | P14.42 |
|13 | P14.43 | P14.44 | P14.45 |
|14 | P14.46 | P14.47 | P14.48 |
|15 | P14.49 | P14.50 | P14.51 |
|16 | P14.52 | P14.53 | P14.54 |

Speed range: −9000 ~ +9000 rpm  
Run time: 0–65535 (0.1 s or min)

---

## P16 – Special Function Parameters

| Code | Name | Range | Unit | Default |
|-----|------|-------|------|---------|
| P16.00 | Fixed length interrupt enable | 0–1 | — | 0 |
| P16.01 | Fixed length 1 displacement | 0–2³⁰ | unit | 10000 |
| P16.03 | Fixed length 1 speed | 0–9000 | rpm | 200 |
| P16.04 | Fixed length accel time | 0–1000 | ms | 200 |
| P16.05 | Fixed length decel time | 0–1000 | ms | 200 |
| P16.06 | Lock release enable | 0–1 | — | 1 |
| P16.08 | Homing enable mode | 0–6 | — | 0 |
| P16.09 | Homing mode | 0–17 | — | 0 |
| P16.10 | Homing high speed | 10–3000 | rpm | 100 |
| P16.11 | Homing low speed | 10–1000 | rpm | 10 |
| P16.12 | Homing accel limit | 0–65535 | ms | 1000 |
| P16.13 | Homing timeout | 0–65535 | ms | 10000 |
| P16.14 | Mechanical home offset | ±2³⁰ | unit | 0 |
| P16.28 | Absolute encoder origin | 0–2³² | inc | 0 |
| P16.30 | Encoder turns at origin | 0–32767 | turn | 0 |
| P16.31 | Zero wait count | 0–65535 | ms | 500 |
| P16.37 | Fixed length 2 displacement | ±2³⁰ | unit | 0 |
| P16.39 | Fixed length 2 speed | 0–9000 | rpm | 200 |

---

## P18 – Display Parameters (Read‑Only)

| Code | Description | Range | Unit |
|-----|------------|-------|------|
| P18.00 | Servo status | Ready / Run / Err / AL | — |
| P18.01 | Motor speed feedback | ±9000 | rpm |
| P18.02 | Average load rate | 0–3000 | 0.1 % |
| P18.03 | Speed command | ±9000 | rpm |
| P18.04 | Internal torque | ±5000 | 0.1 % |
| P18.05 | Phase current RMS | 0–10000 | 0.01 A |
| P18.06 | DC bus voltage | 0–10000 | 0.1 V |
| P18.07 | Absolute position | ±2³⁰ | unit |
| P18.09 | Electrical angle | 0–3600 | 0.1° |
