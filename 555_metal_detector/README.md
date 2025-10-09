# 555 metal detector

## Description
The 555 will oscillate based on the LCR circuit. When metal comes in contact with the inductor the oscillation will change due to it affecting the
magnetic field around the inductor. The output is the sent to the speaker which will change tone when metallic (Magnetic) objects comes near.

## Goals
- [x] Document schematic in Kicad
- [x] Do a theoretical analyis 
- [ ] Implement schematic on breadboard
- [ ] Do a practical analysis
- [ ] Read 555 output frequency on microcontroller
- [ ] Output PWM from microcontroller if freuqncy has changed

## Analytics

### Theoretical
Contains a Kicad analytics file containing a simple theoretical output of the circuit with no metallic object near the inductor.

### Practical
To be done

## Schematics
Kicad file: schematics/schematics.kicad_sch
<img src="./schematics/schematics.svg">

## Components
Reference | Value
C1,C3 and C4 | 1u 
C2 | 10n
J1 | 5V
L1 | 1m
R1 | 100k
Speaker | 8 ohm
U1 | LM555xN http://www.ti.com/lit/ds/symlink/lm555.pdf 

## Code
To be implemented. The plan is to send the output from the into a microcontroller which only outputs a tone if the frequency in changes.




