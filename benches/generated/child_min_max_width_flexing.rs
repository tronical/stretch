pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: stretch::style::Dimension::Points(0f32),
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(60f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: stretch::style::Dimension::Percent(0.5f32),
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(120f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
