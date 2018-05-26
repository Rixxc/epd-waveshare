


#[derive(Clone, Copy)]
pub enum Displayorientation {
    /// No rotation
    Rotate0,
    /// Rotate by 90 degrees clockwise
    Rotate90,
    /// Rotate by 180 degrees clockwise
    Rotate180,
    /// Rotate 270 degrees clockwise
    Rotate270,
}

//WARNING: Adapt for bigger sized displays!
pub struct DisplayDescription {
    width: u16,
    height: u16,
    buffer_size: u16
}

// impl Display_Description {
//     pub fn new(width: u16, height: u16, buffer_size: u16) -> Display_Description {

//     }
// }

pub enum Display {
    Eink42BlackWhite,
}

impl Display {
    /// Gets the Dimensions of a dipslay in the following order:
    /// - Width
    /// - Height
    /// - Neccessary Buffersize
    pub fn get_dimensions(&self) -> (u16, u16, u16) {
        match self {
            Display::Eink42BlackWhite => (400, 300, 15000)
        }
    }
}

pub struct Graphics {
    width: u16,
    height: u16,
    rotation: Displayorientation,
    //buffer: Box<u8>//[u8; 15000],   
}

pub enum Color {
    Black,
    White
}

impl Color {
    fn get_bit_value(&self) -> u8 {
        match self {
            Color::White => 1u8,
            Color::Black => 0u8,            
        }
    }

    fn get_full_byte(&self) -> u8 {
        match self {
            Color::White => 0xff,
            Color::Black => 0x00,
        }
    } 
}





impl Graphics {
    /// width needs to be a multiple of 8!
    pub fn new(width: u16, height: u16) -> Graphics{
        Graphics {width, height, rotation: Displayorientation::Rotate0}
    }

    /// Clears/Fills the full buffer with `color`
    pub fn clear(&self, buffer: &mut[u8], color: &Color) {
        for elem in buffer.iter_mut() {
            *elem = color.get_full_byte();
        }
    }

    /// Draw a single Pixel with `color`
    /// 
    /// limited to i16::max images (buffer_size) at the moment
    pub fn draw_pixel(&self, buffer: &mut[u8], x: u16, y: u16, color: &Color) {
        let (idx, bit) = match self.rotation {
            Displayorientation::Rotate0 | Displayorientation::Rotate180 
                => ((x as usize / 8 + (self.width as usize / 8) * y as usize) ,
                    0x80 >> (x % 8)),
            Displayorientation::Rotate90 | Displayorientation::Rotate270
                => (y as usize / 8 * self.width as usize + x as usize,
                    0x80 >> (y % 8)),
        };

        if idx >= buffer.len() {
            return;
        }

        match color {
            Color::Black => {
                buffer[idx] &= !bit; 
            },
            Color::White => {
                buffer[idx] |= bit;
            }
        }
    }

    ///TODO: implement!
    pub fn draw_char(&self, buffer: &mut[u8]) {
        unimplemented!(); 
    }

    ///TODO: implement!
    pub fn draw_string(&self, buffer: &mut[u8]) {
        unimplemented!(); 
    }

//     void plotLine(int x0, int y0, int x1, int y1)
// {
//    int dx =  abs(x1-x0), sx = x0<x1 ? 1 : -1;
//    int dy = -abs(y1-y0), sy = y0<y1 ? 1 : -1; 
//    int err = dx+dy, e2; /* error value e_xy */
 
//    for(;;){  /* loop */
//       setPixel(x0,y0);
//       if (x0==x1 && y0==y1) break;
//       e2 = 2*err;
//       if (e2 >= dy) { err += dy; x0 += sx; } /* e_xy+e_x > 0 */
//       if (e2 <= dx) { err += dx; y0 += sy; } /* e_xy+e_y < 0 */
//    }
// }
    //bresenham algorithm for lines
    /// draw line 
    pub fn draw_line(&self, buffer: &mut[u8], x0: u16, y0: u16, x1: u16, y1: u16, color: &Color) {
        let mut x0 = x0 as i16;
        let x1 = x1 as i16;
        let mut y0 = y0 as i16;
        let y1 = y1 as i16;

        let dx = i16::abs(x1 - x0);
        let sx = if x0 < x1 { 1 } else { -1 };

        let dy = - i16::abs(y1 - y0);
        let sy = if y0 < y1 { 1 } else { -1 };
        
        let mut err = dx + dy;

        loop {
            self.draw_pixel(buffer, x0 as u16, y0 as u16, color);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2*err;

            if e2 >= dy {
                err += dy;
                x0 += sx;
            }

            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Draw a horizontal line 
    /// TODO: maybe optimize by grouping up the bytes? But is it worth the longer and more complicated function? is it even faster?
    pub fn draw_horizontal_line(&self, buffer: &mut[u8], x: u16, y: u16, length: u16, color: &Color) {
        for i in 0..length {
            self.draw_pixel(buffer, x + i, y, color);
        }
    }

    /// Draws a vertical line
    pub fn draw_vertical_line(&self, buffer: &mut[u8], x: u16, y: u16, length: u16, color: &Color) {
        for i in 0..length {
            self.draw_pixel(buffer, x, y + i, color);
        }
    }

    /// Draws a rectangle. (x0,y0) is top left corner, (x1,y1) bottom right
    pub fn draw_rectangle(&self, buffer: &mut[u8], x0: u16, y0: u16, x1: u16, y1: u16, color: &Color) {
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        let x_len = max_x - min_x;
        let y_len = max_y - min_y;
        self.draw_horizontal_line(buffer, min_x, min_y, x_len, color);
        self.draw_horizontal_line(buffer, min_x, max_y, x_len, color);
        self.draw_vertical_line(buffer, min_x, min_y, y_len, color);
        self.draw_vertical_line(buffer, max_x, min_y, y_len, color);
    }

    /// Draws a filled rectangle. For more info see draw_rectangle
    pub fn draw_filled_rectangle(&self, buffer: &mut[u8], x0: u16, y0: u16, x1: u16, y1: u16, color: &Color) {
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        let x_len = max_x - min_x;
        let y_len = max_y - min_y;
        for i in 0..y_len {
            self.draw_horizontal_line(buffer, min_x, min_y + i, x_len, color);
        }
    }

    fn draw_pixel_helper(&self, buffer: &mut[u8], x: i16, y: i16, color: &Color) {
        if x >= 0 && y >= 0 {
            self.draw_pixel(buffer, x as u16, y as u16, color);
        }
    }

    //TODO: test if circle looks good
    /// Draws a circle
    pub fn draw_circle(&self, buffer: &mut[u8], x: u16, y: u16, radius: u16, color: &Color) {
        let radius = radius as i16;
        let x_mid = x as i16;
        let y_mid = y as i16;
        let mut x_pos: i16 = 0 - radius; 
        let mut y_pos = 0;
        let mut err: i16 = 2 - 2 * radius;

        loop {
            self.draw_pixel_helper(buffer, x_mid - x_pos, y_mid + y_pos, color);
            self.draw_pixel_helper(buffer, x_mid - y_pos, y_mid - x_pos, color);
            self.draw_pixel_helper(buffer, x_mid + x_pos, y_mid - y_pos, color);
            self.draw_pixel_helper(buffer, x_mid + y_pos, y_mid + x_pos, color);

            let radius = err;

            if radius <= y_pos {
                y_pos += 1;
                err += y_pos*2 + 1;
            }

            if radius > x_pos || err > y_pos {
                x_pos += 1;
                err += x_pos*2 + 1;
            }

            if x_pos >= 0 {
                break;
            }
        }
    }
//         }
//         unimplemented!(); 


//         void plotCircle(int xm, int ym, int r)
// {
//    int x = -r, y = 0, err = 2-2*r; /* II. Quadrant */ 
//    do {
//       setPixel(xm-x, ym+y); /*   I. Quadrant */
//       setPixel(xm-y, ym-x); /*  II. Quadrant */
//       setPixel(xm+x, ym-y); /* III. Quadrant */
//       setPixel(xm+y, ym+x); /*  IV. Quadrant */
//       r = err;
//       if (r <= y) err += ++y*2+1;           /* e_xy+e_y < 0 */
//       if (r > x || err > y) err += ++x*2+1; /* e_xy+e_x > 0 or no 2nd y-step */
//    } while (x < 0);
// }
//     }

    ///TODO: implement!
    pub fn draw_filled_circle(&self, buffer: &mut[u8]) {
        unimplemented!(); 
    }

    
}



#[cfg(test)]
mod graphics {
    use super::*;

    #[test]
    fn test_filled_rectangle() {
        let mut buffer = [Color::White.get_full_byte(); 150];
        let graphics = Graphics::new(40, 30);
        graphics.draw_filled_rectangle(&mut buffer, 0, 0, 40, 30, &Color::Black);
        
        assert_eq!(buffer[0], Color::Black.get_full_byte());

        for &elem in buffer.iter() {
            
            assert_eq!(elem, Color::Black.get_full_byte());
        }

        
    }

    /// draw a 4x4 in the top left corner
    #[test]
    fn test_filled_rectangle2() {
        let mut buffer = [Color::White.get_full_byte(); 8];
        let graphics = Graphics::new(8, 8);
        graphics.draw_filled_rectangle(&mut buffer, 0, 0, 4, 4, &Color::Black);
        
        assert_eq!(buffer[0], 0x0f);

        let mut counter = 0;
        for &elem in buffer.iter() {
            counter += 1;           

            if counter <= 4 {
                assert_eq!(elem, 0x0f);
            } else {
                assert_eq!(elem, Color::White.get_full_byte());
            }
        }

        
    }

    #[test]
    fn test_horizontal_line() {
        let mut buffer = [Color::White.get_full_byte(); 4];
        let graphics = Graphics::new(16, 2);
        graphics.draw_horizontal_line(&mut buffer, 1, 0, 14, &Color::Black);
        
        assert_eq!(buffer[0], 0x80);
        assert_eq!(buffer[1], 0x01);
        assert_eq!(buffer[2], Color::White.get_full_byte());
        assert_eq!(buffer[3], Color::White.get_full_byte());
    }

    #[test]
    fn test_vertical_line() {
        let mut buffer = [Color::White.get_full_byte(); 8];
        let graphics = Graphics::new(8, 8);
        graphics.draw_vertical_line(&mut buffer, 0, 0, 8, &Color::Black);

        graphics.draw_vertical_line(&mut buffer, 5, 0, 8, &Color::Black);
        
        
        assert_eq!(buffer[0], 0x7b);

        for &elem in buffer.iter() {
            
            assert_eq!(elem, 0x7bu8);
        }
    }

    //test draw_line for compatibility with draw_vertical_line
    #[test]
    fn draw_line_1() {
        let mut buffer = [Color::White.get_full_byte(); 8];
        let graphics = Graphics::new(8, 8);

        graphics.draw_vertical_line(&mut buffer, 5, 0, 8, &Color::Black);

        let mut buffer2 = [Color::White.get_full_byte(); 8];
        let graphics2 = Graphics::new(8, 8);

        graphics2.draw_line(&mut buffer2, 5, 0, 5, 8, &Color::Black);       

        for i in 0..buffer.len() {            
            assert_eq!(buffer[i], buffer2[i]);
        }
    }

    //test draw_line for compatibility with draw_horizontal_line
    #[test]
    fn draw_line_2() {
        let mut buffer = [Color::White.get_full_byte(); 4];
        let graphics = Graphics::new(16, 2);
        graphics.draw_horizontal_line(&mut buffer, 1, 0, 14, &Color::Black);

        let mut buffer2 = [Color::White.get_full_byte(); 4];
        let graphics2 = Graphics::new(16, 2);
        graphics2.draw_line(&mut buffer2, 1, 0, 14, 0, &Color::Black);       

        for i in 0..buffer.len() {            
            assert_eq!(buffer[i], buffer2[i]);
        }
    }

    //test draw_line for diago
    #[test]
    fn draw_line_3() {
        let mut buffer = [Color::White.get_full_byte(); 8];
        let graphics = Graphics::new(8, 8);

        graphics.draw_line(&mut buffer, 0, 0, 16, 16, &Color::Black);       

        for i in 0..buffer.len() {            
            assert_eq!(buffer[i], !(0x80 >> i % 8));
        }
    }



    #[test]
    fn test_pixel() {
        let mut buffer = [Color::White.get_full_byte(); 8];
        let graphics = Graphics::new(8, 8);
        graphics.draw_pixel(&mut buffer, 1, 0, &Color::Black);

        assert_eq!(buffer[0], !0x40);


        let mut buffer = [Color::White.get_full_byte(); 16];
        let graphics = Graphics::new(16, 8);
        graphics.draw_pixel(&mut buffer, 9, 0, &Color::Black);
        assert_eq!(buffer[0], Color::White.get_full_byte());
        assert_eq!(buffer[1], !0x40);
        
        for &elem in buffer.iter() {
            
            //assert_eq!(elem, 0x00u8);
        }
    }

    // #[test]
    // #[should_panic]
    // fn test_any_panic() {
    //     divide_non_zero_result(1, 0);
    // }

    // #[test]
    // #[should_panic(expected = "Divide result is zero")]
    // fn test_specific_panic() {
    //     divide_non_zero_result(1, 10);
    // }
}