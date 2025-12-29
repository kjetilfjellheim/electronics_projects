---
title: Electronics projects

meta:
  - name: description
    content: Network analysis

---

# Network analysis

# Goal
This calculates the current for a star system with two input power supplies and one output. This is a calculation for two unknowns.

## Schematics
> <img src="./images/schematics.svg">

## Calculations
Using the following formulas for a two branch solution.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>a</mi>
              <mn>1</mn>
            </msub>
            <mi>x</mi>
            <mo>+</mo>
            <msub>
              <mi>b</mi>
              <mn>1</mn>
            </msub>
            <mi>y</mi>
            <mo>+</mo>
            <msub>
              <mi>c</mi>
              <mn>1</mn>
            </msub>
            <mo>=</mo>
            <mn>0</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <msub>
              <mi>a</mi>
              <mn>2</mn>
            </msub>
            <mi>x</mi>
            <mo>+</mo>
            <msub>
              <mi>b</mi>
              <mn>2</mn>
            </msub>
            <mi>y</mi>
            <mo>+</mo>
            <msub>
              <mi>c</mi>
              <mn>2</mn>
            </msub>
            <mo>=</mo>
            <mn>0</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br>
Where 
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mfrac>
              <mi>x</mi>
              <msub>
                <mi>D</mi>
                <mi>x</mi>
              </msub>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mrow>
                <mo>&#x2212;</mo>
                <mi>y</mi>
              </mrow>
              <mrow>
                <mi>D</mi>
                <mi>y</mi>
              </mrow>
            </mfrac>
            <mo>=</mo>
            <mfrac>
              <mn>1</mn>
              <mi>D</mi>
            </mfrac>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <msub>
              <mi>D</mi>
              <mi>x</mi>
            </msub>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <msub>
                      <mi>b</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>c</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <msub>
                      <mi>b</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>c</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <msub>
              <mi>D</mi>
              <mi>y</mi>
            </msub>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <msub>
                      <mi>a</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>c</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <msub>
                      <mi>a</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>c</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <mi>D</mi>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <msub>
                      <mi>a</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>b</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <msub>
                      <mi>a</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>b</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br>
Setting up the formulas for each branch.<br>
Formula for branch 1
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>a</mi>
              <mn>1</mn>
            </msub>
            <mi>x</mi>
            <mo>+</mo>
            <msub>
              <mi>b</mi>
              <mn>1</mn>
            </msub>
            <mi>y</mi>
            <mo>+</mo>
            <mi>c</mi>
            <mn>1</mn>
            <mo>=</mo>
            <mn>0</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <mn>327</mn>
            <msub>
              <mi>I</mi>
              <mn>1</mn>
            </msub>
            <mo>+</mo>
            <mn>280</mn>
            <msub>
              <mi>I</mi>
              <mn>2</mn>
            </msub>
            <mo>&#x2212;</mo>
            <mn>7</mn>
            <mo>=</mo>
            <mn>0</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
Formula for branch 2
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>a</mi>
              <mn>2</mn>
            </msub>
            <mi>x</mi>
            <mo>+</mo>
            <msub>
              <mi>b</mi>
              <mn>2</mn>
            </msub>
            <mi>y</mi>
            <mo>+</mo>
            <mi>c</mi>
            <mn>2</mn>
            <mo>=</mo>
            <mn>0</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <mn>280</mn>
            <msub>
              <mi>I</mi>
              <mn>1</mn>
            </msub>
            <mo>+</mo>
            <mn>380</mn>
            <msub>
              <mi>I</mi>
              <mn>2</mn>
            </msub>
            <mo>&#x2212;</mo>
            <mn>9</mn>
            <mo>=</mo>
            <mn>0</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
Solving determinants.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>d</mi>
            <mi>e</mi>
            <mi>t</mi>
            <mtext>&#xA0;</mtext>
            <msub>
              <mi>D</mi>
              <mi>x</mi>
            </msub>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <msub>
                      <mi>b</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>c</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <msub>
                      <mi>b</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>c</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <mn>280</mn>
                  </mtd>
                  <mtd>
                    <mo>&#x2212;</mo>
                    <mn>7</mn>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <mn>380</mn>
                  </mtd>
                  <mtd>
                    <mo>&#x2212;</mo>
                    <mn>9</mn>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
            <mo>=</mo>
            <mn>140</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <mi>d</mi>
            <mi>e</mi>
            <mi>t</mi>
            <mtext>&#xA0;</mtext>
            <msub>
              <mi>D</mi>
              <mi>y</mi>
            </msub>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <msub>
                      <mi>a</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>c</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <msub>
                      <mi>a</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>c</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <mn>327</mn>
                  </mtd>
                  <mtd>
                    <mo>&#x2212;</mo>
                    <mn>7</mn>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <mn>280</mn>
                  </mtd>
                  <mtd>
                    <mo>&#x2212;</mo>
                    <mn>9</mn>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
            <mo>=</mo>
            <mn>-983</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <mi>d</mi>
            <mi>e</mi>
            <mi>t</mi>
            <mtext>&#xA0;</mtext>
            <mi>D</mi>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <msub>
                      <mi>a</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>b</mi>
                      <mn>1</mn>
                    </msub>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <msub>
                      <mi>a</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                  <mtd>
                    <msub>
                      <mi>b</mi>
                      <mn>2</mn>
                    </msub>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
            <mo>=</mo>
            <mrow data-mjx-texclass="INNER">
              <mo data-mjx-texclass="OPEN">[</mo>
              <mtable columnspacing="1em" rowspacing="4pt">
                <mtr>
                  <mtd>
                    <mn>327</mn>
                  </mtd>
                  <mtd>
                    <mn>280</mn>
                  </mtd>
                </mtr>
                <mtr>
                  <mtd>
                    <mn>280</mn>
                  </mtd>
                  <mtd>
                    <mn>380</mn>
                  </mtd>
                </mtr>
              </mtable>
              <mo data-mjx-texclass="CLOSE">]</mo>
            </mrow>
            <mo>=</mo>
            <mn>45860</mn>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
Calculating the values for I1 and I2.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0" style="font-size: 0.7em">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>I</mi>
              <mn>1</mn>
            </msub>
            <mo>=</mo>
            <mfrac>
              <mn>140</mn>
              <mn>45680</mn>
            </mfrac>
            <mo>=</mo>
            <mn>3</mn>
            <mi>m</mi>
            <mi>A</mi>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <msub>
              <mi>I</mi>
              <mn>2</mn>
            </msub>
            <mo>=</mo>
            <mo>&#x2212;</mo>
            <mfrac>
              <mn>-231</mn>
              <mn>45680</mn>
            </mfrac>
            <mo>=</mo>
            <mn>21.4</mn>
            <mi>m</mi>
            <mi>A</mi>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>

## Practical measurement

Practical measurements current.

| Current value | Measurement from multimeter | Measurement from power supply | Calculated value |
| :--- | :--- | :--- | :--- | 
| I1 | 2.95mA | 3mA | 3mA |
| I2 | 21.52mA | 23mA | 21.4mA |
| I1+I2 | 24.65mA | - | 24.4mA |

## Code
This code calculates the values using Rust.
