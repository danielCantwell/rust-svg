use crate::svg::{Point, SVG, Rect};


trait UserInteraction {
    type Arg;
    type Result;

    fn mouse_down(&mut self, point: Point, item: Option<&mut Self::Arg>) -> Option<Self::Result>;
    fn mouse_move(&mut self, point: Point, item: Option<&mut Self::Arg>) -> Option<Self::Result>;
    fn mouse_up(&mut self, point: Point, item: Option<&mut Self::Arg>) -> Option<Self::Result>;
}


enum DrawState {
    Waiting,
    Ready,
    Drawing,
}

// Waiting
// Click -> Ready
// Move -> Drawing
// Up -> Waiting

// Click... On first mouse_mouse RectTool should create a shape, in the Grid
// On subsequent mouse moves, the RectTool should resize the shape?
// On mouseup, Finish resizes
// 
// I think we also  want a separate "ResizeTool" though, but maybe that does that same thing, except on mouse_down
// it expects to be given a Shape, instead of preparing to create a shape like the RectTool
// 
// MoveTool will be similar - expect to be given a shape on mousedown


trait DrawTool: UserInteraction {
    type Shape;

    fn create_shape(&self) -> Self::Shape;
}


pub struct RectTool {
    origin: Option<Point>,
    state: DrawState,
}

impl RectTool {
    pub fn new() -> RectTool {
        RectTool{
            origin: None,
            state: DrawState::Waiting,
        }
    }
}

impl UserInteraction for RectTool {
    type Arg = Rect;
    type Result = Rect;

    fn mouse_down(&mut self, point: Point, _: Option<&mut Self::Arg>) -> Option<Self::Result> {
        self.origin = Some(point);

        if let DrawState::Waiting = self.state {
            self.state = DrawState::Ready;
        }

        None
    }

    fn mouse_move(&mut self, point: Point, _: Option<&mut Self::Arg>) -> Option<Self::Result> {
        match self.state {
            DrawState::Ready => {
                self.state = DrawState::Drawing;
                Some(self.create_shape())
            },
            DrawState::Drawing => {
                self.state = DrawState::Drawing;
                Some(self.create_shape())
            },
            _ => None
        }
    }

    fn mouse_up(&mut self, point: Point, _: Option<&mut Self::Arg>) -> Option<Self::Result> {
        None
    }
}

impl DrawTool for RectTool {
    type Shape = Rect;

    fn draw(&self) {

    }
}


fn calc<T>(a: RectTool) {
    println!("hey!")
}

fn stuff() {
    let x = RectTool::<Rect>{ shape: None };
    calc(x);
}



// struct Editor {
//     tool: Optional<Tool>,
// }

// impl Editor {
//     fn new() -> Editor {
//         Editor { tool: None }
//     }

//     fn activate_tool(&self, t: Tool) {
//         self.tool = t;
//     }

//     fn handle_interaction(&self, event: UserInteraction) {
//         match self.tool {

//         }
//     }
// }


// enum UserInteraction {
//     MouseDown(point: Point),
//     MouseMove(point: Point),
//     MouseUp(point: Point),
// }


// enum Tool {
//     Select,
//     Move,
//     Draw,
//     Resize,
// }