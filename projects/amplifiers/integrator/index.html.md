---
title: Electronics projects

meta:
  - name: description
    content: Integrator

---

# Integrator
The goal is to understand a integrator amplifier.

# Goals
- [ ] Document schematic in Kicad
- [ ] Do a theoretical analyis 
- [ ] Implement schematic on breadboard
- [ ] Test circuit and do measurements

# Equipment used
| Equipment | Description |
| :------------- | :------------- |
| Peaktech 6075 | DC power supply |
| Peaktech 4055mv | AC power supply |
| RS Pro RSDS 1204X-E | Oscilloscope 

# Components
| Reference | Value | Remarks |
| :------------- | :------------- | :------------- |
| 1x LM741CN | | Operational amplifier | 
| R1 | 1k | | 
| C1 | 22n, 1u | 1u at measurement 1 and 2. 22n at measurement 3. |

# Circuit
> <img src="./images/schematics.svg">

# Practical measurements
All measurements were done with a supply voltage of +10V/-10V and the Vin+ input grounded. This is to ensure that the signal can swing without needing to set the bias og the non inverting input. 

## Measurement 1
> <img src="./images/practical_measurement_1.png">

Yellow here is the output voltage and purple is the input.<br>
This measurement was done at 40hz, with an amplitude of 1.5V. It gives an almost perfect waveform as output.

| Measured value | Measurement |
| :--- | :--- | Remark | 
| Vpp out | 18.8V | This is just inside the LM741 operating range. Outside this by increasing the input voltage see measurement 2.  |

<br><br>
What is interesting about the waveform is that is a straight line rather than a slope as is usual for a RC circuit. The reason it's a straight line is that the input current remains
constant. The cause for this is that the negative feedback keeps the input voltage at the inverting input at 0V, the same as the non inverting input. This again causes the voltage over 
R1 at a constant voltage. Since the voltage is constant the current is constant as well and the capacitor charges and discharges at a constant rate.

## Measurement 2
> <img src="./images/practical_measurement_2.png">

Yellow here is the output voltage, purple is the input and cyan is the voltage on the Vin-.<br>
Notice the cyan voltage increase/decrease over Vin-. The cause for this is that we have reached saturation. This causes the virtual ground at Vin- to start to drift away from 0V.

## Measurement 3
> <img src="./images/practical_measurement_3.png">

This measurement uses a capacitor at 22nF. Getting a perfect saw waveform out at 1.8khz. 

# Changelog
| Date | Change |
| :---- | :---- |
| 2025-12-16 | Added practical measurements and updated circuit. |