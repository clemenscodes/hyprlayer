use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn activate(application: &gtk4::Application) {
  let window = gtk4::ApplicationWindow::new(application);

  window.init_layer_shell();

  window.set_layer(Layer::Overlay);

  window.auto_exclusive_zone_enable();

  let anchors = [
    (Edge::Left, true),
    (Edge::Right, true),
    (Edge::Top, false),
    (Edge::Bottom, true),
  ];

  for (anchor, state) in anchors {
    window.set_anchor(anchor, state);
  }

  let label = gtk4::Label::new(Some(""));
  label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
  window.set_child(Some(&label));
  window.show()
}

fn main() {
  let application = gtk4::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

  application.connect_activate(|app| {
    activate(app);
  });

  application.run();
}
