#[cfg(test)]
mod tests {
    use triangle_sides::Triangle;

    #[test]
    fn test_egyptian() -> Result<(), String> {
        let egyptian = Triangle::new_from_sides(3.0, 4.0, 5.0); //creates egyptian triangle
        assert!((egyptian.angle(2).expect("Error 1").to_degrees() - 90.0).abs() < 0.0000000001);
        Ok(())
    }
}
