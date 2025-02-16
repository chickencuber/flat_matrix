pub struct FlatMatrix<T: Clone> {
    items: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> FlatMatrix<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        return Self {
            items: vec![fill; width*height],
            width,
            height,
        };
    }
    pub fn resize(&mut self, width: usize, height: usize, fill: T) {
       self.width = width;
       self.height = height;
       self.items.resize(width*height, fill);
    }
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        return y*self.width+x; 
    }
    pub fn get_pos(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index/self.width;
        return (x, y);
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
       let index = self.get_index(x, y); 
       return self.items.get(index);
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.get_index(x, y); 
        return self.items.get_mut(index);        
    }
    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        if let Some(v) = self.get_mut(x, y) {
            *v = value;
            return true; 
        } else {
            return false;
        }
    }
}

