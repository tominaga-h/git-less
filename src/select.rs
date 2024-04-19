use skim::prelude::*;
use std::io::Cursor;

pub fn skim_select(source: String) -> Vec<Arc<dyn SkimItem>> {
	let options = SkimOptionsBuilder::default()
		.multi(false)
		.build()
		.unwrap();

	let item_reader = SkimItemReader::default();
	let items = item_reader.of_bufread(Cursor::new(source));

	Skim::run_with(&options, Some(items))
		.map(|out| out.selected_items)
		.unwrap_or_default()
}
