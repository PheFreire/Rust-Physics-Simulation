pub struct Poligon {
    max_x: f32,
    min_x: f32,
    max_y: f32,
    min_y: f32,

    poligon_vertex: Vec<[f32;2]>
}

impl Poligon {
    // Todo
    // Implement Raycasting
    // Implement colliding coordinates correcting to poligon closest edge

    fn new(poligon_vertex: Vec<[f32;2]>) -> Poligon {
        Poligon {
            max_x: 0.0,
            min_x: 0.0,
            max_y: 0.0,
            min_y: 0.0,
            poligon_vertex: poligon_vertex,
        }
        // Implement creating auto detect max and mix poligons points using poligons_vertex Vec 
    }

    fn raycasting() {
        // Raycasting
        // To know if some object is colliding and where is colliding with in the poligon we use Raycasting
        // the object gonna enter the space between the max and min poligon structure space 
        // draw an horizontal or vertical line from one side to he other of the poligon passing through the object
        // and count the intersects between the line and the poligons edges
        // if the line intersects returns an odd number the object is coliding with the polligon

    }
}




