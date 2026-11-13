//! [`BakedQuad`]: a baked face's quad. Upstream's terrain models read the quad's material, so the
//! fork's slimmed copy (without the material) became the full [`ModelQuad`] again in the merge.
pub type BakedQuad = super::ModelQuad;
