use view::*;
use geometry::*;
use tile::*;

pub struct Window {
	pub rect: Rect,
	pub view: View,
}

impl Window {

	pub fn new(rect: Rect) -> Window {
		Window {
			rect: rect,
			view: View::new_default("main".to_string(), rect, Point::new(0, 0)),
		}
	}

	pub fn redraw(&mut self) {
		self.view.draw();

		print!("\x1B[?25l"); //hide cursor
		print!("\x1B[2J");   //clear screen
		print!("\x1B[3J\x1B[1;1H"); //remove history

		for i in 0..self.rect.height {
	        for j in 0..self.rect.width {
				let t = self.view.view.get_tile(Point::new(j, self.rect.height - 1 - i));
				if let Some(tile) = t {
					tile.print_tile();
				}
				else {
					Tile::new_text_background('.', Colour::Transparent, Colour::Transparent).print_tile();
				}
            	print!("\x1B[0m");  //reset colours
        	}
	    	print!("\n");
	    }
		print!("\x1B[?25h"); //show cursor
		//self.clear();
	}


}
