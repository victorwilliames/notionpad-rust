use floem::prelude::*;

fn main() {
    floem::launch(counter_view);
}

fn counter_view() -> impl IntoView {
    let counter = RwSignal::new(0);

    h_stack((
        button("Increment").action(move || counter.update(|c| *c += 1)),
        label(move || format!("Value: {}", counter.get())),
        button("Decrement").action(move || counter.update(|c| *c -= 1)),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(10))
}
