use tile::Tile;
use tile::Colour;
use geometry::Point;
use geometry::Rect;

#[allow(dead_code)]
pub struct Context {
	rect: Rect,
	tiles: Vec<Vec<Tile>>
}

#[allow(dead_code)]
impl Context {

	pub fn new(rect: Rect) -> Context {

		let mut t = Vec::new();

		for _ in 0..rect.height {
			let mut t2 = Vec::new();
			for _ in 0..rect.width {
				t2.push(Tile::new());
			}
			t.push(t2)
		}

		Context {
			rect: rect,
			tiles: t
		}
	}

	pub fn new1(width: i32, height: i32) -> Context {

		let mut t = Vec::new();

		for _ in 0..height {
			let mut t2 = Vec::new();
			for _ in 0..width {
				t2.push(Tile::new());
			}
			t.push(t2)
		}

		Context {
			rect: Rect::new(width, height),
			tiles: t
		}
	}

	pub fn set_tiles(&mut self, p: Point, tile: Tile) {
		if self.rect.contains_point(p) {
			let mut bg_colour = tile.background_colour;
			let mut text_colour = tile.text_colour;
			let mut text = tile.value;

			if bg_colour == Colour::Transparent {
				bg_colour = self.get_tile(p).unwrap().background_colour;
			}
			if text_colour == Colour::Transparent {
				text_colour = self.get_tile(p).unwrap().text_colour;
			}
			if text == ' ' {
				text = self.get_tile(p).unwrap().value;
			}

			let t = Tile::new_text_background(text, text_colour, bg_colour);

			self.tiles[p.y as usize][p.x as usize] = t;

		}
	}

	pub fn get_tile(&self, p: Point) -> Option<Tile> {
		if self.rect.contains_point(p) {
			return Some(self.tiles[p.y as usize ][p.x as usize]);
		}
		None
	}

	pub fn point_within_context(&self, p: Point) -> bool {
		if self.rect.contains_point(p) {
			return true
		}
		false
	}

}
