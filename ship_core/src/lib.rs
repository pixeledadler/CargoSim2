//! Represents the ships details

struct Particulars {
    shipname: String,
    loa: f32,
    beam: f32,
    height: f32,
    toughness_surface: f32, // Tonns per m2
    toughness_cover: f32,   // Tonns per m2
    minimal_draft: f32,
    maximal_draft: f32,
    max_dwt: u16,
    lightship: u16,
    lcg_offset: f32,
    vcg_offset: f32,
    tcg_offset: f32,
    compartment: Vec<Compartments>,
}

enum CompartmentType {
    Ballast,
    CargoHold,
    CargoTank,
    HFO,
    DO,
    LO,
    FW,
    Other,
}
struct Compartments {
    compartment_name: String,
    compartment_type: CompartmentType,
    x_coordinates: f32,
    y_coordinates: f32,
    z_coordinates: f32,
    leanght: f32,
    height: f32,
    breadth: f32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
