<div align="center"><h1>simple brainfuck-to-C compiler.</h1></div>
<div align="center"><h3>this program compiles any brainfuck program into a C program!<br>
  it also automatically compiles the C program with GCC, with optional<br>
  optimization flags. (<code>-O1</code>, <code>-O2</code> etc etc...).<br><br>
  the GNU C Compiler does<br>the heavy lifting here...</h3></div>
hanoi.bf is in the repo. to test it, run:<br><code>cargo run hanoi.bf -o hanoi -O4</code><br>
you must have GCC installed in order for this code
to work properly, obviously
