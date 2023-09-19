
Text is composed Lines, a line is composed of Spans which looks something like:
```
Text [
	Line[span(hello), span( world!)],
	Line[span(this is a line of spans of text)]
]
```

In order to highlight the character the cursor is on in a block of text we need to divide an input String into Lines at \n and separate the line into 3 spans before, at, and after the cursor. which looks like
```
Text[
	Line[span(some text in a line)]
	Line[span(string before cursor), span(char), span(string after cursor)]
	Line[span(some more text)]
]
```

~~rant~~: theres this problem with lines and spans. say i need to highlight the character at the cursor. i need to take the line im at and deconstruct the spans into a string then reconstruct the lines out of a set of spans representing before, at, and after the cursor. which results in something like this:
```
Line(span(hello), span( world), span( !!!))
+-------------------------^
                our cursor is here
```
when we deconstruct the spans into a string we get ```hello world !!!``` and after parcing the string to highlight the char at the cursor we then recreate a line of spans that looks like this:
```
Line(span(hello wo), span(r), span(ld !!!))
```
this completely destroys the previous formatting of the line and i am unsure what to do about it. for my current goal of making a textbox widget it might not be a problem, but would definetly be a problem when doing markdown formatting, or code highlighting.

 * from Eyesonjune: "*You would have to copy the style of the previous spans into the new ones*"
 * from Strange-Knoll: "*I could probably do something with the width of the spans to figure out what span in the line my cursor is at and only split that span. and then copy the spans accordingly*"

```
Line(span(hello), span( world), span( !!!))
//widths:[5, 6, 4]        ^
//cursor:9 -------------- | our cursor is here in the string
```
we do some processing to get the index of the span our cursor in in.
```
sum_widths = 0
index = 0
for span in Line{
	if sum_widths > cursor{
		return index
	}else{
		sum_widths += span.width
		index ++
	}
}
```
which gives us data that looks like
```
sum_widths = 5
index = 1
// the index points at span( world)
```
we then subtract sum_widths from our cursor pos to our position in the span
```
span_pos = cursor - sum_widths // 9-5 = 4
```
now we know what span to look at and where in the span the cursor is. we take the content of that span and separate it into 3 spans centered on the cursor position. the outer 2 spans will copy the style of the original span, and the center span will use the cursor style. 
```
span( wo), span(r),  span(ld)
```
finally we reconstruct the line by inserting the first span in its original spans place, and then inserting the other 2 spans behind it, which gives us our final output:
```
Line(
	span1(hello), 
	span2( wo), c_span(r), span2(ld),
	span3( !!!))
```



~~i think in can do it by tracking the widths of the original spans and reconstruct the line by applying the copied spans selectively like 
```
line = Line(span(hello ), span(world))
widths = [6, 5]
string = line_to_string(line) //hello world

// do some facny logic to sort out where the cursor is
// we get to ...
new_line = Line(span(Hel), span(l), span(o world))
for span in line.spans{
	// some logic that 
}
```

