// use tile::Tile;
// use tile::Colour;
// use context::Context;
// use geometry::Point;
// use floating::Floating;
//
//
// #[allow(dead_code)]
// pub struct Layer {
// 	pub hidden: bool,
// 	pub position: i32,
// 	pub context: Context
// }
//
// #[allow(dead_code)]
// impl Layer {
// 	pub fn new(hidden: bool, position: i32, context: Context) -> Layer {
// 		Layer {
// 			hidden: hidden,
// 			position: position,
// 			context: context
// 		}
// 	}
//
// 	pub fn draw(&mut self, p: Point, t: Tile) {
// 		if self.context.point_within_context(p) {
// 			self.context.tiles[(self.context.rect.height-p.y-1) as usize][p.x as usize] = t;
// 		}
// 	}
//
// 	pub fn draw_floating(&mut self, f: &Floating, bottom_left: Point) {
// 		let height_bound = match f.rect.height >= self.context.rect.height {
// 			true => self.context.rect.height-1,
// 			false => f.rect.height
// 		};
// 		let width_bound = match f.rect.width >= self.context.rect.width {
// 			true => self.context.rect.width-1,
// 			false => f.rect.width
// 		};
//
//
// 		for i in 0..height_bound {
// 			for j in 0..width_bound {
// 				let i_index = bottom_left.y + i;
// 				let j_index = bottom_left.x + j;
// 				self.draw(Point {x: j_index, y: i_index}, f.get_tile(j, i));
// 			}
// 		}
// 	}
// }
//
//
// #[allow(dead_code)]
// pub struct Graphics{
// 	pub width: i32,
// 	pub height: i32,
// 	context: Context,
// 	layers: Vec<Layer>,
// 	diffs: Vec<Point>
// }
//
// #[allow(dead_code)]
// impl Graphics {
//
// 	pub fn new(width: i32, height: i32) -> Graphics {
// 		Graphics {
// 			width: width,
// 			height: height,
// 			context: Context::new(width, height),
// 			layers: vec!(Layer::new(false, 0, Context::new(width, height))),
// 			diffs: vec!()
// 		}
// 	}
//
// 	pub fn number_of_layers(&self) -> i32 {
// 		self.layers.len() as i32
// 	}
//
// 	pub fn draw_layer(&mut self, p: Point, tile: Tile, layer: i32) {
// 		self.layers[layer as usize].draw(p, tile);
// 		self.diffs.push(p);
// 	}
//
// 	pub fn draw_floating(&mut self, f: &Floating, bottom_left: Point, layer: i32) {
// 		self.layers[layer as usize].draw_floating(f, bottom_left);
// 		// let height_bound = match f.rect.height > self.context.rect.height {
// 		// 	true => self.context.rect.height,
// 		// 	false => f.rect.height
// 		// };
// 		// let width_bound = match f.rect.height > self.context.rect.height {
// 		// 	true => self.context.rect.width,
// 		// 	false => f.rect.width
// 		// };
// 		//
// 		// for i in 0..height_bound {
// 		// 	for j in 0..width_bound {
// 		// 		let i_index = bottom_left.y + i;
// 		// 		let j_index = bottom_left.x + j;
// 		// 		self.diffs.push( Point {x: j_index, y: i_index} );
// 		// 	}
// 		// }
// 	}
//
// 	pub fn add_layer(&mut self) {
// 		let n = self.number_of_layers();
// 		self.layers.push(Layer::new(false, n, Context::new(self.width, self.height)));
// 	}
//
// 	pub fn remove_layer(&mut self, layer: i32) {
// 		if layer >= 0 && layer < self.number_of_layers() {
// 			self.layers.remove(layer as usize);
// 		}
// 	}
//
// 	pub fn hide_layer(&mut self, layer: i32) {
// 		self.layers[layer as usize].hidden = true;
// 	}
//
// 	pub fn move_layer_to(&mut self, source: i32, destination: i32) {
// 		if source < self.number_of_layers() && destination < self.number_of_layers() {
// 			let source_layer = self.layers.remove(source as usize);
// 			self.layers.insert(destination as usize, source_layer);
// 		}
// 	}
//
// 	pub fn clear(&mut self) {
// 		for l in &mut self.layers {
// 			if l.hidden == false {
// 				for i in 0..self.height {
// 		        	for j in 0..self.width {
// 						l.context.tiles[i as usize][j as usize] = Tile::new1(' ', Colour::White);
// 		        	}
// 		    	}
// 			}
// 		}
// 	}
//
// 	pub fn render(&mut self) {
// 		print!("\x1B[?25l"); //hide cursor
//     	print!("\x1B[2J");   //clear screen
//     	print!("\x1B[3J\x1B[1;1H"); //remove history
//
// 		for l in &self.layers {
// 			if l.hidden == false {
// 				for i in 0..self.height {
// 		        	for j in 0..self.width {
// 						let tile = l.context.tiles[i as usize][j as usize];
// 						tile.print_tile();
//
// 		            	print!("\x1B[0m");  //reset colours
// 		        	}
// 		        	print!("\n");
// 		    	}
// 			}
// 		}
//
//     	print!("\x1B[?25h"); //show cursor
// 		self.clear();
// 	}
// }
