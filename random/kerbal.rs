use bmp::{px, Image, Pixel};

struct Rect {
	color: (u32, u32, u32),
	origin: (u32, u32),
	width: u32,
	height: u32,
}

struct Picture {
	size: [u32; 2],
	file_path: &'static str,
	data: Image,
}
impl Picture {
	fn get_size(&self) -> [u32; 2] {
		self.size
	}
	fn paint(&mut self, s: &Rect) {
		for (x, y) in self.data.coordinates() {
			if 	(x >= s.origin.0 && x <= s.origin.0+s.width) && 
				(y >= s.origin.1 && y <= s.origin.1+s.height) {
				
				self.data.set_pixel(x, y, px!(s.color.0, s.color.1, s.color.2));
			}
		}
	}
	fn save(&self) {
		let _ = self.data.save(format!("{}.bmp", self.file_path));
	}
}

enum Colors {
	Gween,
	White,
	Black,
}
impl Colors {
	fn get(c: Colors) -> (u32, u32, u32) {
		match c {
			Colors::Gween => (80, 150, 20),
			Colors::White => (200, 200, 200),
			Colors::Black => (50, 50, 50),
		}
	}
}

fn main() {
	let mut pic = Picture {
		size: [640, 480],
		file_path: "kerbal",
		data: Image::new(640, 480),
	};
	
	// Define Rects to paint
	
	let bg = Rect {
		color: Colors::get(Colors::Gween),
		origin: (0,0),
		width: pic.get_size()[0],
		height: pic.get_size()[1],
	};
	
	let eyes = [Rect {
		color: Colors::get(Colors::White),
		origin: (120, 100),
		width: 100,
		height: 180,
	},
				Rect {
		color: Colors::get(Colors::White),
		origin: (420, 100),
		width: 100,
		height: 150,
	}];
	
	let pupils = [Rect {
		color: Colors::get(Colors::Black),
		origin: (130, 110),
		width: 40,
		height: 40,
	},
				Rect {
		color: Colors::get(Colors::Black),
		origin: (430, 110),
		width: 40,
		height: 40,
	}];
	
	pic.paint(&bg);
	pic.paint(&eyes[0]);
	pic.paint(&eyes[1]);
	pic.paint(&pupils[0]);
	pic.paint(&pupils[1]);
	
	pic.save();
}
