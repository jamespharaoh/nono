pub trait IntoDefault {
	fn into_default (self) -> Self;
}

impl <Item> IntoDefault for Vec <Item> {

	fn into_default (mut self) -> Vec <Item> {

		self.truncate (0);

		self

	}

}

pub trait IntoExtend <Item> {

	fn into_extend <Source: IntoIterator <Item = Item>> (
		self, 
		source: Source,
	) -> Self;

}

impl <Item> IntoExtend <Item> for Vec <Item> {

	fn into_extend <Source: IntoIterator <Item = Item>> (
		mut self,
		source: Source,
	) -> Vec <Item> {

		self.extend (source);

		self

	}

}
 
