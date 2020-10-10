#!/usr/bin/env python3

"""
Tests the instance of our Vala actor.

I expect to see a red square on the white stage.
(It can be clicked.)

"""

import sys
from gi.repository import Palelib, Clutter

Clutter.init(sys.argv)
stage = Clutter.Stage()
stage.set_size(800, 400)
stage.set_title("Blah blah")
stage.connect('destroy', lambda x: Clutter.main_quit() )


# Make our Object:
rs = Palelib.RedSquare.new() #Note the .new() call. Yuck.
print(rs)
#print(dir(rs)) # See that it is an Actor object.

rs.set_position(100,100)

stage.add_child(rs)

#Force rs to appear. Calls a Vala method and passes a string.
rs.redraw("from Python")

"""
# Crud for testing:
r1 = Clutter.Rectangle()
r1.set_size(50,50)
r1.set_position(0,0)
damnweird = Clutter.Color.new(0,0,0,255)
r1.set_color( damnweird  )

stage.add_child(r1)
"""



"""
Let's get an event going from Python!
Because the RedSquare actor is *already* listening
to a button-press-event (in Vala) this is the second 
such event it will obey. 

I *think* it happens after the vala cleek() method runs.
If you |return true| in cleek(), then this does NOT run,
so that implies that Python is happening second in the chain.
"""
def gogo( a, evt ):
  print ("Hello from gogo. %s %s" % (a,evt))
rs.connect("button_press_event", gogo)



stage.show_all()
Clutter.main()