use crate::{layers::Layer, utils::random_gradient_definition};
use crate::utils::random_color;
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct DiagonalSplitBackground;

impl Layer for DiagonalSplitBackground {
    fn generate(&self, random: &mut Random) -> Vec<Element> {
        // Generate the two triangles that will make up the diagonal split background
        let mut triangle1 = Polygon::new()
            .set("points", "0, 0, 1000, 1000, 0, 1000");

        let mut triangle2 = Polygon::new()
            .set("points", "1000, 1000, 1000, 0, 0, 0");

        // Possibly mirror the triangles
        if random.next_bool() {
            triangle1 = triangle1.set("transform", "scale(-1,1) translate(-1000)");
            triangle2 = triangle2.set("transform", "scale(-1,1) translate(-1000)");
        }        

        // Set the colors and return the result
        if random.next_bool() {
            // Pick two solid colors
            let random_color1 = random_color(random);
            let random_color2 = random_color(random);              

            // Add the fill to the triangles
            triangle1 = triangle1.set("fill", random_color1);
            triangle2 = triangle2.set("fill", random_color2);

            vec![triangle1.into(), triangle2.into()]
        } else {
            // Generate two gradient definitions
            let (random_gradient1, gradient_name1) = random_gradient_definition(random, Some(45));
            let (random_gradient2, gradient_name2) = random_gradient_definition(random, Some(45));

            // Add the fill to the triangles
            triangle1 = triangle1.set("fill", gradient_name1);
            triangle2 = triangle2.set("fill", gradient_name2);  

            vec![random_gradient1.into(), random_gradient2.into(), triangle1.into(), triangle2.into()]
        }
    }
}
