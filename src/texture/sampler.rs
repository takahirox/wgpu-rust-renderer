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
	pub mag_filter: FilterMode,
	pub min_filter: FilterMode,
	pub mipmap_filter: FilterMode,
	pub wrap_u: WrapMode,
	pub wrap_v: WrapMode,
	pub wrap_w: WrapMode,
}

impl Default for SamplerDescriptor {
	fn default() -> Self {
		SamplerDescriptor {
			mag_filter: FilterMode::Linear,
			min_filter: FilterMode::Linear,
			mipmap_filter: FilterMode::Linear,
			wrap_u: WrapMode::ClampToEdge,
			wrap_v: WrapMode::ClampToEdge,
			wrap_w: WrapMode::ClampToEdge,
		}
	}
}

impl Sampler {
	pub fn new(desc: SamplerDescriptor) -> Self {
		// @TODO: Fix default parameters
		Sampler {
			mag_filter: desc.mag_filter,
			min_filter: desc.min_filter,
			mipmap_filter: desc.mipmap_filter,
			wrap_u: desc.wrap_u,
			wrap_v: desc.wrap_v,
			wrap_w: desc.wrap_w,
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
