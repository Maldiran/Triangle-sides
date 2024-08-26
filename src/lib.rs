/*
©Maldiran
This library provides a Triangle struct that has various properties of a given triangle.

The struct's data consists of two types: Option<f64> and [Option<f64>; 3].

The Option<f64> type is used for:

    Perimeter
    Area
    Inradius
    Circumradius

The [Option<f64>; 3] data are ordered in a specific way:

    Sides: The order of sides defines the order of arrays in the whole struct.
    Heights: The height's index reflects the side's index, so that sides[i] * heights[i] / 2.0 = area.
    Angles: The angle at index i is opposite to sides[i]. Their values are given in radians. To convert them to degrees, use standard library methods such as f64.to_degrees() and f64.to_radians().
    Medians: This starts from the middle of the side of a given index.
    Sines, cosines, tangents: Their index corresponds to the angle.

A new struct is created using the method new(sides: [Option<f64>; 3], angles: [Option<f64>; 3]). It calculates all properties at creation time. Some fields of the above function can be left as None. If the triangle can be constructed from the given data, the function will return Some(Triangle), if not you will get None. Angles specified in the above function are used only to calculate sides, they are discarded later, so there can be a situation when someone inputs angle 0.5000 and gets 0.4999 from the struct. Sides always take precedence over the angles, so if you provide incorrect angle, but the side data will be sufficient, it will not be taken into consideration and the function will return Some(Triangle).

Another way to create the struct is through the new_from_sides(sides: [f64; 3]) method, which takes just the lengths of the sides. This is slightly faster than the previous method.

You can also create a blank struct with the function new_blank(sides: [f64; 3]). This will create a triangle struct with no pre-calculated properties, and all of them will be calculated on user demand.

To access the struct's data, use:
Triangle.side(i: usize) → Option<f64>: The side at index i.
Triangle.sides() → [Option<f64>; 3]: An array of sides. Triangle.perimeter() → Option<f64>
Triangle.area() → Option<f64>
Triangle.height(i: usize) → Option<f64>
Triangle.heights() → [Option<f64>; 3]
Triangle.median(i: usize) → Option<f64>
Triangle.medians() → [Option<f64>; 3]
Triangle.inradius() → Option<f64>
Triangle.circumradius() → Option<f64>
Triangle.angle(i: usize) → Option<f64>
Triangle.angles() → [Option<f64>; 3]
Triangle.sine(i: usize) → Option<f64>
Triangle.sines() → [Option<f64>; 3]
Triangle.cosine(i: usize) → Option<f64>
Triangle.cosines() → [Option<f64>; 3]
Triangle.tangent(i: usize) → Option<f64>
Triangle.tangents() → [Option<f64>; 3]

Every one of them apart from side and sides can mutate the struct apart from only displaying a value:
Triangle.area_mut() → Option<f64>: Mutates the area.
This is useful only when creating with new_blank function, since new and new_from_sides generate all the data ahead of time so there is no reason to mutate them later. You can access data created with new_blank without mutating the struct, but the data will be generated with every access.
*/
pub mod triangle_sides {

    #[derive(Copy, Clone, Debug)]
    pub struct Triangle {
        sides: [Option<f64>; 3],
        heights: [Option<f64>; 3],
        medians: [Option<f64>; 3],
        angles: [Option<f64>; 3],
        sines: [Option<f64>; 3],
        cosines: [Option<f64>; 3],
        tangents: [Option<f64>; 3],
        perimeter: Option<f64>,
        area: Option<f64>,
        inradius: Option<f64>,
        circumradius: Option<f64>,
    }

    impl Triangle {
        pub fn new(sides: [Option<f64>; 3], mut angles: [Option<f64>; 3]) -> Option<Triangle> {
            let mut side_num: u8 = 0;
            let mut angle_num: u8 = 0;
            for side in sides {
                match side {
                    Some(_) => side_num += 1,
                    None => (),
                };
            }
            for angle in angles {
                match angle {
                    Some(_) => angle_num += 1,
                    None => (),
                };
            }
            if angle_num == 2 {
                angles = Self::special_third_angle(angles);
                angle_num = 3;
            }
            if angle_num == 3 {
                Self::check_triangle_angles([
                    angles[0].unwrap(),
                    angles[1].unwrap(),
                    angles[2].unwrap(),
                ]);
            }
            let sides_complete: [f64; 3] = match (side_num, angle_num) {
                (3, _) => {
                    //all 3 sides known
                    [sides[0].unwrap(), sides[1].unwrap(), sides[2].unwrap()]
                }
                (2, 1..=3) => {
                    match (sides, angles) {
                        ([Some(s1), Some(s2), None], [_, _, Some(a)]) => {
                            [s1, s2, Self::third_side_calc((s1, s2), a)]
                        }
                        ([None, Some(s1), Some(s2)], [Some(a), _, _]) => {
                            [Self::third_side_calc((s1, s2), a), s1, s2]
                        }
                        ([Some(s1), None, Some(s2)], [_, Some(a), _]) => {
                            [s1, Self::third_side_calc((s1, s2), a), s2]
                        }
                        _ => {
                            return None;
                        } //angle other than oposite of the missing side might
                          //provide two solutions, so the function will not
                          //deciide which one of them is desired
                    }
                }
                (1, 3) => {
                    let angles_values =
                        [angles[0].unwrap(), angles[1].unwrap(), angles[2].unwrap()];
                    let angles_sines = [
                        angles_values[0].sin(),
                        angles_values[1].sin(),
                        angles_values[2].sin(),
                    ];
                    match sides {
                        [Some(s), None, None] => Self::two_sides_calc(s, 0, angles_sines),
                        [None, Some(s), None] => Self::two_sides_calc(s, 1, angles_sines),
                        [None, None, Some(s)] => Self::two_sides_calc(s, 2, angles_sines),
                        _ => {
                            return None;
                        }
                    }
                }
                _ => {
                    return None;
                }
            };
            Self::new_from_sides(sides_complete)
        }
        pub fn new_from_sides(sides: [f64; 3]) -> Option<Triangle> {
            let new = Self::new_blank(sides);
            match new {
                Some(mut new) => {
                    new.perimeter_mut();
                    new.area_mut();
                    new.heights_mut();
                    new.medians_mut();
                    new.inradius_mut();
                    new.circumradius_mut();
                    new.sines_mut();
                    new.cosines_mut();
                    new.angles_mut();
                    new.tangents_mut();
                    Some(new)
                }
                _ => None,
            }
        }
        pub fn new_blank(sides: [f64; 3]) -> Option<Triangle> {
            if Self::check_triangle_sides(sides) {
                Some(Triangle {
                    sides: [Some(sides[0]), Some(sides[1]), Some(sides[2])],
                    heights: [None; 3],
                    medians: [None; 3],
                    angles: [None; 3],
                    sines: [None; 3],
                    cosines: [None; 3],
                    tangents: [None; 3],
                    perimeter: None,
                    area: None,
                    inradius: None,
                    circumradius: None,
                })
            } else {
                None
            }
        }
        pub fn check_triangle_angles(angles: [f64; 3]) -> bool {
            use core::f64::consts::PI;
            const ROUNDING_ERROR: f64 = 0.0000000000001;
            if (angles[0] + angles[1] + angles[2] - PI).abs() < ROUNDING_ERROR {
                true
            } else {
                false
            }
        }
        pub fn check_triangle_sides(sides: [f64; 3]) -> bool {
            for side in sides {
                if side <= 0.0 {
                    return false;
                }
            }
            let maxside = Self::max_side(sides);
            if maxside > sides[0] + sides[1] + sides[2] - maxside {
                false
            } else {
                true
            }
        }
        pub fn max_side(sides: [f64; 3]) -> f64 {
            let mut maxside = 0.0;
            for side in sides {
                if side > maxside {
                    maxside = side;
                }
            }
            maxside
        }
        fn check_index(index: usize) -> bool {
            match index {
                0..3 => true,
                _ => false,
            }
        }
        pub fn side(&self, index: usize) -> Option<f64> {
            if Self::check_index(index) {
                self.sides[index]
            } else {
                None
            }
        }
        pub fn sides(&self) -> [Option<f64>; 3] {
            self.sides
        }
        pub fn perimeter(&self) -> Option<f64> {
            match (self.perimeter, self.sides) {
                (Some(perimeter), _) => Some(perimeter),
                (None, [Some(s1), Some(s2), Some(s3)]) => Some(s1 + s2 + s3),
                _ => None,
            }
        }
        pub fn perimeter_mut(&mut self) -> Option<f64> {
            self.perimeter = self.perimeter();
            self.perimeter
        }
        pub fn area(&self) -> Option<f64> {
            match (self.area, self.perimeter(), self.sides) {
                (Some(area), _, _) => Some(area),
                (None, Some(p), [Some(s0), Some(s1), Some(s2)]) => {
                    let sp = p / 2.0; // semi-perimeter
                    Some((sp * (sp - s0) * (sp - s1) * (sp - s2)).sqrt()) // heron's formula
                }
                _ => None,
            }
        }
        pub fn area_mut(&mut self) -> Option<f64> {
            if self.perimeter == None {
                self.perimeter_mut();
            }
            self.area = self.area();
            self.area
        }
        pub fn height(&self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            match (self.heights[index], self.area(), self.sides[index]) {
                (Some(h), _, _) => Some(h),
                (None, Some(a), Some(s)) => Some(a * 2.0 / s),
                _ => None,
            }
        }
        pub fn height_mut(&mut self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            if self.area == None {
                self.area_mut();
            }
            self.heights[index] = self.height(index);
            self.heights[index]
        }
        pub fn heights(&self) -> [Option<f64>; 3] {
            [self.height(0), self.height(1), self.height(2)]
        }
        pub fn heights_mut(&mut self) -> [Option<f64>; 3] {
            [self.height_mut(0), self.height_mut(1), self.height_mut(2)]
        }
        pub fn median(&self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            match (self.medians[index], self.sides) {
                (Some(m), _) => Some(m),
                (None, [Some(s0), Some(s1), Some(s2)]) => Some(
                    ((2.0 * (s0 * s0 + s1 * s1 + s2 * s2)
                        - 3.0 * self.sides[index].unwrap() * self.sides[index].unwrap())
                        / 4.0)
                        .sqrt(),
                ),
                //formula for median
                _ => None,
            }
        }
        pub fn median_mut(&mut self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            self.medians[index] = self.median(index);
            self.medians[index]
        }
        pub fn medians(&self) -> [Option<f64>; 3] {
            [self.median(0), self.median(1), self.median(2)]
        }
        pub fn medians_mut(&mut self) -> [Option<f64>; 3] {
            [self.median_mut(0), self.median_mut(1), self.median_mut(2)]
        }
        pub fn circumradius(&self) -> Option<f64> {
            match (self.circumradius, self.area(), self.sides) {
                (Some(r), _, _) => Some(r),
                (None, Some(a), [Some(s0), Some(s1), Some(s2)]) => Some(s0 * s1 * s2 / a / 4.0),
                _ => None,
            }
        }
        pub fn circumradius_mut(&mut self) -> Option<f64> {
            if self.area == None {
                self.area_mut();
            }
            self.circumradius = self.circumradius();
            self.circumradius
        }
        pub fn inradius(&self) -> Option<f64> {
            match (self.inradius, self.area(), self.perimeter()) {
                (Some(r), _, _) => Some(r),
                (None, Some(a), Some(p)) => Some(a * 2.0 / p),
                _ => None,
            }
        }
        pub fn inradius_mut(&mut self) -> Option<f64> {
            if self.area == None {
                self.area_mut();
            }
            self.inradius = self.inradius();
            self.inradius
        }
        pub fn sine(&self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            match (self.sines[index], self.sides, self.area()) {
                (Some(sin), _, _) => Some(sin),
                (None, [Some(s0), Some(s1), Some(s2)], Some(a)) => {
                    Some(a * 2.0 * self.sides[index].unwrap() / (s0 * s1 * s2))
                }
                _ => None,
            }
        }
        pub fn sine_mut(&mut self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            if self.area == None {
                self.area_mut();
            }
            self.sines[index] = self.sine(index);
            self.sines[index]
        }
        pub fn sines(&self) -> [Option<f64>; 3] {
            [self.sine(0), self.sine(1), self.sine(2)]
        }
        pub fn sines_mut(&mut self) -> [Option<f64>; 3] {
            [self.sine_mut(0), self.sine_mut(1), self.sine_mut(2)]
        }
        pub fn cosine(&self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            match (self.cosines[index], self.sides) {
                (Some(cos), _) => Some(cos),
                (None, [Some(s0), Some(s1), Some(s2)]) => Some(
                    (s0 * s0 + s1 * s1 + s2 * s2
                        - 2.0 * self.sides[index].unwrap() * self.sides[index].unwrap())
                        * self.sides[index].unwrap()
                        / (2.0 * s0 * s1 * s2),
                ),
                //form law of cosines - solution for cos
                _ => None,
            }
        }
        pub fn cosine_mut(&mut self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            self.cosines[index] = self.cosine(index);
            self.cosines[index]
        }
        pub fn cosines(&self) -> [Option<f64>; 3] {
            [self.cosine(0), self.cosine(1), self.cosine(2)]
        }
        pub fn cosines_mut(&mut self) -> [Option<f64>; 3] {
            [self.cosine_mut(0), self.cosine_mut(1), self.cosine_mut(2)]
        }
        pub fn angle(&self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            match (self.angles[index], self.cosine(index)) {
                (Some(a), _) => Some(a),
                (None, Some(cos)) => Some(cos.acos()),
                _ => None,
            }
        }
        pub fn angle_mut(&mut self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            if self.cosines[index] == None {
                self.cosine_mut(index);
            }
            self.angles[index] = self.angle(index);
            self.angles[index]
        }
        pub fn angles(&self) -> [Option<f64>; 3] {
            [self.angle(0), self.angle(1), self.angle(2)]
        }
        pub fn angles_mut(&mut self) -> [Option<f64>; 3] {
            [self.angle_mut(0), self.angle_mut(1), self.angle_mut(2)]
        }
        pub fn tangent(&self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            match (self.tangents[index], self.angle(index)) {
                (Some(tan), _) => Some(tan),
                (None, Some(a)) => Some(a.tan()),
                _ => None,
            }
        }
        pub fn tangent_mut(&mut self, index: usize) -> Option<f64> {
            if !Self::check_index(index) {
                return None;
            }
            if self.angles[index] == None {
                self.angle_mut(index);
            }
            self.tangents[index] = self.tangent(index);
            self.tangents[index]
        }
        pub fn tangents(&self) -> [Option<f64>; 3] {
            [self.tangent(0), self.tangent(1), self.tangent(2)]
        }
        pub fn tangents_mut(&mut self) -> [Option<f64>; 3] {
            [
                self.tangent_mut(0),
                self.tangent_mut(1),
                self.tangent_mut(2),
            ]
        }
        //special function only designed to calculate third angle in new() function
        fn special_third_angle(angles: [Option<f64>; 3]) -> [Option<f64>; 3] {
            use core::f64::consts::PI;
            match angles {
                [None, Some(a2), Some(a3)] => [Some(PI - a2 - a3), Some(a2), Some(a3)],
                [Some(a1), None, Some(a3)] => [Some(a1), Some(PI - a1 - a3), Some(a3)],
                [Some(a1), Some(a2), None] => [Some(a1), Some(a2), Some(PI - a1 - a2)],
                _ => angles,
            }
        }
        fn third_side_calc(sides: (f64, f64), angle: f64) -> f64 {
            //cosine law for third side
            (sides.0 * sides.0 + sides.1 * sides.1 - 2.0 * sides.0 * sides.1 * angle.cos()).sqrt()
        }
        fn two_sides_calc(side: f64, index: usize, sines: [f64; 3]) -> [f64; 3] {
            //sine law for second and third side
            let mut sides: [Option<f64>; 3] = [None; 3];
            for i in 0..3 {
                if i != index {
                    sides[i] = Some(side * sines[i] / sines[index]);
                } else {
                    sides[i] = Some(side);
                }
            }
            [sides[0].unwrap(), sides[1].unwrap(), sides[2].unwrap()]
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::triangle_sides::Triangle;
    const ROUNDING_ERROR: f64 = 0.0000000000001;

    #[test]
    fn test_egyptian() -> Result<(), String> {
        let egyptian = Triangle::new_from_sides([3.0, 4.0, 5.0]).unwrap(); //creates egyptian triangle
        assert!((egyptian.angle(2).unwrap().to_degrees() - 90.0).abs() < ROUNDING_ERROR);
        assert!((egyptian.circumradius().unwrap() - 2.5).abs() < ROUNDING_ERROR);
        assert!((egyptian.area().unwrap() - 6.0).abs() < ROUNDING_ERROR);
        assert!((egyptian.perimeter().unwrap() - 12.0).abs() < ROUNDING_ERROR);
        assert!((egyptian.height(2).unwrap() - 2.4).abs() < ROUNDING_ERROR);
        assert!((egyptian.median(2).unwrap() - 2.5).abs() < ROUNDING_ERROR);
        assert!((egyptian.inradius().unwrap() - 1.0).abs() < ROUNDING_ERROR);
        Ok(())
    }

    #[test]
    fn test_obtuse() -> Result<(), String> {
        // https://www.calculator.net/triangle-calculator.html?vc=&vx=&vy=1&va=1&vz=&vb=0.5&angleunits=r&x=Calculate
        let obtuse = Triangle::new([Some(1.0), None, None], [Some(1.0), Some(0.5), None]).unwrap(); //creates obtuse triangle
        assert!((obtuse.angle(2).unwrap() - 1.64).abs() < 0.01);
        assert!((obtuse.inradius().unwrap() - 0.21).abs() < 0.01);
        assert!((obtuse.circumradius().unwrap() - 0.59).abs() < 0.01);
        assert!((obtuse.median(2).unwrap() - 0.56).abs() < 0.01);
        assert!((obtuse.sine(0).unwrap() - 1.0_f64.sin()).abs() < ROUNDING_ERROR);
        assert!((obtuse.cosine(0).unwrap() - 1.0_f64.cos()).abs() < ROUNDING_ERROR);
        assert!((obtuse.tangent(0).unwrap() - 1.0_f64.tan()).abs() < ROUNDING_ERROR);
        Ok(())
    }

    #[test]
    fn test_acute() -> Result<(), String> {
        // https://www.calculator.net/triangle-calculator.html?vc=1&vx=1.5&vy=1&va=&vz=&vb=&angleunits=r&x=Calculate
        let acute = Triangle::new([Some(1.0), Some(1.5), None], [None, None, Some(1.0)]).unwrap(); //creates acute triangle
        assert!((acute.angle(1).unwrap() - 1.42).abs() < 0.01);
        assert!((acute.inradius().unwrap() - 0.33).abs() < 0.01);
        assert!((acute.circumradius().unwrap() - 0.76).abs() < 0.01);
        assert!((acute.median(2).unwrap() - 1.10).abs() < 0.01);
        assert!((acute.sine(2).unwrap() - 1.0_f64.sin()).abs() < ROUNDING_ERROR);
        assert!((acute.cosine(2).unwrap() - 1.0_f64.cos()).abs() < ROUNDING_ERROR);
        assert!((acute.tangent(2).unwrap() - 1.0_f64.tan()).abs() < ROUNDING_ERROR);
        Ok(())
    }

}
