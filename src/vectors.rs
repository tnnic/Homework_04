pub const VEC3_LEN: usize = 3;

#[derive(Debug, PartialEq)]
pub struct Vec3([i32; VEC3_LEN]);
impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Vec3 {
    // Метод для создания вектора по умолчанию
    pub fn new() -> Self {
        Vec3([0; VEC3_LEN])
    }

    // Метод для поэлементного сложения двух векторов
    pub fn vector_sum(&self, other: &Vec3) -> Vec3 {
        let mut c = Vec3::new();
        for i in 0..VEC3_LEN {
            c.0[i] = self.0[i] + other.0[i];
        }
        c
    }

    // Метод для вычисления скалярной суммы элементов двух векторов
    pub fn scalar_sum(&self, other: &Vec3) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += self.0[i] + other.0[i];
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_vector_sum() {
        let vec1 = Vec3([1, 2, 3]);
        let vec2 = Vec3([4, 5, 6]);
        assert_eq!(vec1.vector_sum(&vec2), Vec3([5, 7, 9]));
    }

    #[test]
    fn test_default_vec3() {
        assert_eq!(Vec3::new(), Vec3([0; VEC3_LEN]));
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let vec1 = Vec3([1, 2, 3]);
        let vec2 = Vec3([4, 5, 6]);
        assert_eq!(vec1.scalar_sum(&vec2), 21);
    }
}
