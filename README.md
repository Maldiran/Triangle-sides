# Triangle-sides
This library provides a `Triangle` struct that has various properties of a given triangle.

The struct's data consists of two types: `Option<f64>` and `[Option<f64>; 3]`.

The `Option<f64>` type is used for:

-   Perimeter
-   Area
-   Inradius
-   Circumradius

The `[Option<f64>; 3]` data are ordered in a specific way:  

-   Sides: The order of sides defines the order of arrays in the whole struct.  
-   Heights: The height's index reflects the side's index, so that `sides[i] * heights[i] / 2.0 = area`.  
-   Angles: The angle at index `i` is opposite to `sides[i]`. Their values are given in radians. To convert them to degrees, use standard library methods such as `f64.to_degrees()` and `f64.to_radians()`.  
-   Medians: This starts from the middle of the side of a given index.  
-   Sines, cosines, tangents: Their index corresponds to the angle.

A new struct is created using the method `new(sides: [Option<f64>; 3], angles: [Option<f64>; 3])`. It calculates all properties at creation time. Some fields of the above function can be left as `None`. If the triangle can be constructed from the given data, the function will return `Some(Triangle)`, if not you will get `None`. Angles specified in the above function are used only to calculate sides, they are discarded later, so there can be a situation when someone inputs angle 0.5000 and gets 0.4999 from the struct. Sides always take precedence over the angles, so if you provide incorrect angle, but the side data will be sufficient, it will not be taken into consideration and the function will return Some(Triangle).

Another way to create the struct is through the `new_from_sides(sides: [f64; 3])` method, which takes just the lengths of the sides. This is slightly faster than the previous method.

You can also create a blank struct with the function `new_blank(sides: [f64; 3])`. This will create a triangle struct with no pre-calculated properties, and all of them will be calculated on user demand.

**To access the struct's data, use:**  
`Triangle.side(i: usize)` → `Option<f64>`: The side at index `i`.  
`Triangle.sides()` → `[Option<f64>; 3]`: An array of sides.
`Triangle.perimeter()` → `Option<f64>`  
`Triangle.area()` → `Option<f64>`  
`Triangle.height(i: usize)` → `Option<f64>`  
`Triangle.heights()` → `[Option<f64>; 3]`  
`Triangle.median(i: usize)` → `Option<f64>`  
`Triangle.medians()` → `[Option<f64>; 3]`  
`Triangle.inradius()` → `Option<f64>`  
`Triangle.circumradius()` → `Option<f64>`  
`Triangle.angle(i: usize)` → `Option<f64>`  
`Triangle.angles()` → `[Option<f64>; 3]`  
`Triangle.sine(i: usize)` → `Option<f64>`  
`Triangle.sines()` → `[Option<f64>; 3]`  
`Triangle.cosine(i: usize)` → `Option<f64>`  
`Triangle.cosines()` → `[Option<f64>; 3]`  
`Triangle.tangent(i: usize)` → `Option<f64>`  
`Triangle.tangents()` → `[Option<f64>; 3]`  

Every one of them apart from `side` and `sides` can mutate the struct apart from only displaying a value:  
`Triangle.area_mut()` → `Option<f64>`: Mutates the area.  
This is useful only when creating with `new_blank` function, since `new` and `new_from_sides` generate all the data ahead of time so there is no reason to mutate them later. You can access data created with `new_blank` without mutating the struct, but the data will be generated with every access.
