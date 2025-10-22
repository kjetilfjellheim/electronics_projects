# Passive lowpass filter

## Description
The goal is to implement, analyze and do practical measurements different lowpass filters.

## Goals
- [x] Document schematic in Kicad
- [x] Do a theoretical analyis 
- [ ] Implement schematic on breadboard
- [ ] Test circuit and do measurements

## Equipment used
| Equipment | Description |
| :------------- | :------------- |
| Peaktech 4055 | AC power supply |
| RS Pro RSDS 1204X-E | Oscilloscope |

## Components
| Reference | Value | Remarks |
| :------------- | :------------- | :------------- |
| 5x capacitor | 22n | |
| 5x resistor | 1k | |
| J1 | ~5V | |

## Formulas
Gain is calculated with this formula.
```math
Gain_db = 20 * log(\frac{V_out}{V_in})
```

## Circuits
Circuits for 1st to 5th order.
<img src="./schematics/rc_filter.svg">

## Simulations
Gain for 1st to 5th order filter
<img src="./images/rc_lowpass_filter_gain_simulation.png">

Phase for 1st to 5th order filter
<img src="./images/rc_lowpass_filter_phase_simulation.png">

Simulation values
| Order | -3db | -20db | -40db |
| :------------- | :------------- | :------------- | :------------- |
| 1 | 7.19KHz | 71.9KHz | Not measureable |
| 2 | 2.69KHz | 19.2KHz | 71.2KHz |
| 3 | 1.40KHz | 9.82KHz | 30.2KHz |
| 4 | 852Hz | 6.12KHz | 18.2KHz |
| 5 | 575Hz | 4.10KHz | 12.3KHz |

## Calculations
Assuming input voltage 5V
| Gain | Voltage |
| :------------- | :------------- |
| -3db | 3.53V |
| -10db | 1.58V |
| -20db | 0.5V |
| -40db | 0.25V |

## Practical measurements
