pub enum WrapMode {
	ClampToBorder,
	ClampToEdge,
	MirrorRepeat,
	Repeat,
}

pub enum FilterMode {
	Linear,
	Nearest,	
}

pub struct Sampler {
	mag_filter: FilterMode,
	min_filter: FilterMode,
	mipmap_filter: FilterMode,
	wrap_u: WrapMode,
	wrap_v: WrapMode,
	wrap_w: WrapMode,
}

pub struct SamplerDescriptor {
	pub mag_filter: Option<FilterMode>,
	pub min_filter: Option<FilterMode>,
	pub mipmap_filter: Option<FilterMode>,
	pub wrap_u: Option<WrapMode>,
	pub wrap_v: Option<WrapMode>,
	pub wrap_w: Option<WrapMode>,
}

impl Sampler {
	pub fn new(desc: SamplerDescriptor) -> Self {
		// @TODO: Fix default parameters
		Sampler {
			mag_filter: match desc.mag_filter {
				Some(filter) => filter,
				None => FilterMode::Linear,
			},
			min_filter: match desc.min_filter {
				Some(filter) => filter,
				None => FilterMode::Linear,
			},
			mipmap_filter: match desc.mipmap_filter {
				Some(filter) => filter,
				None => FilterMode::Linear,
			},
			wrap_u: match desc.wrap_u {
				Some(wrap) => wrap,
				None => WrapMode::ClampToEdge,
			},
			wrap_v: match desc.wrap_v {
				Some(wrap) => wrap,
				None => WrapMode::ClampToEdge,
			},
			wrap_w: match desc.wrap_w {
				Some(wrap) => wrap,
				None => WrapMode::ClampToEdge,
			},
		}
	}

	pub fn mag_filter(&self) -> &FilterMode {
		&self.mag_filter
	}

	pub fn min_filter(&self) -> &FilterMode {
		&self.min_filter
	}

	pub fn mipmap_filter(&self) -> &FilterMode {
		&self.mipmap_filter
	}

	pub fn wrap_u(&self) -> &WrapMode {
		&self.wrap_u
	}

	pub fn wrap_v(&self) -> &WrapMode {
		&self.wrap_v
	}

	pub fn wrap_w(&self) -> &WrapMode {
		&self.wrap_w
	}
}
