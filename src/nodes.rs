use egui_snarl::{
    InPin, NodeId, OutPin, Snarl,
    ui::{PinInfo, SnarlPin, SnarlViewer},
};
use tufa::export::egui::{Color32, DragValue, Frame, Pos2, RichText, Slider, Ui};

pub struct NodeViewer;

pub enum Node {
    Primitive { ty: PrimitiveType },
    Mix,
    Output,

    Constant(f32),
    Time,
}

#[derive(Clone, Copy)]
enum PrimitiveType {
    Sphere,
    Square,
}

impl PrimitiveType {
    fn name(&self) -> &str {
        match self {
            PrimitiveType::Sphere => "Sphere",
            PrimitiveType::Square => "Square",
        }
    }
}

mod colors {
    use tufa::export::egui::Color32;

    pub const SDF_HEADER: Color32 = Color32::from_rgb(29, 114, 94);
    pub const MATH_HEADER: Color32 = Color32::from_rgb(36, 98, 131);

    pub const SDF_CONNECTOR: Color32 = Color32::from_rgb(0, 214, 163);
    pub const SCALAR_CONNECTOR: Color32 = Color32::from_rgb(99, 99, 199);
}

impl SnarlViewer<Node> for NodeViewer {
    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Primitive { ty } => ty.name().to_owned(),
            Node::Mix => "Mix".to_owned(),
            Node::Output => "Output".to_owned(),
            Node::Constant(_) => "Constant".to_owned(),
            Node::Time => "Time".to_owned(),
        }
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Primitive { .. } | Node::Constant(_) | Node::Time => 0,
            Node::Mix => 3,
            Node::Output => 1,
        }
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        match &snarl[pin.id.node] {
            Node::Primitive { .. } | Node::Constant(_) | Node::Time => unreachable!(),
            Node::Mix => {
                if pin.id.input <= 1 {
                    ui.label(["a", "b"][pin.id.input]);
                    PinInfo::circle().with_fill(colors::SDF_CONNECTOR)
                } else {
                    ui.label("t");
                    PinInfo::square().with_fill(colors::SCALAR_CONNECTOR)
                }
            }
            Node::Output => PinInfo::circle().with_fill(colors::SDF_CONNECTOR),
        }
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Primitive { .. } | Node::Mix | Node::Constant(_) | Node::Time => 1,
            Node::Output => 0,
        }
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        _ui: &mut Ui,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        match snarl[pin.id.node] {
            Node::Primitive { .. } | Node::Mix => {
                PinInfo::circle().with_fill(colors::SDF_CONNECTOR)
            }
            Node::Constant(_) | Node::Time => PinInfo::square().with_fill(colors::SCALAR_CONNECTOR),
            Node::Output => unreachable!(),
        }
    }

    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: Pos2, ui: &mut Ui, snarl: &mut Snarl<Node>) {
        let mut button = |ui: &mut Ui, name: &str, node: &dyn Fn() -> Node| {
            if ui.button(name).clicked() {
                snarl.insert_node(pos, node());
                ui.close();
            }
        };

        button(ui, "Output", &|| Node::Output);
        ui.menu_button("Primitive", |ui| {
            for ty in [PrimitiveType::Sphere, PrimitiveType::Square] {
                button(ui, ty.name(), &|| Node::Primitive { ty })
            }
        });

        ui.separator();

        button(ui, "Mix", &|| Node::Mix);
        button(ui, "Constant", &|| Node::Constant(0.0));
        button(ui, "Time", &|| Node::Time);
    }

    fn has_node_menu(&mut self, _node: &Node) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<Node>,
    ) {
        if matches!(snarl[node], Node::Output) {
            return;
        }

        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close();
        }
    }

    fn has_body(&mut self, node: &Node) -> bool {
        matches!(node, Node::Constant(_))
    }

    fn show_body(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<Node>,
    ) {
        match &mut snarl[node] {
            Node::Constant(val) => {
                ui.add(DragValue::new(val));
            }
            _ => unreachable!(),
        }
    }

    fn header_frame(
        &mut self,
        frame: Frame,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        snarl: &Snarl<Node>,
    ) -> Frame {
        match &snarl[node] {
            Node::Primitive { .. } | Node::Output => frame.fill(colors::SDF_HEADER),
            Node::Mix | Node::Constant(_) | Node::Time => frame.fill(colors::MATH_HEADER),
        }
    }

    fn show_header(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<Node>,
    ) {
        ui.label(RichText::new(self.title(&snarl[node])).color(Color32::WHITE));
    }
}
