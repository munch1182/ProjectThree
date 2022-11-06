use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BaseBean<D: Copy> {
    pub code: u8,
    pub data: Option<D>,
}

impl<D: Copy> BaseBean<D> {
    // pub fn is_ok(&self) -> bool {
    //     self.code == 0
    // }

    pub fn success() -> Self {
        BaseBean {
            code: 0,
            data: None,
        }
    }

    pub fn error(code: u8) -> Self {
        BaseBean { code, data: None }
    }

    pub fn error1() -> Self {
        Self::error(1)
    }

    pub fn data(&mut self, data: D) -> Self {
        self.data = Some(data);
        *self
    }
}
