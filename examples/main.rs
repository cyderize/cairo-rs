extern crate cairo;

fn main() {
	let image = cairo::surface::ImageSurface::new(cairo::format::Format::AlphaRedGreenBlue32, 100, 100);
	let g = cairo::Context::new(&image);
	g.set_source_rgba(0.5, 0.5, 0.5, 1.);
	g.rectangle(10., 10., 10., 10.);
	g.fill();

	cairo::surface::Surface::write_to_png(&image, "test.png");
	//image.write_to_png("test.png");

	println!("cairo version: {}", cairo::version::version());
}