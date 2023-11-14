# Numbers Challenge

It is a common recreational problem to make a target number using a selection of other numbers. In this problem you will be given six numbers and a target number.

For example, given the six numbers 2, 3, 4, 6, 7, 25 and a target of 211, one possible solution is:

<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mn>211</mn>
  <mo>=</mo>
  <mo stretchy="false">(</mo>
  <mn>3</mn>
  <mo>+</mo>
  <mn>6</mn>
  <mo stretchy="false">)</mo>
  <mo>&#xD7;</mo>
  <mn>25</mn>
  <mo>&#x2212;</mo>
  <mo stretchy="false">(</mo>
  <mn>4</mn>
  <mo>&#xD7;</mo>
  <mn>7</mn>
  <mo stretchy="false">)</mo>
  <mo>&#xF7;</mo>
  <mn>2</mn>
</math>

This uses all six numbers. However, it is not necessary to do so. Another solution that does not use the 7 is:

<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mn>211</mn>
  <mo>=</mo>
  <mo stretchy="false">(</mo>
  <mn>25</mn>
  <mo>&#x2212;</mo>
  <mn>2</mn>
  <mo stretchy="false">)</mo>
  <mo>&#xD7;</mo>
  <mo stretchy="false">(</mo>
  <mn>6</mn>
  <mo>+</mo>
  <mn>3</mn>
  <mo stretchy="false">)</mo>
  <mo>+</mo>
  <mn>4</mn>
</math>

Define the score of a solution to be the sum of the numbers used. In the above example problem, the two given solutions have scores 47  and 40 respectively. It turns out that this problem has no solutions with score less than 40.

When combining numbers, the following rules must be observed:

* Each available number may be used at most once.
* Only the four basic arithmetic operations are permitted:
 <math xmlns="http://www.w3.org/1998/Math/MathML">
  <mo>+</mo>
</math>, 
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mo>&#x2212;</mo>
</math>, 
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mo>&#xD7;</mo>
</math>,
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mo>&#xF7;</mo>
</math>.
* All intermediate values must be positive integers, so for example 
 <math xmlns="http://www.w3.org/1998/Math/MathML">
  <mo stretchy="false">(</mo>
  <mn>3</mn>
  <mo>&#xF7;</mo>
  <mn>2</mn>
  <mo stretchy="false">)</mo>
</math> is never permitted as a subexpression (even if the final answer is an integer).

The attached file number-challenges.txt contains 200 problems, one per line in the format:
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mn>211</mn>
  <mo>:</mo>
  <mn>2</mn>
  <mo>,</mo>
  <mn>3</mn>
  <mo>,</mo>
  <mn>4</mn>
  <mo>,</mo>
  <mn>6</mn>
  <mo>,</mo>
  <mn>7</mn>
  <mo>,</mo>
  <mn>25</mn>
</math>

where the number before the colon is the target and the remaining comma-separated numbers are those available to be used.

Numbering the problems 1, 2, …, 200, we let 
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <msub>
    <mi>s</mi>
    <mi>n</mi>
  </msub>
</math> be the minimum score of the solution to the 
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mi>n</mi>
</math>th problem. For example, 
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <msub>
    <mi>s</mi>
    <mn>1</mn>
  </msub>
  <mo>=</mo>
  <mn>40</mn>
</math>, as the first problem in the file is the example given above. Note that not all problems have a solution; in such cases we take  
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <msub>
    <mi>s</mi>
    <mi>n</mi>
  </msub>
  <mo>=</mo>
  <mn>0</mn>
</math>.

Find 
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mstyle displaystyle="true" scriptlevel="0">
    <munderover>
      <mo data-mjx-texclass="OP">&#x2211;</mo>
      <mrow data-mjx-texclass="ORD">
        <mi>n</mi>
        <mo>=</mo>
        <mn>1</mn>
      </mrow>
      <mrow data-mjx-texclass="ORD">
        <mn>200</mn>
      </mrow>
    </munderover>
    <msup>
      <mn>3</mn>
      <mi>n</mi>
    </msup>
    <msub>
      <mi>s</mi>
      <mi>n</mi>
    </msub>
  </mstyle>
</math>. Give your answer modulo <math xmlns="http://www.w3.org/1998/Math/MathML">
  <mn>1005075251</mn>
</math>.