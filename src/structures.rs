use drowsed_math::FVec2;

pub struct GlobalUBO {
    pub resolution: FVec2,
}
pub struct Curve {
    pub p: [FVec2; 3],
}
impl Curve {
    pub fn curve_setup() -> Vec<Curve> {
        vec![
            Curve {
                p: [  
                    FVec2::new(1.0, 1.0),
                    FVec2::new(0.0, 0.0),
                    FVec2::new(1.0, 0.0),
                ],
            },
        ]
    }
}