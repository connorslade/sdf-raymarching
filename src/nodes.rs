use egui_snarl::{
    InPin, NodeId, OutPin, Snarl,
    ui::{PinInfo, SnarlViewer},
};
use tufa::export::egui::{Color32, Pos2, Ui};

pub struct NodeViewer;

pub enum Node {
    Primitive { ty: PrimitiveType },
    Mix,
    Output,
}

enum PrimitiveType {
    Sphere,
    Square,
}

const SDF_COLOR: Color32 = Color32::from_rgb(0x00, 0xb0, 0x00);
const SCALAR_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0x00);

impl SnarlViewer<Node> for NodeViewer {
    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Primitive { .. } => "Primitive",
            Node::Mix => "Mix",
            Node::Output => "Mix",
        }
        .to_string()
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Primitive { .. } => 0,
            Node::Mix => 3,
            Node::Output => 1,
        }
    }

    fn show_input(&mut self, _pin: &InPin, _ui: &mut Ui, _snarl: &mut Snarl<Node>) -> PinInfo {
        PinInfo::circle()
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Primitive { .. } => 1,
            Node::Mix => 1,
            Node::Output => 0,
        }
    }

    fn show_output(&mut self, pin: &OutPin, _ui: &mut Ui, snarl: &mut Snarl<Node>) -> PinInfo {
        match snarl[pin.id.node] {
            Node::Primitive { .. } => PinInfo::circle().with_fill(SDF_COLOR),
            Node::Mix => PinInfo::circle().with_fill(SDF_COLOR),
            Node::Output => unreachable!(),
        }
    }

    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: Pos2, ui: &mut Ui, snarl: &mut Snarl<Node>) {
        ui.menu_button("Primitive", |ui| {
            for ty in [PrimitiveType::Sphere, PrimitiveType::Square] {
                let name = match ty {
                    PrimitiveType::Sphere => "Sphere",
                    PrimitiveType::Square => "Square",
                };
                if ui.button(name).clicked() {
                    snarl.insert_node(pos, Node::Primitive { ty });
                    ui.close();
                }
            }
        });

        if ui.button("Mix").clicked() {
            snarl.insert_node(pos, Node::Mix);
            ui.close();
        }
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
}
