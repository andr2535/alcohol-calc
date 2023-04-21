use iced::{
	widget::{text, text_input},
	Alignment, Element, Sandbox, Settings,
};
const ALCOHOL_DENSITY: f64 = 0.789;
#[derive(Debug)]
struct AlcoholCalculator {
	pure_alcohol_in_grams:     f64,
	alcohol_percentage_weight: f64,
}
impl AlcoholCalculator {
	fn get_alcohol_percentage_by_volume(&self) -> f64 {
		(self.alcohol_percentage_weight) /
			(ALCOHOL_DENSITY * (-self.alcohol_percentage_weight) + ALCOHOL_DENSITY + self.alcohol_percentage_weight)
	}

	fn get_total_amount_by_weight(&self) -> f64 {
		self.pure_alcohol_in_grams / self.alcohol_percentage_weight
	}

	fn get_total_amount_by_volume(&self) -> f64 {
		(self.get_total_amount_by_weight() - self.pure_alcohol_in_grams) + self.pure_alcohol_in_grams / ALCOHOL_DENSITY
	}
}
trait ToStringLowPrecision {
	fn to_lp_string(&self) -> String;
}
impl ToStringLowPrecision for f64 {
	fn to_lp_string(&self) -> String {
		format!("{:.2}", self)
	}
}

#[derive(Debug, Clone)]
enum Message {
	PureAlcoholAmountChanged(String),
	AlcoholPercentageWeightChanged(String),
	AlcoholPercentageVolumeChanged(String),
	TotalAmountWeightChanged(String),
	TotalAmountVolumeChanged(String),
}

impl Sandbox for AlcoholCalculator {
	type Message = Message;

	fn new() -> AlcoholCalculator {
		AlcoholCalculator { pure_alcohol_in_grams: 0f64, alcohol_percentage_weight: 0.031829 }
	}

	fn title(&self) -> String {
		String::from("Alcohol calculator")
	}

	fn update(&mut self, message: Message) {
		match message {
			Message::PureAlcoholAmountChanged(new_pure_alcohol_percentage_string) => {
				self.pure_alcohol_in_grams = new_pure_alcohol_percentage_string.parse::<f64>().unwrap_or(self.pure_alcohol_in_grams);
			},
			Message::AlcoholPercentageWeightChanged(new_alcohol_percentage_by_weight_string) => {
				let new_alcohol_percentage_by_weight = new_alcohol_percentage_by_weight_string
					.parse::<f64>()
					.unwrap_or(self.alcohol_percentage_weight)
					.clamp(0f64, 100f64);
				self.alcohol_percentage_weight = new_alcohol_percentage_by_weight / 100f64;
			},
			Message::AlcoholPercentageVolumeChanged(new_alcohol_percentage_by_volume_string) => {
				let new_alcohol_percentage_by_volume = new_alcohol_percentage_by_volume_string
					.parse::<f64>()
					.unwrap_or(self.get_alcohol_percentage_by_volume() * 100f64)
					.clamp(0f64, 100f64);
				let t = new_alcohol_percentage_by_volume * ALCOHOL_DENSITY;
				self.alcohol_percentage_weight = t / (100f64 - new_alcohol_percentage_by_volume + t)
			},
			Message::TotalAmountWeightChanged(new_total_weight_string) => {
				let new_total_weight = new_total_weight_string.parse::<f64>().unwrap_or(self.get_total_amount_by_weight());
				self.pure_alcohol_in_grams = new_total_weight * self.alcohol_percentage_weight;
			},
			Message::TotalAmountVolumeChanged(new_total_volume_string) => {
				let new_total_volume = new_total_volume_string.parse::<f64>().unwrap_or(self.get_total_amount_by_volume());
				let volume_percent = self.get_alcohol_percentage_by_volume();
				self.pure_alcohol_in_grams = (new_total_volume * volume_percent) * ALCOHOL_DENSITY;
			},
		}
	}

	fn view(&self) -> Element<Message> {
		use iced::widget::{column, row};
		column![
			text("Pure alcohol in grams:"),
			row![
				text_input("Pure alcohol in grams", self.pure_alcohol_in_grams.to_lp_string().as_ref())
					.on_input(Message::PureAlcoholAmountChanged),
				text("gram")
			]
			.spacing(10),
			text("Alcohol percent by weight:"),
			row![
				text_input("Alcohol percent by weight", (self.alcohol_percentage_weight * 100f64).to_lp_string().as_ref())
					.on_input(Message::AlcoholPercentageWeightChanged),
				text("%")
			]
			.spacing(10),
			text("Total liquid amount in grams:"),
			row![
				text_input("Total amount by weight", self.get_total_amount_by_weight().to_lp_string().as_ref())
					.on_input(Message::TotalAmountWeightChanged),
				text("gram")
			]
			.spacing(10),
			text("Alcohol percent by volume:"),
			row![
				text_input(
					"Alcohol percent by volume",
					(self.get_alcohol_percentage_by_volume() * 100f64).to_lp_string().as_ref()
				)
				.on_input(Message::AlcoholPercentageVolumeChanged),
				text("%")
			]
			.spacing(10),
			text("Total liquid amount in milliliters:"),
			row![
				text_input("Total amount by volume", self.get_total_amount_by_volume().to_lp_string().as_ref())
					.on_input(Message::TotalAmountVolumeChanged),
				text("ml")
			]
			.spacing(10),
		]
		.padding(20)
		.align_items(Alignment::Start)
		.into()
	}
}

pub fn main() -> iced::Result {
	AlcoholCalculator::run(Settings::default())
}
