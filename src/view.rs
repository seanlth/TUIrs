use tile::Tile;
use tile::Colour;
use context::Context;
use geometry::Point;
use geometry::Rect;

pub trait Drawable {
	fn get_tile(&self, position: Point) -> Option<Tile>;
	fn draw_tile(&mut self, position: Point, tile: Tile);
	fn fill(&mut self, colour: Colour);
	fn fill_rect(&mut self, position: Point, rect: Rect, colour: Colour);
}

pub trait Castable<T> {
	fn cast(view: &ViewTypes) -> Option<&T>;
	fn cast_mut(view: &mut ViewTypes) -> Option<&mut T>;
}

struct BaseView {
	rect: Rect,
	context: Context
}

impl BaseView {
	pub fn new(rect: Rect) -> BaseView {
		BaseView {
			rect: rect,
			context: Context::new(rect)
		}
	}

	pub fn fill(&mut self, colour: Colour) {
		for i in 0..self.rect.width {
			for j in 0..self.rect.height {
				self.context.set_tiles(Point::new(i, j), Tile::new_background(colour));
			}
		}
	}
}

pub struct DefaultView {
	view: BaseView
}

impl DefaultView {
	pub fn new(rect: Rect) -> DefaultView {
		DefaultView {
			view: BaseView::new(rect)
		}
	}
}

impl Castable<DefaultView> for DefaultView {
	fn cast(view: &ViewTypes) -> Option<&DefaultView> {
		if let &ViewTypes::DefaultView(ref v) = view {
			return Some(v);
		}
		None
	}

	fn cast_mut(view: &mut ViewTypes) -> Option<&mut DefaultView> {
		if let &mut ViewTypes::DefaultView(ref mut v) = view {
			return Some(v);
		}
		None
	}

}

impl Drawable for DefaultView {
	fn get_tile(&self, position: Point) -> Option<Tile> {
		self.view.context.get_tile(position)
	}

	fn draw_tile(&mut self, position: Point, tile: Tile) {
		self.view.context.set_tiles(position, tile);
	}

	fn fill(&mut self, colour: Colour) {
		self.view.fill(colour);
	}

	fn fill_rect(&mut self, position: Point, rect: Rect, colour: Colour) {
		for i in 0..rect.width {
			for j in 0..rect.height {
				self.draw_tile(Point::new(position.x + i, position.y + j), Tile::new_background(colour));
			}
		}
	}
}

pub struct TextBox {
	view: BaseView,
	pub text: String,
	pub text_colour: Colour,
	pub background_colour: Colour
}

impl TextBox {
	pub fn new(rect: Rect, text: String, text_colour: Colour, background_colour: Colour) -> TextBox {
		let mut text_box = TextBox {
			view: BaseView::new(rect),
			text: text,
			text_colour: text_colour,
			background_colour: background_colour
		};

		text_box.draw();
		text_box
	}

	fn count(&self, text: &String) -> usize {
		let mut count = 0;

		for i in text.as_bytes().iter() {
			let c = *i as char;

			count += match c {
				'\t' => 4,
				'\n'|'\r' => self.view.rect.width - count % self.view.rect.width,
				_ => 1
			};
		}
		count as usize
	}

	fn get_text_to_remove(&self, text: &String, count: i32) -> usize {
		let mut current_count = 0;
		let mut adjusted_count = 0;

		println!("count == {}", count);

		for i in text.as_bytes().iter() {
			let c = *i as char;
			println!("ascii == {}", i);
			let v = match c {
				'\t' => 4,
				'\n'|'\r' => ( self.view.rect.width - adjusted_count as i32 % self.view.rect.width ) as i32,
				_ => 1
			};
			adjusted_count += v;
			current_count += 1;
			println!("adjusted = {}", adjusted_count);
			if adjusted_count >= count {
				break;
			}
		}

		current_count as usize
	}

	pub fn draw(&mut self) {
		self.view.fill(self.background_colour);

		let prod = ( self.view.rect.height*self.view.rect.width ) as usize;
		let count = self.count(&self.text);

		let mut text_copy = self.text.clone();

		if count > prod {
			let remove = (((count - prod) / (self.view.rect.width as usize + 1)) + 1) * self.view.rect.width as usize; // amount of physical tiles taken up
			let text_to_remove = self.get_text_to_remove(&self.text, remove as i32);
			text_copy = self.text[text_to_remove..self.text.len()].to_string();
			//println!("{}", text);
		}

		let mut x = 0;
		let mut y = 0;

		for i in 0..text_copy.len() {
			let c = text_copy.as_bytes()[i as usize] as char;
			if c == '\n' || c == '\r' {
				y += 1;
				x = 0;
			}
			else if c == '\t' {
				x += 4;
			}
			else {
				let t = Tile::new_text_background(c, self.text_colour, Colour::Transparent);
				let p = Point::new( (x as i32) % self.view.rect.width,
									(self.view.rect.height - 1 - x as i32 / self.view.rect.width) - y);

				self.view.context.set_tiles( p, t );
				x += 1;
			}
		}
	}
}

impl Castable<TextBox> for TextBox {
	fn cast(view: &ViewTypes) -> Option<&TextBox> {
		if let &ViewTypes::TextBox(ref v) = view {
			return Some(v);
		}
		None
	}

	fn cast_mut(view: &mut ViewTypes) -> Option<&mut TextBox> {
		if let &mut ViewTypes::TextBox(ref mut v) = view {
			return Some(v);
		}
		None
	}
}

impl Drawable for TextBox {
	fn get_tile(&self, position: Point) -> Option<Tile> {
		self.view.context.get_tile(position)
	}

	fn draw_tile(&mut self, position: Point, tile: Tile) {
		self.view.context.set_tiles(position, tile);
	}

	fn fill(&mut self, colour: Colour) {
		self.view.fill(colour);
	}

	fn fill_rect(&mut self, position: Point, rect: Rect, colour: Colour) {
		for i in 0..rect.width {
			for j in 0..rect.height {
				self.draw_tile(Point::new(position.x + i, position.y + j), Tile::new_background(colour));
			}
		}
	}
}

pub enum ViewTypes {
	DefaultView(DefaultView),
	TextBox(TextBox),
}

impl Drawable for ViewTypes {
	fn get_tile(&self, position: Point) -> Option<Tile> {
		match self {
			&ViewTypes::DefaultView(ref v) => v.get_tile(position),
			&ViewTypes::TextBox(ref v) => v.get_tile(position),
		}
	}

	fn draw_tile(&mut self, position: Point, tile: Tile) {
		match self {
			&mut ViewTypes::DefaultView(ref mut v) => v.draw_tile(position, tile),
			&mut ViewTypes::TextBox(ref mut v) => v.draw_tile(position, tile)
		}
	}

	fn fill(&mut self, colour: Colour) {
		match self {
			&mut ViewTypes::DefaultView(ref mut v) => v.fill(colour),
			&mut ViewTypes::TextBox(ref mut v) => v.fill(colour)
		}
	}

	fn fill_rect(&mut self, position: Point, rect: Rect, colour: Colour) {
		for i in 0..rect.width {
			for j in 0..rect.height {
				self.draw_tile(Point::new(position.x + i, position.y + j), Tile::new_background(colour));
			}
		}
	}
}

pub struct View {
	pub id: String,
	pub rect: Rect,
	pub position: Point,
	pub view: ViewTypes,
	subviews: Vec<View>
}

impl View {
	pub fn new(id: String, rect: Rect, position: Point, view_type: ViewTypes) -> View {
		View {
			id: id,
			rect: rect,
			position: position,
			view: view_type,
			subviews: Vec::new()
		}
	}

	pub fn new_textbox(id: String, rect: Rect, position: Point, text: String, text_colour: Colour, background_colour: Colour) -> View {
		View {
			id: id,
			rect: rect,
			position: position,
			view: ViewTypes::TextBox(TextBox::new(rect, text, text_colour, background_colour)),
			subviews: Vec::new()
		}
	}

	pub fn new_default(id: String, rect: Rect, position: Point) -> View {
		View {
			id: id,
			rect: rect,
			position: position,
			view: ViewTypes::DefaultView(DefaultView::new(rect)),
			subviews: Vec::new()
		}
	}

	pub fn add_subview(&mut self, view: View) {
		self.subviews.push(view);
	}

	pub fn get_subview(&mut self, id: String) -> Option<&mut View> {
		let i = self.subviews.iter().position(|ref r| r.id == id);
		if let Some(index) = i {
			return Some(&mut self.subviews[index]);
		}
		None
	}

	pub fn draw(&mut self) {
		for s in &mut self.subviews {
			s.draw();

			let subview_width_position = s.position.x + s.rect.width;
			let subview_height_position = s.position.y + s.rect.height;

			let width_bound = match subview_width_position >= self.rect.width {
	 			true => self.rect.width,
				false => subview_width_position
			};
			let height_bound = match subview_height_position >= self.rect.height {
	 			false => self.rect.height,
				true => subview_height_position
			};

			for i in 0..width_bound {
				for j in 0..height_bound {
					let t = s.view.get_tile(Point::new(i, j));
					if let Some(tile) = t {
						self.view.draw_tile(Point::new(s.position.x + i , s.position.y + j), tile);
					}
				}
			}
		}
	}

	pub fn cast<T : Castable<T>>(view: &View) -> Option<&T> {
		return T::cast(&view.view)
	}

	pub fn cast_mut<T : Castable<T>>(view: &mut View) -> Option<&mut T> {
		return T::cast_mut(&mut view.view)
	}
}
