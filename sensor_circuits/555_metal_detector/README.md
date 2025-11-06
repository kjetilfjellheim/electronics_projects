# 555 metal detector

## Description
The 555 will oscillate based on the LCR circuit. When metal comes in contact with the inductor the oscillation will change due to it affecting the
magnetic field around the inductor. The output is the sent to the speaker which will change tone when metallic (Magnetic) objects comes near.

This is not a particularly powerful metal detector, for that a greater voltage/current is requred for the inductor so that it generates a larger magnetic field.
Using a coil rather than a simple inductor would also make the metal detector better as it would cover a larger area.

## Source
Original source: Unknown

## Goals
- [x] Document schematic in Kicad
- [x] Do a theoretical analyis 
- [x] Implement schematic on breadboard
- [x] Test circuit and do measurements

## Schematics
Kicad file: schematics/schematics.kicad_sch
<img src="./schematics/schematics.svg">

## Breadboard
Breadboard setup
<img src="./images/breadboard.jpg">

## Equipment used
| Equipment | Description |
| :------------- | :------------- |
| Peaktech 6075 | DC power supply |
| RS Pro RSDS 1204X-E | Oscilloscope |
| Peak Atlas LCR45 | LCR meter |

## Components
| Reference | Value | Remarks |
| :------------- | :------------- | :------------- |
| C1,C3 and C4 | 2.2u | |
| C2 | 450p | |
| J1 | 5V | |
| L1 | 10m | |
| R1 | 91k | |
| R2 | 12 | | 
| Speaker | 8 ohm | |
| U1 | 7555 https://www.alldatasheet.com/datasheet-pdf/pdf/17796/PHILIPS/ICM7555.html |  555 should also work fine |

I have tried this circuit with voltage levels from 3V to 9V. It should work above this.

## Measurements
Power supply settings
| Settings | Value |
| :------------- | :------------- |
| Voltage | 5V |
* Reported current by power supply 4mA.

### With no metallic object near inductor
Probe 1: Vspeaker
Probe 2: VL1
Probe 3: Vc4
Probe 4: Vc2
<img src="./images/oscilloscope.png">

| Measurements | Value | Remark | 
| :------------- | :------------- | :--- |
| L1 | 9.588mH | Measured using Peak atlas LCR45 outside of circuit | 
| Fspkr | 509Hz | |
| Vspkr Pk-Pk | 3.92V | |
| Vspkr Vmax | 3.88V | |
| Vspkr Vmin | -40mV | |
| Spkr -Duty | 10.74% | |
| Spkr +Duty | 89.26% | |
| VL1 Pk-Pk | 8.08V | |
| VL1 Vmax | 4.88V | |
| VL1 Vmin | -3.2V | |
| L1 -Duty | 99.9% | | 
| L1 +Duty | 0.01% | <img src="./images/oscilloscope2.png"> |

### With magnetic near inductor
Probe 1: Vspeaker
Probe 2: Vl1
Probe 3: Vc4
Probe 4: Vc2
<img src="./images/oscilloscope3.png">

| Measurements | Value | Remark | 
| :------------- | :------------- | :--- |
| L1 | 8.06uH | Measured using Peak atlas LCR45 outside of circuit | 
| Fspkr | 443Hz | |
| Vspkr Pk-Pk | 3.36V | |
| Vspkr Vmax | 3.92V | |
| Vspkr Vmin | 560mV | |
| Spkr -Duty | 53.75% | |
| Spkr +Duty | 46.25% | |
| VL1 Pk-Pk | 6.84V | |
| VL1 Vmax | 3.56V | |
| VL1 Vmin | -3.28V | |
| L1 -Duty | Unmeasurable | | 
| L1 +Duty | Unmeasurable | |




