namespace Palelib {

    public class RedSquare : Clutter.Actor {

    //private vars
    private Clutter.Canvas _canvas;
    private int[] _col = { 255, 0, 0 };

    //Constructor - Needs to be called explicitly from Python by .new()
    public RedSquare() {
      stdout.printf( "RedSquare constructor.\n" );

      _canvas = new Clutter.Canvas();
      _canvas.set_size(300,300);

      this.set_size(300,300);
      this.set_content( _canvas );

      //Connect to the draw signal.
      _canvas.draw.connect(drawme);

      //Make it reactive and connect to the button-press-event
      this.set_reactive(true);
      this.button_press_event.connect( cleek );
    }

    //Button press signal handler
    private bool cleek ( Clutter.ButtonEvent evt ) {
      stdout.printf("Vala cleek() has run!\n");
      this._col = {0,255,0}; //Just change the colour
      this.redraw("from Vala");
      //return true; //Stops the signal here. Python won't get it.
      return false; //Lets the signal carry on going (to Python).
    }

    //Draws the Cairo art to the canvas
    private bool drawme( Cairo.Context ctx, int w, int h) {
      stdout.printf("drawme test.\n");
      ctx.set_source_rgb(this._col[0],this._col[1],this._col[2]);
      ctx.rectangle(0,0,300,300);
      ctx.fill();
      return true;
    }

    //Redraw - forces invalidate which trips the draw event
    //Am gonna call this directly from Python too!
    public void redraw(string? thing) {
      thing = thing ?? "from null"; //tests for null or else
      stdout.printf( "redraw test %s.\n", thing );

      this._canvas.invalidate();
    }
  } //end RedSquare c
}