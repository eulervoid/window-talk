use ezing;
use nannou::prelude::*;

#[derive(Copy, Clone)]
pub enum Pointer {
    Idle(Point2),
    Moving { from: Point2, to: Point2, t: f32 },
}

impl Pointer {
    pub fn position(self) -> Point2 {
        match self {
            Pointer::Idle(pos) => pos,
            Pointer::Moving { from, to, t } => {
                let tc = ezing::quart_inout(t);
                from * (1.0 - tc) + to * tc
            }
        }
    }

    pub fn update(self, target_option: Option<Point2>) -> Self {
        if let Some(target) = target_option {
            match self {
                Pointer::Idle(pos) => Pointer::Moving {
                    from: pos,
                    to: target,
                    t: 0.0,
                },
                Pointer::Moving { from: _, to, t: _ } => {
                    if (target - to).magnitude() <= 1.0 {
                        if self.arrived() {
                            Pointer::Idle(to)
                        } else {
                            self.advance(0.025)
                        }
                    } else {
                        Pointer::Moving {
                            from: self.position(),
                            to: target,
                            t: 0.0,
                        }
                    }
                }
            }
        } else {
            Pointer::Idle(self.position())
        }
    }

    fn advance(self, delta: f32) -> Self {
        match self {
            Pointer::Idle(_) => self,
            Pointer::Moving { to, from, t } => Pointer::Moving {
                from,
                to,
                t: t + delta,
            },
        }
    }

    fn arrived(self) -> bool {
        match self {
            Pointer::Idle(_) => true,
            Pointer::Moving { to: _, from: _, t } => (1.0 - t) <= 0.01,
        }
    }
}
