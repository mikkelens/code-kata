use std::collections::{BTreeMap, HashSet};

pub type Item = char;
pub type Dependencies = HashSet<Item>;
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn parse_to_map(s: &str) -> BTreeMap<Item, Dependencies> {
	let map: BTreeMap<Item, Dependencies> = s
		.lines()
		.map(|line| {
			let (first, second) = line.split_once("  ").unwrap();
			(
				{
					assert_eq!(first.chars().count(), 1);
					first.chars().next().unwrap()
				},
				second.chars().filter(|c| c != &' ').collect()
			)
		})
		.collect();
	map
}
pub fn expand_dependencies(map: &mut BTreeMap<Item, Dependencies>) {
	fn gather_dependencies_recursive(
		source: Item,
		target: Item,
		map: &BTreeMap<Item, Dependencies>
	) -> Dependencies {
		let Some(direct_known_dependencies) = map.get(&target) else {
            return HashSet::new();
        };
		let mut more_non_source_dependencies: Dependencies = direct_known_dependencies
			.iter()
			.filter(|&dep| dep != &source)
			.copied()
			.collect::<HashSet<_>>();
		for sub_dependency in more_non_source_dependencies.clone() {
			more_non_source_dependencies.extend(gather_dependencies_recursive(
				source,
				sub_dependency,
				map
			));
		}
		more_non_source_dependencies
	}

	let map_unchanged = map.clone();
	for (dependency, sub_dependencies) in map.iter_mut() {
		sub_dependencies.extend(gather_dependencies_recursive(
			*dependency,
			*dependency,
			&map_unchanged
		));
	}
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;

	use super::{Item, *};

	const SAMPLE_1_INPUT_STR: &str = include_str!("sample_1_input.txt");
	#[test]
	fn input_parsing_works() {
		let automatic_map: BTreeMap<Item, Dependencies> = parse_to_map(SAMPLE_1_INPUT_STR);
		let manual_map: BTreeMap<Item, Dependencies> = BTreeMap::from([
			('A', HashSet::from(['B', 'C'])),
			('B', HashSet::from(['C', 'E'])),
			('C', HashSet::from(['G'])),
			('D', HashSet::from(['A', 'F'])),
			('E', HashSet::from(['F'])),
			('F', HashSet::from(['H']))
		]);
		assert_eq!(automatic_map, manual_map);
	}

	const SAMPLE_1_EXPECTATION_STR: &str = include_str!("sample_1_expectation.txt");
	#[test]
	fn sample_1_expectation_matches_input() {
		let mut input_map: BTreeMap<Item, Dependencies> = parse_to_map(SAMPLE_1_INPUT_STR);
		expand_dependencies(&mut input_map);
		let expectation_map: BTreeMap<Item, Dependencies> = parse_to_map(SAMPLE_1_EXPECTATION_STR);
		assert_eq!(input_map, expectation_map);
	}

	const SAMPLE_2_INPUT_STR: &str = include_str!("sample_2_input.txt");
	const SAMPLE_2_EXPECTATION_STR: &str = include_str!("sample_2_expectation.txt");
	#[test]
	fn sample_2_expectation_matches_input() {
		let mut input_map: BTreeMap<Item, Dependencies> = parse_to_map(SAMPLE_2_INPUT_STR);
		expand_dependencies(&mut input_map);
		let expectation_map: BTreeMap<Item, Dependencies> = parse_to_map(SAMPLE_2_EXPECTATION_STR);
		assert_eq!(input_map, expectation_map);
	}
}
