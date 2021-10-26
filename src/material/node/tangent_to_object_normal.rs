use std::collections::HashMap;
use crate::{
	material::node::node::{
		MaterialNode,
		UniformContents,
	},
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
};

const FUNCTION_CHUNK: &str = "
fn perturb_normal_to_arb(
  eye_pos: vec3<f32>,
  surf_norm: vec3<f32>,
  map_n: vec3<f32>,
  uv: vec2<f32>
) -> vec3<f32> {
  let q0: vec3<f32> = vec3<f32>(dpdx(eye_pos.x), dpdx(eye_pos.y), dpdx(eye_pos.z));
  let q1: vec3<f32> = vec3<f32>(dpdy(eye_pos.x), dpdy(eye_pos.y), dpdy(eye_pos.z));
  let st0: vec2<f32> = dpdx(uv);
  let st1: vec2<f32> = dpdy(uv);
  let n: vec3<f32> = surf_norm; // normalized
  let q1perp: vec3<f32> = cross(q1, n);
  let q0perp: vec3<f32> = cross(n, q0);
  let t = q1perp * st0.x + q0perp * st1.x;
  let b = q1perp * st0.y + q0perp * st1.y;
  let det: f32 = max(dot(t, t), dot(b, b));
  var scale: f32;
  if (det == 0.0) {
    scale = 0.0;
  } else {
    scale = inverseSqrt(det);
  }
  return normalize(t * (map_n.x * scale) + b * (map_n.y * scale) + n * map_n.z);
}
";

pub struct TangentToObjectNormalNode {
	node: ResourceId<Box<dyn MaterialNode>>,
}

impl TangentToObjectNormalNode {
	pub fn new(
		node: ResourceId<Box<dyn MaterialNode>>,
	) -> Self {
		TangentToObjectNormalNode {
			node: node,
		}
	}
}

impl MaterialNode for TangentToObjectNormalNode {
	fn collect_nodes (
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<ResourceId<Box<dyn MaterialNode>>>,
		visited: &mut HashMap<ResourceId<Box<dyn MaterialNode>>, bool>,
		self_rid: ResourceId<Box<dyn MaterialNode>>,
	) {
		pool.borrow(&self.node).unwrap().collect_nodes(
			pool, nodes, visited, self.node,
		);
		if !visited.contains_key(&self_rid) {
			visited.insert(self_rid, true);
			nodes.push(self_rid);
		}
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		None
	}

	fn build_declaration(&self, _self_id: usize) -> String {
		format!("")
	}

	fn build_functions(&self, _self_id: usize) -> String {
		// @TODO: Add self_id suffix for unique function name
		FUNCTION_CHUNK.to_string()
	}

	fn build_fragment_shader(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
		visited: &mut HashMap<usize, bool>,
		self_id: usize,
	) -> String {
		if visited.contains_key(&self_id) {
			return "".to_string();
		}
		visited.insert(self_id, true);

		let node = pool.borrow(&self.node).unwrap();

		node.build_fragment_shader(pool, visited, self.node.id) +
		&format!("let {} = perturb_normal_to_arb(-in.view_position, in.normal, {}, in.uv);\n",
			self.get_fragment_output(self_id),
			node.get_fragment_output(self.node.id),
		)
	}

	fn get_fragment_output(&self, self_id: usize) -> String {
		format!("tangent_to_object_normal_output_{}", self_id)
	}
}
