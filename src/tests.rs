use crate::*;

#[test]
fn test_color() {
	assert_eq!(
		iced::Color::from(HexColor(0x8160dc)),
		iced::Color::from_rgb8(0x81, 0x60, 0xdc)
	);
}
