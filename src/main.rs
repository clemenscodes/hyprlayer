use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use gtk4::glib::timeout_add_local;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt, WidgetExt};
use gtk4::{Application, ApplicationWindow, Label};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

const APP_ID: &str = "claycodes.hyprlayer.overlay";

fn activate(app: &Application) {
  let window = ApplicationWindow::new(app);
  window.set_application(Some(app));
  window.init_layer_shell();
  window.set_layer(Layer::Overlay);
  window.set_exclusive_zone(-1);
  window.set_decorated(false);
  window.set_keyboard_mode(KeyboardMode::None);

  for (anchor, state) in [(Edge::Left, true), (Edge::Right, true), (Edge::Top, true)] {
    window.set_anchor(anchor, state);
  }

  let css = gtk4::CssProvider::new();
  css.load_from_data(
    r#"
    window {
      background-color: rgba(0,0,0,0);
    }

    .overlay-label {
      color: white;
      font-size: 24pt;
      font-weight: bold;
      padding: 10px;
    }
    "#,
  );
  gtk4::style_context_add_provider_for_display(&window.display(), &css, gtk4::STYLE_PROVIDER_PRIORITY_USER);

  let label = Label::new(Some(""));
  label.set_label("hyprlayer");
  label.set_css_classes(&["overlay-label"]);
  window.set_child(Some(&label));

  let window_ref = Rc::new(window);
  let visible_state = Rc::new(RefCell::new(false));

  {
    let window = window_ref.clone();
    let state = visible_state.clone();

    timeout_add_local(std::time::Duration::from_millis(500), move || {
      let condition = check_external_condition();

      if condition && !*state.borrow() {
        window.present();
        *state.borrow_mut() = true;
      } else if !condition && *state.borrow() {
        window.hide();
        *state.borrow_mut() = false;
      }

      gtk4::glib::ControlFlow::Continue
    });
  }
}

fn check_external_condition() -> bool {
  let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
  secs % 10 < 5
}

fn main() {
  let app = Application::new(Some(APP_ID), Default::default());
  app.connect_activate(activate);
  app.run();
}
