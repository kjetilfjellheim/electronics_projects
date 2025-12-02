---
title: Electronics projects

meta:
  - name: description
    content: Class-A (Common emitter) 

---
# Class-A (Common emitter) 
The goal is to understand a class A common emitter amplifier.

# Goals
- [x] Document schematic in Kicad
- [x] Do a theoretical analysis with DC
- [x] Do a theoretical analysis with AC
- [ ] Implement schematic on breadboard
- [ ] Test circuit and do measurements with DC
- [ ] Test circuit and do measurements with AC


# Equipment used
| Equipment | Description |
| :------------- | :------------- |
| Peaktech 6075 | DC power supply |
| Peaktech 4055mv | AC power supply |
| RS Pro RSDS 1204X-E | Oscilloscope 

# Components
| Reference | Value | Remarks |
| :------------- | :------------- | :------------- |
| C1 | 470n | Input coupling |
| C2 | 6u | Bias decoupling |
| C3 | 265u | Emitter bypass |
| C4 | 220n | Output coupling |
| R1 | 32.4k |  |
| R2 | 7.6k |  |
| R3 | 500 | |
| R4 | 120 |  |
| BC547 | Transistor | hfe=400 |

# DC circuit
## Circuit
> <img src="images/dc_schematics.svg">

The values given in the circuit are calculated in the calculations below. The given values 

## Formulas
The theoretical voltage gain for a common emitter amplifier is.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>G</mi>
            <mi>a</mi>
            <mi>i</mi>
            <mi>n</mi>
            <mo>=</mo>
            <mo>&#x2212;</mo>
            <mfrac>
              <msub>
                <mi>R</mi>
                <mn>3</mn>
              </msub>
              <msub>
                <mi>R</mi>
                <mn>4</mn>
              </msub>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Choose a collector current through R3. The formula for R3 becomes. 
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>R</mi>
              <mn>3</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mfrac>
                <mrow>
                  <mi>V</mi>
                  <mi>c</mi>
                  <mi>c</mi>
                </mrow>
                <mn>2</mn>
              </mfrac>
              <msub>
                <mi>I</mi>
                <mi>c</mi>
              </msub>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
The emitter voltage should be around 10%-15% of Vcc. If we assume Ie=Ic the R4 is.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>R</mi>
              <mn>4</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <msub>
                <mi>V</mi>
                <mi>e</mi>
              </msub>
              <msub>
                <mi>I</mi>
                <mi>e</mi>
              </msub>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating Ib with this formula. The hfe is the gain given by the transistors datasheet. 
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>I</mi>
              <mi>b</mi>
            </msub>
            <mo>=</mo>
            <mfrac>
              <msub>
                <mi>I</mi>
                <mi>c</mi>
              </msub>
              <mrow>
                <mi>h</mi>
                <mi>f</mi>
                <mi>e</mi>
              </mrow>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Vb should be around 0.7V higher than Ve so that we are on the linear part of the transistor characteristic.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>V</mi>
              <mi>b</mi>
            </msub>
            <mo>=</mo>
            <msub>
              <mi>V</mi>
              <mi>e</mi>
            </msub>
            <mo>+</mo>
            <mn>0.7</mn>
            <mi>V</mi>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating the DC bias network current. The current should be 10 times Ib.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>I</mi>
              <mrow data-mjx-texclass="ORD">
                <mi>r</mi>
                <mn>1</mn>
              </mrow>
            </msub>
            <mo>=</mo>
            <msub>
              <mi>I</mi>
              <mi>b</mi>
            </msub>
            <mo>&#xD7;</mo>
            <mn>10</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating the resistor R1 is the Vcc minus Vb divided by the bias current.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>R</mi>
              <mn>1</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mrow>
                <msub>
                  <mi>V</mi>
                  <mrow data-mjx-texclass="ORD">
                    <mi>c</mi>
                    <mi>c</mi>
                  </mrow>
                </msub>
                <mo>&#x2212;</mo>
                <msub>
                  <mi>V</mi>
                  <mi>b</mi>
                </msub>
              </mrow>
              <msub>
                <mi>I</mi>
                <mrow data-mjx-texclass="ORD">
                  <mi>r</mi>
                  <mn>1</mn>
                  <mi>r</mi>
                  <mn>2</mn>
                </mrow>
              </msub>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating the resistor R2 is Vb divided by bias current.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>R</mi>
              <mn>2</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <msub>
                <mi>V</mi>
                <mi>b</mi>
              </msub>
              <msub>
                <mi>I</mi>
                <mrow data-mjx-texclass="ORD">
                  <mi>r</mi>
                  <mn>1</mn>
                  <mi>r</mi>
                  <mn>2</mn>
                </mrow>
              </msub>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>

## Calculations
Calculating gain
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>G</mi>
            <mi>a</mi>
            <mi>i</mi>
            <mi>n</mi>
            <mo>=</mo>
            <mo>&#x2212;</mo>
            <mfrac>
              <msub>
                <mi>R</mi>
                <mn>3</mn>
              </msub>
              <msub>
                <mi>R</mi>
                <mn>4</mn>
              </msub>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mrow>
                <mo>&#x2212;</mo>
                <mn>500</mn>
              </mrow>
              <mn>120</mn>
            </mfrac>
            <mo>=</mo>
            <mo>&#x2212;</mo>
            <mn>4.17</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Ic max for a BC547 is 100mA. We choose the Ic to be 10% of Ic max therefore 10mA. We choose a Vcc of 10V. This makes R3.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>R</mi>
              <mn>3</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mfrac>
                <msub>
                  <mi>V</mi>
                  <mrow data-mjx-texclass="ORD">
                    <mi>c</mi>
                    <mi>c</mi>
                  </mrow>
                </msub>
                <mn>2</mn>
              </mfrac>
              <msub>
                <mi>I</mi>
                <mi>c</mi>
              </msub>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mn>5</mn>
              <mrow>
                <mn>10</mn>
                <mo>&#xD7;</mo>
                <msup>
                  <mn>10</mn>
                  <mrow data-mjx-texclass="ORD">
                    <mo>&#x2212;</mo>
                    <mn>3</mn>
                  </mrow>
                </msup>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mn>500</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
The emitter voltage should be around 10%-15% of Vcc. If we assume Ie=Ic the R4 is.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>R</mi>
              <mn>4</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <msub>
                <mi>V</mi>
                <mi>e</mi>
              </msub>
              <msub>
                <mi>I</mi>
                <mi>e</mi>
              </msub>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mn>1.2</mn>
              <mrow>
                <mn>10</mn>
                <mo>&#xD7;</mo>
                <msup>
                  <mn>10</mn>
                  <mrow data-mjx-texclass="ORD">
                    <mo>&#x2212;</mo>
                    <mn>3</mn>
                  </mrow>
                </msup>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mn>120</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating Ib we use use a value of 400 for the hfe as the typical value.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>I</mi>
              <mi>b</mi>
            </msub>
            <mo>=</mo>
            <mfrac>
              <msub>
                <mi>I</mi>
                <mi>c</mi>
              </msub>
              <mrow>
                <mi>h</mi>
                <mi>f</mi>
                <mi>e</mi>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mrow>
                <mn>10</mn>
                <mo>&#xD7;</mo>
                <msup>
                  <mn>10</mn>
                  <mrow data-mjx-texclass="ORD">
                    <mo>&#x2212;</mo>
                    <mn>3</mn>
                  </mrow>
                </msup>
              </mrow>
              <mn>400</mn>
            </mfrac>
            <mo>=</mo>
            <mn>25</mn>
            <mo>&#xD7;</mo>
            <msup>
              <mn>10</mn>
              <mrow data-mjx-texclass="ORD">
                <mo>&#x2212;</mo>
                <mn>6</mn>
              </mrow>
            </msup>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating Vb we add 0.7V to the expected Ve.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>V</mi>
              <mi>b</mi>
            </msub>
            <mo>=</mo>
            <msub>
              <mi>V</mi>
              <mi>e</mi>
            </msub>
            <mo>+</mo>
            <mn>0.7</mn>
            <mi>V</mi>
            <mo>=</mo>
            <mo stretchy="false">(</mo>
            <mn>10</mn>
            <mo>&#xD7;</mo>
            <mn>0.12</mn>
            <mo stretchy="false">)</mo>
            <mo>+</mo>
            <mn>0.7</mn>
            <mo>=</mo>
            <mn>1.9</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating current through R1 and R2.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0"  style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>I</mi>
              <mrow data-mjx-texclass="ORD">
                <mi>r</mi>
                <mn>1</mn>
                <mi>r</mi>
                <mn>2</mn>
              </mrow>
            </msub>
            <mo>=</mo>
            <msub>
              <mi>I</mi>
              <mi>b</mi>
            </msub>
            <mo>&#xD7;</mo>
            <mn>10</mn>
            <mo>=</mo>
            <mn>250</mn>
            <mo>&#xD7;</mo>
            <msup>
              <mn>10</mn>
              <mrow data-mjx-texclass="ORD">
                <mo>&#x2212;</mo>
                <mn>6</mn>
              </mrow>
            </msup>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating the resistor R1.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>R</mi>
              <mn>1</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mrow>
                <msub>
                  <mi>V</mi>
                  <mrow data-mjx-texclass="ORD">
                    <mi>c</mi>
                    <mi>c</mi>
                  </mrow>
                </msub>
                <mo>&#x2212;</mo>
                <msub>
                  <mi>V</mi>
                  <mi>b</mi>
                </msub>
              </mrow>
              <msub>
                <mi>I</mi>
                <mrow data-mjx-texclass="ORD">
                  <mi>r</mi>
                  <mn>1</mn>
                  <mi>r</mi>
                  <mn>2</mn>
                </mrow>
              </msub>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mrow>
                <mn>10</mn>
                <mo>&#x2212;</mo>
                <mn>1.9</mn>
              </mrow>
              <mrow>
                <mn>250</mn>
                <mo>&#xD7;</mo>
                <msup>
                  <mn>10</mn>
                  <mrow data-mjx-texclass="ORD">
                    <mo>&#x2212;</mo>
                    <mn>6</mn>
                  </mrow>
                </msup>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mn>32400</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating R2.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>R</mi>
              <mn>2</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <msub>
                <mi>V</mi>
                <mi>b</mi>
              </msub>
              <msub>
                <mi>I</mi>
                <mrow data-mjx-texclass="ORD">
                  <mi>r</mi>
                  <mn>1</mn>
                  <mi>r</mi>
                  <mn>2</mn>
                </mrow>
              </msub>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mn>1.9</mn>
              <mrow>
                <mn>10</mn>
                <mo>&#xD7;</mo>
                <msup>
                  <mn>10</mn>
                  <mrow data-mjx-texclass="ORD">
                    <mo>&#x2212;</mo>
                    <mn>3</mn>
                  </mrow>
                </msup>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mn>7600</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>

## Simulation
> <img src="images/simulation_dc.png">

The output voltage decreases as the input voltage increases. As the voltage of Vbe increases this causes the resitance over
CE to decrease which causes a greater amount of voltage to be stored on R3. So the output phase is 180 degrees of the input.
<br><br>
When the input voltage is 0.4V, the output is 4.9V and at 1.4V the output is 1.14V. This gives a voltage gain of 3.76.

## Practical measurements

# AC
## Formulas
The input impedance is approximately.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>Z</mi>
            <mi>i</mi>
            <mi>n</mi>
            <mo>=</mo>
            <mi>R</mi>
            <mn>1</mn>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">|</mo>
            <mi>R</mi>
            <mn>2</mn>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">(</mo>
            <mi>&#x3B2;</mi>
            <mo>&#xD7;</mo>
            <mi>R</mi>
            <mn>4</mn>
            <mo stretchy="false">)</mo>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating C1. This should be for the lower frequency for -3db.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>C</mi>
              <mn>1</mn>
            </msub>
            <mo>=</mo>
            <msup>
              <mfrac>
                <mn>1</mn>
                <mrow>
                  <mn>2</mn>
                  <mi>&#x3C0;</mi>
                  <mi>f</mi>
                  <msub>
                    <mi>Z</mi>
                    <mi>i</mi>
                  </msub>
                  <mi>n</mi>
                </mrow>
              </mfrac>
            </msup>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
C2 is used for power supply decoupling. Usually this value is between 1u and 10u. The formula for approximate it is.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>C</mi>
              <mn>2</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mi>f</mi>
                <msub>
                  <mi>R</mi>
                  <mn>3</mn>
                </msub>
              </mrow>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculatuing C3 as a bypass capacitor. The frequency is for the lower cutoff.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>C</mi>
            <mn>3</mn>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mi>f</mi>
                <mo>&#xD7;</mo>
                <mfrac>
                  <msub>
                    <mi>R</mi>
                    <mn>4</mn>
                  </msub>
                  <mn>10</mn>
                </mfrac>
              </mrow>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating C4. This should be for the lower frequency for -3db.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>C</mi>
              <mn>4</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mi>f</mi>
                <msub>
                  <mi>X</mi>
                  <mrow data-mjx-texclass="ORD">
                    <mi>o</mi>
                    <mi>u</mi>
                    <mi>t</mi>
                  </mrow>
                </msub>
              </mrow>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>

## Calculations
First calculating input impedance. 
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>Z</mi>
            <mi>i</mi>
            <mi>n</mi>
            <mo>=</mo>
            <mi>R</mi>
            <mn>1</mn>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">|</mo>
            <mi>R</mi>
            <mn>2</mn>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">(</mo>
            <mi>&#x3B2;</mi>
            <mo>&#xD7;</mo>
            <mi>R</mi>
            <mn>4</mn>
            <mo stretchy="false">)</mo>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <mi>Z</mi>
            <mi>i</mi>
            <mi>n</mi>
            <mo>=</mo>
            <mn>32400</mn>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">|</mo>
            <mn>7600</mn>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">|</mo>
            <mo stretchy="false">(</mo>
            <mn>400</mn>
            <mo>&#xD7;</mo>
            <mn>120</mn>
            <mo stretchy="false">)</mo>
            <mo>=</mo>
            <mn>5400</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
We calculate C1 with a -3db at 50Hz. The input impedance of the input with R2 and C1 is approximately 7600. This will need tuning when the circuit is set up as it's only approximately.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>C</mi>
              <mn>1</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mi>f</mi>
                <msub>
                  <mi>Z</mi>
                  <mi>i</mi>
                </msub>
                <mi>n</mi>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mo>&#xD7;</mo>
                <mi>&#x3C0;</mi>
                <mo>&#xD7;</mo>
                <mn>50</mn>
                <mo>&#xD7;</mo>
                <mn>5400</mn>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mn>470</mn>
            <mo>&#xD7;</mo>
            <msup>
              <mn>10</mn>
              <mrow data-mjx-texclass="ORD">
                <mo>&#x2212;</mo>
                <mn>9</mn>
              </mrow>
            </msup>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
We calculate C2.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>C</mi>
              <mn>2</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mi>f</mi>
                <msub>
                  <mi>R</mi>
                  <mn>3</mn>
                </msub>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mtext>&#xA0;</mtext>
                <mn>50</mn>
                <mo>&#xD7;</mo>
                <mn>500</mn>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mn>6</mn>
            <mo>&#xD7;</mo>
            <msup>
              <mn>10</mn>
              <mrow data-mjx-texclass="ORD">
                <mo>&#x2212;</mo>
                <mn>6</mn>
              </mrow>
            </msup>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating the buypass capacitor, C3.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>C</mi>
            <mn>3</mn>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mi>f</mi>
                <mo>&#xD7;</mo>
                <mfrac>
                  <msub>
                    <mi>R</mi>
                    <mn>4</mn>
                  </msub>
                  <mn>10</mn>
                </mfrac>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mi>f</mi>
                <mo>&#xD7;</mo>
                <mn>50</mn>
                <mo>&#xD7;</mo>
                <mfrac>
                  <mn>120</mn>
                  <mn>10</mn>
                </mfrac>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mn>265</mn>
            <mo>&#xD7;</mo>
            <msup>
              <mn>10</mn>
              <mrow data-mjx-texclass="ORD">
                <mo>&#x2212;</mo>
                <mn>6</mn>
              </mrow>
            </msup>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Calculating C4. We assume thout load to the circuit (external) is 10k.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>C</mi>
              <mn>4</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mi>f</mi>
                <msub>
                  <mi>Z</mi>
                  <mrow data-mjx-texclass="ORD">
                    <mi>o</mi>
                    <mi>u</mi>
                    <mi>t</mi>
                  </mrow>
                </msub>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mrow>
                <mn>2</mn>
                <mi>&#x3C0;</mi>
                <mn>50</mn>
                <mo>&#xD7;</mo>
                <mn>10000</mn>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mn>159</mn>
            <mo>&#xD7;</mo>
            <msup>
              <mn>10</mn>
              <mrow data-mjx-texclass="ORD">
                <mo>&#x2212;</mo>
                <mn>9</mn>
              </mrow>
            </msup>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>

## Circuit<!-- pagebreak -->
> <img src="images/ac_schematics.svg">

Compared to the DC circuit we have added four capacitors. 
C1 is used to set the input lowpass filter with R1.<br>
C2 is used to to isolate DC from the output.<br>
C3 is used to avoid AC voltage over R4.<br>
C4 is used to define a lowpass filter with R3.<br>

## Simulation
> <img src="images/simulation_ac.png">

The circuits gain have now increased. Vin peak to peak is now 0.06V, but the output is between 0.88V-4.45V. The gain therefore increased 
to around -59 (theoretical). There is a very slight distortion as the top is slightly wider than the bottom. 
<br><br>
The lower-than-theoretical gain occurs because:
- C3 doesn't fully bypass R4 at 50Hz startup transient
- Some AC voltage still appears across R4
- Transistor output impedance reduces gain

## Too high amplitude
> <img src="images/simulation_ac_too_high_amplitude.png">

When the input amplitude is too high, the transistor enters:
- top clipping: Vce ≈ 0V, collector at Ve
- bottom clipping: Transistor turns off, Vce ≈ Vcc
<br>
This causes the flattened curves. The linear operating range is limited by the DC bias point.

## Practical measurements

# Changelog
| Date | Change |
| :---- | :---- |
| 2025-11-30 | Adds dc circuit and calculations |
| 2025-12-01 | Adds ac circuit and simulations |
| 2025-12-02 | Update circuit and improved calculations |