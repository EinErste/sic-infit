use amethyst::core::transform::Transform;
use amethyst::core::math::Vector3;
///Trait which implements a parallax effect
pub trait AdjustToDistance {
    fn adjust_to_distance(&mut self ,distance: f32, initial_width: f32, initial_height: f32) -> (f32,f32);
}
///Function used for creating a parallax effect for the background
impl AdjustToDistance for Transform {
    fn adjust_to_distance(&mut self, distance: f32,initial_width: f32, initial_height: f32,) -> (f32,f32){

        //Magic angles(seriously)
        let rad_x = 44_f32.to_radians();
        let rad_y = 70_f32.to_radians();
        //new
        let rad_x = 20_f32.to_radians();
        let rad_y = 60_f32.to_radians();

        let scale_x = (2.*distance*(rad_x.cos()/rad_x.sin())+initial_width)/initial_width;
        let scale_y = (2.*distance*(rad_y.cos()/rad_y.sin())+initial_height)/initial_height;
        self.set_scale(Vector3::new(scale_x,scale_y,1.));
        (scale_x*initial_width,scale_y*initial_height)

        // let distance = distance.abs() + 1920.;
        //
        // let rad_x = 20_f32.to_radians();
        // let rad_y = 15_f32.to_radians();
        //
        //
        // let scale_x = (initial_width + rad_x.sin() * distance * 2.)/initial_width;
        // let scale_y = (initial_height + rad_y.sin() * distance * 2.)/initial_height;
        // self.set_scale(Vector3::new(scale_x,scale_y,1.));
        //
        // dbg!((scale_x*initial_width,scale_y*initial_height));
        // (scale_x*initial_width,scale_y*initial_height)
    }
}
