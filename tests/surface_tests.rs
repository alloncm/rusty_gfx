use stupid_gfx::surface::Surface;

#[test]
fn new_from_raw_on_solid_data_returns_same_data(){
    let data = vec![0xFF;400];

    let surface = Surface::new_from_raw(data, 20, 20);

    for i in 0..400{
        assert_eq!(surface.pixels_data[i].dword, 0xFFu32);
    }
}

#[test]
fn new_from_raw_on_hetrogen_data_returns_same_data(){
    let mut data = vec![0;400];
    for i in 0..400{
        data[i] = i as u32;
    }

    let surface = Surface::new_from_raw(data, 20, 20);

    for i in 0..400{
        assert_eq!(surface.pixels_data[i].dword, i as u32);
    }
}