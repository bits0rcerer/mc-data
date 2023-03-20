use valence::prelude::BlockPos;

use crate::density_function::DensityFunction;

pub struct YClampedGradient {
    from: [i32; 2],
    to: [f64; 2],
}

impl YClampedGradient {
    pub fn new(from_y: i32, to_y: i32, from_value: f64, to_value: f64) -> eyre::Result<Box<dyn DensityFunction>> {
        Ok(Box::new(
            Self {
                from: [from_y, to_y],
                to: [from_value, to_value],
            }
        ))
    }
}

impl DensityFunction for YClampedGradient {
    fn compute(&self, pos: BlockPos) -> f64 {
        let v = i32::clamp(pos.y, self.from[0], self.from[1]) - self.from[0];
        let v = (v as f64) / ((self.from[1] - self.from[0]) as f64);
        self.to[0] + (v * (self.to[1] - self.to[0]))
    }

    fn map(&self, _: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        todo!()
    }

    fn min(&self) -> f64 {
        f64::min(self.to[0], self.to[1])
    }

    fn max(&self) -> f64 {
        f64::max(self.to[0], self.to[1])
    }
}

#[cfg(test)]
mod test {
    use valence::prelude::BlockPos;

    use crate::density_function::y_clamped_gradient::YClampedGradient;
    use crate::random::xoroshiro::XoroshiroRandom;

    #[test]
    fn y_clamped_gradient_test() {
        /*
        Java Code to generate sample

        var size = 128;
        var r = new XoroshiroRandomSource(0x786b544d6f473757L);
        var f = DensityFunctions.yClampedGradient(-64, 64, -1, 2);

        var b = new StringBuilder();
        b.append("\nlet sample = vec![");
        for (int i = 0; i < size; i++) {
            if (i > 0) b.append(", ");
            b.append(String.format("%.17f_f64", f.compute(new DensityFunction.FunctionContext() {
                @Override
                public int blockX() {
                    return 0;
                }

                @Override
                public int blockY() {
                    return r.nextIntBetweenInclusive(-128, 128);
                }

                @Override
                public int blockZ() {
                    return 0;
                }
            })));
        }
        b.append("];\n");

        System.out.println(b.toString());
         */

        let sample = vec![1.20312500000000000_f64, -1.00000000000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, -1.00000000000000000_f64, 1.01562500000000000_f64, 2.00000000000000000_f64, 1.50781250000000000_f64, 2.00000000000000000_f64, 0.33593750000000000_f64, 0.73437500000000000_f64, -1.00000000000000000_f64, 2.00000000000000000_f64, -0.95312500000000000_f64, -0.76562500000000000_f64, 2.00000000000000000_f64, 0.33593750000000000_f64, 0.52343750000000000_f64, 0.52343750000000000_f64, -1.00000000000000000_f64, 2.00000000000000000_f64, 1.13281250000000000_f64, -1.00000000000000000_f64, -1.00000000000000000_f64, 1.69531250000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, 0.14843750000000000_f64, 1.88281250000000000_f64, 0.03125000000000000_f64, -0.69531250000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, -1.00000000000000000_f64, 2.00000000000000000_f64, 0.05468750000000000_f64, 2.00000000000000000_f64, 1.60156250000000000_f64, 2.00000000000000000_f64, 0.33593750000000000_f64, 0.80468750000000000_f64, -0.39062500000000000_f64, -0.22656250000000000_f64, 2.00000000000000000_f64, 0.19531250000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, -0.97656250000000000_f64, 0.52343750000000000_f64, 1.43750000000000000_f64, 0.85156250000000000_f64, -1.00000000000000000_f64, 1.53125000000000000_f64, 0.99218750000000000_f64, 2.00000000000000000_f64, 1.62500000000000000_f64, 2.00000000000000000_f64, 1.08593750000000000_f64, 2.00000000000000000_f64, -0.50781250000000000_f64, -1.00000000000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, -1.00000000000000000_f64, -1.00000000000000000_f64, -0.67187500000000000_f64, 2.00000000000000000_f64, 1.32031250000000000_f64, -1.00000000000000000_f64, 1.17968750000000000_f64, 0.75781250000000000_f64, 0.64062500000000000_f64, 2.00000000000000000_f64, -1.00000000000000000_f64, 2.00000000000000000_f64, 0.64062500000000000_f64, -1.00000000000000000_f64, 0.59375000000000000_f64, 0.71093750000000000_f64, 1.43750000000000000_f64, -0.46093750000000000_f64, 1.53125000000000000_f64, -0.53125000000000000_f64, 0.68750000000000000_f64, -1.00000000000000000_f64, -1.00000000000000000_f64, -1.00000000000000000_f64, 0.71093750000000000_f64, 0.33593750000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, -1.00000000000000000_f64, 0.05468750000000000_f64, 1.43750000000000000_f64, 1.97656250000000000_f64, -1.00000000000000000_f64, 2.00000000000000000_f64, 1.25000000000000000_f64, 1.29687500000000000_f64, -1.00000000000000000_f64, -0.08593750000000000_f64, -1.00000000000000000_f64, 0.78125000000000000_f64, -0.32031250000000000_f64, -1.00000000000000000_f64, 0.26562500000000000_f64, 1.27343750000000000_f64, -1.00000000000000000_f64, -0.95312500000000000_f64, -1.00000000000000000_f64, 1.50781250000000000_f64, 0.45312500000000000_f64, -1.00000000000000000_f64, 0.75781250000000000_f64, 2.00000000000000000_f64, -1.00000000000000000_f64, 0.45312500000000000_f64, -1.00000000000000000_f64, -0.46093750000000000_f64, 2.00000000000000000_f64, 1.88281250000000000_f64, 2.00000000000000000_f64, 2.00000000000000000_f64, 1.97656250000000000_f64, -1.00000000000000000_f64, 1.29687500000000000_f64];
        let mut r = XoroshiroRandom::new(0x786b544d6f473757_i64);
        let f = YClampedGradient::new(-64, 64, -1.0, 2.0).unwrap();

        for s in sample {
            assert_eq!(s, f.compute(BlockPos::new(0, r.next_i32_between_inclusive((-128, 128)), 0)))
        }
    }

    #[test]
    fn y_clamped_gradient_overworld_range_test() {
        let sample = vec![1.00000000000000000_f64, 0.99218750000000000_f64, 0.98437500000000000_f64, 0.97656250000000000_f64, 0.96875000000000000_f64, 0.96093750000000000_f64, 0.95312500000000000_f64, 0.94531250000000000_f64, 0.93750000000000000_f64, 0.92968750000000000_f64, 0.92187500000000000_f64, 0.91406250000000000_f64, 0.90625000000000000_f64, 0.89843750000000000_f64, 0.89062500000000000_f64, 0.88281250000000000_f64, 0.87500000000000000_f64, 0.86718750000000000_f64, 0.85937500000000000_f64, 0.85156250000000000_f64, 0.84375000000000000_f64, 0.83593750000000000_f64, 0.82812500000000000_f64, 0.82031250000000000_f64, 0.81250000000000000_f64, 0.80468750000000000_f64, 0.79687500000000000_f64, 0.78906250000000000_f64, 0.78125000000000000_f64, 0.77343750000000000_f64, 0.76562500000000000_f64, 0.75781250000000000_f64, 0.75000000000000000_f64, 0.74218750000000000_f64, 0.73437500000000000_f64, 0.72656250000000000_f64, 0.71875000000000000_f64, 0.71093750000000000_f64, 0.70312500000000000_f64, 0.69531250000000000_f64, 0.68750000000000000_f64, 0.67968750000000000_f64, 0.67187500000000000_f64, 0.66406250000000000_f64, 0.65625000000000000_f64, 0.64843750000000000_f64, 0.64062500000000000_f64, 0.63281250000000000_f64, 0.62500000000000000_f64, 0.61718750000000000_f64, 0.60937500000000000_f64, 0.60156250000000000_f64, 0.59375000000000000_f64, 0.58593750000000000_f64, 0.57812500000000000_f64, 0.57031250000000000_f64, 0.56250000000000000_f64, 0.55468750000000000_f64, 0.54687500000000000_f64, 0.53906250000000000_f64, 0.53125000000000000_f64, 0.52343750000000000_f64, 0.51562500000000000_f64, 0.50781250000000000_f64, 0.50000000000000000_f64, 0.49218750000000000_f64, 0.48437500000000000_f64, 0.47656250000000000_f64, 0.46875000000000000_f64, 0.46093750000000000_f64, 0.45312500000000000_f64, 0.44531250000000000_f64, 0.43750000000000000_f64, 0.42968750000000000_f64, 0.42187500000000000_f64, 0.41406250000000000_f64, 0.40625000000000000_f64, 0.39843750000000000_f64, 0.39062500000000000_f64, 0.38281250000000000_f64, 0.37500000000000000_f64, 0.36718750000000000_f64, 0.35937500000000000_f64, 0.35156250000000000_f64, 0.34375000000000000_f64, 0.33593750000000000_f64, 0.32812500000000000_f64, 0.32031250000000000_f64, 0.31250000000000000_f64, 0.30468750000000000_f64, 0.29687500000000000_f64, 0.28906250000000000_f64, 0.28125000000000000_f64, 0.27343750000000000_f64, 0.26562500000000000_f64, 0.25781250000000000_f64, 0.25000000000000000_f64, 0.24218750000000000_f64, 0.23437500000000000_f64, 0.22656250000000000_f64, 0.21875000000000000_f64, 0.21093750000000000_f64, 0.20312500000000000_f64, 0.19531250000000000_f64, 0.18750000000000000_f64, 0.17968750000000000_f64, 0.17187500000000000_f64, 0.16406250000000000_f64, 0.15625000000000000_f64, 0.14843750000000000_f64, 0.14062500000000000_f64, 0.13281250000000000_f64, 0.12500000000000000_f64, 0.11718750000000000_f64, 0.10937500000000000_f64, 0.10156250000000000_f64, 0.09375000000000000_f64, 0.08593750000000000_f64, 0.07812500000000000_f64, 0.07031250000000000_f64, 0.06250000000000000_f64, 0.05468750000000000_f64, 0.04687500000000000_f64, 0.03906250000000000_f64, 0.03125000000000000_f64, 0.02343750000000000_f64, 0.01562500000000000_f64, 0.00781250000000000_f64, 0.00000000000000000_f64, -0.00781250000000000_f64, -0.01562500000000000_f64, -0.02343750000000000_f64, -0.03125000000000000_f64, -0.03906250000000000_f64, -0.04687500000000000_f64, -0.05468750000000000_f64, -0.06250000000000000_f64, -0.07031250000000000_f64, -0.07812500000000000_f64, -0.08593750000000000_f64, -0.09375000000000000_f64, -0.10156250000000000_f64, -0.10937500000000000_f64, -0.11718750000000000_f64, -0.12500000000000000_f64, -0.13281250000000000_f64, -0.14062500000000000_f64, -0.14843750000000000_f64, -0.15625000000000000_f64, -0.16406250000000000_f64, -0.17187500000000000_f64, -0.17968750000000000_f64, -0.18750000000000000_f64, -0.19531250000000000_f64, -0.20312500000000000_f64, -0.21093750000000000_f64, -0.21875000000000000_f64, -0.22656250000000000_f64, -0.23437500000000000_f64, -0.24218750000000000_f64, -0.25000000000000000_f64, -0.25781250000000000_f64, -0.26562500000000000_f64, -0.27343750000000000_f64, -0.28125000000000000_f64, -0.28906250000000000_f64, -0.29687500000000000_f64, -0.30468750000000000_f64, -0.31250000000000000_f64, -0.32031250000000000_f64, -0.32812500000000000_f64, -0.33593750000000000_f64, -0.34375000000000000_f64, -0.35156250000000000_f64, -0.35937500000000000_f64, -0.36718750000000000_f64, -0.37500000000000000_f64, -0.38281250000000000_f64, -0.39062500000000000_f64, -0.39843750000000000_f64, -0.40625000000000000_f64, -0.41406250000000000_f64, -0.42187500000000000_f64, -0.42968750000000000_f64, -0.43750000000000000_f64, -0.44531250000000000_f64, -0.45312500000000000_f64, -0.46093750000000000_f64, -0.46875000000000000_f64, -0.47656250000000000_f64, -0.48437500000000000_f64, -0.49218750000000000_f64, -0.50000000000000000_f64, -0.50781250000000000_f64, -0.51562500000000000_f64, -0.52343750000000000_f64, -0.53125000000000000_f64, -0.53906250000000000_f64, -0.54687500000000000_f64, -0.55468750000000000_f64, -0.56250000000000000_f64, -0.57031250000000000_f64, -0.57812500000000000_f64, -0.58593750000000000_f64, -0.59375000000000000_f64, -0.60156250000000000_f64, -0.60937500000000000_f64, -0.61718750000000000_f64, -0.62500000000000000_f64, -0.63281250000000000_f64, -0.64062500000000000_f64, -0.64843750000000000_f64, -0.65625000000000000_f64, -0.66406250000000000_f64, -0.67187500000000000_f64, -0.67968750000000000_f64, -0.68750000000000000_f64, -0.69531250000000000_f64, -0.70312500000000000_f64, -0.71093750000000000_f64, -0.71875000000000000_f64, -0.72656250000000000_f64, -0.73437500000000000_f64, -0.74218750000000000_f64, -0.75000000000000000_f64, -0.75781250000000000_f64, -0.76562500000000000_f64, -0.77343750000000000_f64, -0.78125000000000000_f64, -0.78906250000000000_f64, -0.79687500000000000_f64, -0.80468750000000000_f64, -0.81250000000000000_f64, -0.82031250000000000_f64, -0.82812500000000000_f64, -0.83593750000000000_f64, -0.84375000000000000_f64, -0.85156250000000000_f64, -0.85937500000000000_f64, -0.86718750000000000_f64, -0.87500000000000000_f64, -0.88281250000000000_f64, -0.89062500000000000_f64, -0.89843750000000000_f64, -0.90625000000000000_f64, -0.91406250000000000_f64, -0.92187500000000000_f64, -0.92968750000000000_f64, -0.93750000000000000_f64, -0.94531250000000000_f64, -0.95312500000000000_f64, -0.96093750000000000_f64, -0.96875000000000000_f64, -0.97656250000000000_f64, -0.98437500000000000_f64, -0.99218750000000000_f64, -1.00000000000000000_f64, -1.00781250000000000_f64, -1.01562500000000000_f64, -1.02343750000000000_f64, -1.03125000000000000_f64, -1.03906250000000000_f64, -1.04687500000000000_f64, -1.05468750000000000_f64, -1.06250000000000000_f64, -1.07031250000000000_f64, -1.07812500000000000_f64, -1.08593750000000000_f64, -1.09375000000000000_f64, -1.10156250000000000_f64, -1.10937500000000000_f64, -1.11718750000000000_f64, -1.12500000000000000_f64, -1.13281250000000000_f64, -1.14062500000000000_f64, -1.14843750000000000_f64, -1.15625000000000000_f64, -1.16406250000000000_f64, -1.17187500000000000_f64, -1.17968750000000000_f64, -1.18750000000000000_f64, -1.19531250000000000_f64, -1.20312500000000000_f64, -1.21093750000000000_f64, -1.21875000000000000_f64, -1.22656250000000000_f64, -1.23437500000000000_f64, -1.24218750000000000_f64, -1.25000000000000000_f64, -1.25781250000000000_f64, -1.26562500000000000_f64, -1.27343750000000000_f64, -1.28125000000000000_f64, -1.28906250000000000_f64, -1.29687500000000000_f64, -1.30468750000000000_f64, -1.31250000000000000_f64, -1.32031250000000000_f64, -1.32812500000000000_f64, -1.33593750000000000_f64, -1.34375000000000000_f64, -1.35156250000000000_f64, -1.35937500000000000_f64, -1.36718750000000000_f64, -1.37500000000000000_f64, -1.38281250000000000_f64, -1.39062500000000000_f64, -1.39843750000000000_f64, -1.40625000000000000_f64, -1.41406250000000000_f64, -1.42187500000000000_f64, -1.42968750000000000_f64, -1.43750000000000000_f64, -1.44531250000000000_f64, -1.45312500000000000_f64, -1.46093750000000000_f64, -1.46875000000000000_f64, -1.47656250000000000_f64, -1.48437500000000000_f64, -1.49218750000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64, -1.50000000000000000_f64];
        let f = YClampedGradient::new(-64, 320, 1.5, -1.5).unwrap();

        for (i, &s) in sample.iter().enumerate() {
            assert_eq!(s, f.compute(BlockPos::new(0, i as i32, 0)))
        }
    }
}
