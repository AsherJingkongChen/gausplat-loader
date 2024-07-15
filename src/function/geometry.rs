pub fn rotation_matrix_from_quaternion(quaternion: &[f64; 4]) -> [[f64; 3]; 3] {
        let [r0, r1, r2, r3] = quaternion;
        let r1_r1 = r1 * r1 * 2.0;
        let r2_r2 = r2 * r2 * 2.0;
        let r3_r3 = r3 * r3 * 2.0;
        let r0_r1 = r0 * r1 * 2.0;
        let r0_r2 = r0 * r2 * 2.0;
        let r0_r3 = r0 * r3 * 2.0;
        let r1_r2 = r1 * r2 * 2.0;
        let r1_r3 = r1 * r3 * 2.0;
        let r2_r3 = r2 * r3 * 2.0;
        [
            [1.0 - r2_r2 - r3_r3, r1_r2 - r0_r3, r1_r3 + r0_r2],
            [r1_r2 + r0_r3, 1.0 - r1_r1 - r3_r3, r2_r3 - r0_r1],
            [r1_r3 - r0_r2, r2_r3 + r0_r1, 1.0 - r1_r1 - r2_r2],
        ]
}