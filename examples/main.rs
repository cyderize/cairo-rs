extern crate cairo;

fn main() {
	let image = cairo::surface::ImageSurface::new(cairo::format::Format::AlphaRedGreenBlue32, 100, 100);
	let g = cairo::Context::new(&image);
	g.set_source_rgba(0.5, 0.5, 0.5, 1.);
	g.rectangle(10., 10., 10., 10.);
	g.fill();

	g.select_font_face("Marker Felt", cairo::common::FontSlant::Normal, cairo::common::FontWeight::Normal);
	g.set_font_size(24.);
	g.set_source_rgb(0.1, 0.1, 0.1);
	g.move_to(20., 20.);
	g.show_text("HELLO WORLD!");

	cairo::surface::Surface::write_to_png(&image, "test.png");
	println!("cairo version: {}", cairo::version::version());
}