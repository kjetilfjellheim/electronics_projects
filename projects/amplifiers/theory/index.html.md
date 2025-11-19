---
title: Electronics projects

meta:
  - name: description
    content: Op-amps theory

---

# Op-amps theory
The goal is to understand the theory of an op-amp. 

# Theory
The theoretical amplification of an op-amp is given by the following formula.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>V</mi>
              <mrow data-mjx-texclass="ORD">
                <mi>o</mi>
                <mi>u</mi>
                <mi>t</mi>
              </mrow>
            </msub>
            <mo>=</mo>
            <mi>A</mi>
            <mo>&#xD7;</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>V</mi>
              <mo>+</mo>
            </msub>
            <mo>&#x2212;</mo>
            <msub>
              <mi>V</mi>
              <mo>&#x2212;</mo>
            </msub>
            <mo stretchy="false">)</mo>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Which gives the following formula for gain.

## Rules
The rules of op-amps.

1. The open-loop gain of an ideal op-amp is infinite. Open-loop in this case means without any feedback.
2. The input impedance for an ideal op-amp is infinite.
3. The input of an ideal op-amp is zero amp.

<br>
<ins>Example</ins>
<br>
When the input signal is on V+ and V- is grounded the formula for Vout is.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>V</mi>
              <mrow data-mjx-texclass="ORD">
                <mi>o</mi>
                <mi>u</mi>
                <mi>t</mi>
              </mrow>
            </msub>
            <mo>=</mo>
            <mi>A</mi>
            <mo>&#xD7;</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>V</mi>
              <mo>+</mo>
            </msub>
            <mo>&#x2212;</mo>
            <msub>
              <mi>V</mi>
              <mo>&#x2212;</mo>
            </msub>
            <mo stretchy="false">)</mo>
            <mo>=</mo>
            <mi>A</mi>
            <mo>&#xD7;</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>V</mi>
              <mo>+</mo>
            </msub>
            <mo>&#x2212;</mo>
            <mn>0</mn>
            <mo stretchy="false">)</mo>
            <mo>=</mo>
            <mi>A</mi>
            <mo>&#xD7;</mo>
            <msub>
              <mi>V</mi>
              <mo>+</mo>
            </msub>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br>
Which gives the given formula for gain.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>A</mi>
            <mo>=</mo>
            <mfrac>
              <msub>
                <mi>V</mi>
                <mrow data-mjx-texclass="ORD">
                  <mi>o</mi>
                  <mi>u</mi>
                  <mi>t</mi>
                </mrow>
              </msub>
              <msub>
                <mi>V</mi>
                <mo>+</mo>
              </msub>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br>
<ins>Example</ins>
<br>
When the input signal is on V- and V+ is grounded the formula for Vout is.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <msub>
              <mi>V</mi>
              <mrow data-mjx-texclass="ORD">
                <mi>o</mi>
                <mi>u</mi>
                <mi>t</mi>
              </mrow>
            </msub>
            <mo>=</mo>
            <mi>A</mi>
            <mo>&#xD7;</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>V</mi>
              <mo>+</mo>
            </msub>
            <mo>&#x2212;</mo>
            <msub>
              <mi>V</mi>
              <mo>&#x2212;</mo>
            </msub>
            <mo stretchy="false">)</mo>
            <mo>=</mo>
            <mi>A</mi>
            <mo>&#xD7;</mo>
            <mo stretchy="false">(</mo>
            <mn>0</mn>
            <mo>&#x2212;</mo>
            <msub>
              <mi>V</mi>
              <mo>&#x2212;</mo>
            </msub>
            <mo stretchy="false">)</mo>
            <mo>=</mo>
            <mi>A</mi>
            <mo>&#xD7;</mo>
            <mo>&#x2212;</mo>
            <msub>
              <mi>V</mi>
              <mo>&#x2212;</mo>
            </msub>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>
<br><br>
Which gives the given formula for gain.
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mi>A</mi>
            <mo>=</mo>
            <mo>&#x2212;</mo>
            <mfrac>
              <msub>
                <mi>V</mi>
                <mrow data-mjx-texclass="ORD">
                  <mi>o</mi>
                  <mi>u</mi>
                  <mi>t</mi>
                </mrow>
              </msub>
              <msub>
                <mi>V</mi>
                <mo>&#x2212;</mo>
              </msub>
            </mfrac>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>

# Rule for feedback loops
For feedback loops there is one more rule.
<br><br>
4. When there's a difference in input voltage between V- and V+ the op-amp will try to feed back as much current or voltage to keep the difference as close to 0 as possible.

# Changelog
| Date | Change |
| :---- | :---- |
| 2025-11-17 | First version |
