use crate::density_function::abs::abs;
use crate::density_function::add::add;
use crate::density_function::clamp::Clamp;
use crate::density_function::constant::Constant;
use crate::density_function::cube::cube;
use crate::density_function::DensityFunction;
use crate::density_function::deserialize::{DensityFunctionTree, InlineDensityFunctionTree};
use crate::density_function::half_negative::half_negative;
use crate::density_function::max::max;
use crate::density_function::min::min;
use crate::density_function::mul::mul;
use crate::density_function::quarter_negative::quarter_negative;
use crate::density_function::square::square;
use crate::density_function::squeeze::squeeze;
use crate::random::random_state::RandomState;
use crate::registry::Registry;

impl DensityFunctionTree {
    pub fn compile(&self, random_state: &RandomState) -> eyre::Result<Box<dyn DensityFunction>> {
        match self {
            DensityFunctionTree::Constant(arg) => Ok(Constant::new(*arg)),
            DensityFunctionTree::Reference(id) => random_state.registry.density_function(id.clone())?.compile(random_state),
            DensityFunctionTree::Inline(f) => f.compile(random_state),
        }
    }
}

impl InlineDensityFunctionTree {
    pub fn compile(&self, random_state: &RandomState) -> eyre::Result<Box<dyn DensityFunction>> {
        match self {
            InlineDensityFunctionTree::Constant { argument } => Ok(Constant::new(*argument)),

            InlineDensityFunctionTree::Abs { argument } => Ok(abs(argument.compile(random_state)?)),
            InlineDensityFunctionTree::Square { argument } => Ok(square(argument.compile(random_state)?)),
            InlineDensityFunctionTree::Cube { argument } => Ok(cube(argument.compile(random_state)?)),
            InlineDensityFunctionTree::HalfNegative { argument } => Ok(half_negative(argument.compile(random_state)?)),
            InlineDensityFunctionTree::QuarterNegative { argument } => Ok(quarter_negative(argument.compile(random_state)?)),
            InlineDensityFunctionTree::Squeeze { argument } => Ok(squeeze(argument.compile(random_state)?)),

            InlineDensityFunctionTree::Max { argument1, argument2 } => Ok(max(argument1.compile(random_state)?, argument2.compile(random_state)?)),
            InlineDensityFunctionTree::Min { argument1, argument2 } => Ok(min(argument1.compile(random_state)?, argument2.compile(random_state)?)),
            InlineDensityFunctionTree::Add { argument1, argument2 } => Ok(add(argument1.compile(random_state)?, argument2.compile(random_state)?)),
            InlineDensityFunctionTree::Mul { argument1, argument2 } => Ok(mul(argument1.compile(random_state)?, argument2.compile(random_state)?)),

            InlineDensityFunctionTree::Clamp { input, min, max } => Ok(Clamp::new(input.compile(random_state)?, *min, *max)),

            InlineDensityFunctionTree::Cache2D { argument } => todo!(),
            InlineDensityFunctionTree::CacheAllInCell { argument } => todo!(),
            InlineDensityFunctionTree::CacheOnce { argument } => todo!(),
            InlineDensityFunctionTree::FlatCache { argument } => todo!(),
            InlineDensityFunctionTree::Interpolated { argument } => todo!(),

            InlineDensityFunctionTree::Noise { noise, xz_scale, y_scale } => todo!(),
            InlineDensityFunctionTree::Shift { noise } => todo!(),
            InlineDensityFunctionTree::ShiftA { noise } => todo!(),
            InlineDensityFunctionTree::ShiftB { noise } => todo!(),
            InlineDensityFunctionTree::ShiftedNoise { noise, shift_x, shift_y, shift_z, xz_scale, y_scale } => todo!(),

            InlineDensityFunctionTree::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => todo!(),
            InlineDensityFunctionTree::Spline { spline } => todo!(),
            InlineDensityFunctionTree::WeirdScaledSampler { noise, input, rarity_value_mapper } => todo!(),
            InlineDensityFunctionTree::YClampedGradient { from_y, to_y, from_value, to_value } => todo!(),

            // Blending
            InlineDensityFunctionTree::BlendDensity { argument } => todo!(),
            InlineDensityFunctionTree::OldBlendNoise { xz_scale, y_scale, xz_factor, y_factor, smear_scale_multiplier } => todo!(),
            InlineDensityFunctionTree::Slide { argument } => todo!(),
        }
    }
}
