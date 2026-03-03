#[derive(Debug, Clone,PartialEq,Eq)]
pub enum PassKind {
	Zero,
	One,
	Pattern(u8),
	ThreeBytePattern([u8; 3]),
	Random,
}

#[derive(Debug, Clone,PartialEq,Eq)]
pub struct PassInfo {
	pub kind: PassKind,
}

#[derive(Debug, Clone,PartialEq,Eq)]
pub struct AnalysisReport {
	pub pass_count: usize,
	pub passes: Vec<PassInfo>,
}